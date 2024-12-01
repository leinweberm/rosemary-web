<script setup lang="ts">
import {useRoute, useRouter} from 'vue-router';
import {inject, onMounted, ref} from "vue";
import {ApiSDK as SDK, TPaintingDetail, TUploadImagePaintingQuery} from "../../sdk/api.ts";
import {routesOpts} from "../../router/router.ts";
import {processInputImageFiles} from "../../composable/image/resize.ts";
import PaintingTranslations  from "../../components/PaintingTranslations.vue";
import PaintingInformation from "../../components/PaintingInformation.vue";
import PaintingImageRow from "../../components/PaintingImageRow.vue";
import {useUserStore} from "../../stores/userStore.ts";
import {isEqual, cloneDeep} from "lodash";
import {handleExistingImageFormEvent, handleNewImageFormEvent, type TEventPaintingImageRow} from "../../composable/image/imageChangeEvent.ts";
import {handleExistingPaintingFormEvent, type TEventPaintingInformation} from "../../composable/painting/paintingChangeEvent.ts";
// @ts-ignore IDK
import {PaintingSave} from "../../composable/painting/paintingSave.ts";
import SavingDialog from "../../components/SavingDialog.vue";

const userStore = useUserStore();
const route = useRoute();
const router = useRouter();
const ApiSDK: SDK | undefined = inject<SDK>('ApiSDK');
let painting = ref<TPaintingDetail>();
let originalPainting = ref<TPaintingDetail>();
const previewUrl = ref<string>('');
const edit = ref<boolean>(false);
const dirty = ref<boolean>(false);
const paintingForm = ref(null);
const newImages = ref<File[]>([]);
const newImagesPreviews = ref<string[]>([]);
const newImagesMetadata = ref<TUploadImagePaintingQuery[]>([]);
const loaded = ref<boolean>(false);
const saveSteps = ref<number>(0);
const saveProgress = ref<number>(0);
const isSaving = ref<number>(0);

const fetchPainting = async (id: string) => {
	const data = await ApiSDK?.getPaintingDetail(id);
	if (data) {
		painting.value = cloneDeep(data);
		originalPainting.value = cloneDeep(data);
		previewUrl.value = `${ApiSDK?.staticUrl}/${painting.value.painting.preview.urls[0]}`;
	}
};

const handleFileInput = async () => {
	const {previews, data} = await processInputImageFiles(newImages);
	newImagesPreviews.value = [...previews];
	newImagesMetadata.value = [...data];
}

const handlePaintingInfoChange = (event: TEventPaintingInformation): void => {
	if (painting.value?.painting) {
		// @ts-expect-error
		handleExistingPaintingFormEvent(painting, event);
		dirty.value = true;
	}
};

const handleExistingFilesChange = (event: TEventPaintingImageRow, index: number): void => {
	if (painting.value?.images[index]) {
		painting.value.images[index] = handleExistingImageFormEvent(painting.value.images[index], event);
		dirty.value = true;
	}
};

const handleNewFilesChange = (event: TEventPaintingImageRow, index: number): void => {
	if (newImagesMetadata.value[index]) {
		newImagesMetadata.value[index] = handleNewImageFormEvent(newImagesMetadata.value[index], event);
		dirty.value = true;
	}
};

const handleRemovePaintingImage = (index: number, isNew: boolean): void => {
	edit.value = true;
	if (isNew) {
		newImages.value.splice(index, 1);
		newImagesPreviews.value.splice(index, 1);
		newImagesMetadata.value.splice(index, 1);
	} else if (painting.value) {
		painting.value.images.splice(index, 1);
	}
};

const handlePreviewChange = (index: number, isNew: boolean): void => {
	edit.value = true;
	const findNewIndex = newImagesMetadata.value.findIndex((el) => el.preview === 'true');
	if (findNewIndex > -1) {
		newImagesMetadata.value[findNewIndex].preview = 'false';
		return;
	}

	if (painting.value) {
		const findExistingIndex =  painting.value.images.findIndex((el) => el.preview);
		if (findExistingIndex > -1 && painting.value.images[findExistingIndex]) {
			painting.value.images[findExistingIndex].preview = false;
			return;
		}
	}

	if (isNew) {
		newImagesMetadata.value[index].preview = 'true';
		previewUrl.value = newImagesPreviews.value[index];
	} else if (painting.value?.images[index]) {
		painting.value.images[index].preview = true;
		previewUrl.value = `${ApiSDK?.staticUrl}/${painting.value?.images[index].urls[0]}`;
	}

	dirty.value = true;
};

const cancelEdit = async () => {
	edit.value = false;
	newImagesPreviews.value = [];
	newImagesMetadata.value = [];
	newImages.value = [];
	painting.value = cloneDeep(originalPainting.value);
};

const save = async () => {
	// @ts-expect-error
	const valid = await paintingForm.value.validate();
	if (!valid) return;

	await userStore.authRouteAccess();
	const token = userStore.getUser?.token;
	if (!token || !painting.value || !originalPainting.value) {
		return;
	}
	isSaving.value = 1;

	const existingImagesChanged: Array<{oldIndex: number, newIndex: number}> = [];
	const existingImagesRemoved: string[] = [];
	let tempSaveSteps = 0;

	if (!isEqual(painting.value, originalPainting.value)) {
		tempSaveSteps += 1;
		for (let i = 0, length = originalPainting.value.images.length; i < length; i++) {
			const originalImage = originalPainting.value.images[i];
			const updatedImageIndex = painting.value.images.findIndex((el) => el.id === originalImage.id);
			if (updatedImageIndex > -1) {
				if (!isEqual(painting.value.images[updatedImageIndex], originalImage)) {
					existingImagesChanged.push({oldIndex: i, newIndex: updatedImageIndex});
				}
			} else {
				existingImagesRemoved.push(originalImage.id);
			}
		}
		tempSaveSteps += (existingImagesChanged.length + existingImagesRemoved.length);
	}

	if (newImages.value.length) {
		tempSaveSteps += newImages.value.length;
	}

	saveSteps.value = tempSaveSteps;
	const stepValue = (saveSteps.value) ? Math.ceil(100 / saveSteps.value) : 100;

	const saveHandler = new PaintingSave(token);
	saveHandler.addEventListener('saveProgress', () => {
		if ((saveProgress.value + stepValue) > 100) {
			saveProgress.value += stepValue;
		} else {
			saveProgress.value = 100;
		}
	});

	try {
		if (!isEqual(painting.value, originalPainting.value)) {
			await saveHandler.updatePainting(painting.value, originalPainting.value);
			for (let i = 0, length = existingImagesChanged.length; i < length; i++) {
				await saveHandler.updateImage(
					originalPainting.value.images[existingImagesChanged[i].oldIndex],
					painting.value.images[existingImagesChanged[i].newIndex]
				);
			}
			for (let i = 0, length = existingImagesRemoved.length; i < length; i++) {
				await saveHandler.removeImage(existingImagesRemoved[i]);
			}
		}
		for (let i = 0, length = newImages.value.length; i < length; i++) {
			await saveHandler.uploadImage(
				newImages.value[i],
				newImagesMetadata.value[i],
				painting.value.painting.id
			);
		}
	} catch (error) {
		console.error(error);
		saveHandler.removeEventListener('saveProgress', null);
		isSaving.value = 0;
		return;
	}

	saveProgress.value = 100;
	saveHandler.removeEventListener('saveProgress', null);
	isSaving.value = 2;
	edit.value = false;
};

const removePainting = async () => {
	await userStore.authRouteAccess();
	const token = userStore.getUser?.token;
	if (!token || !painting.value?.painting.id) {
		return;
	}
	await ApiSDK?.removePainting(painting.value.painting.id, token);
	await router.push({name: routesOpts.P_LIST});
}

const openDetail = () => {
	location.reload();
};

onMounted(async () => {
	if (!route.params.id) {
		await router.push({name: routesOpts.P_LIST});
	}
	await fetchPainting(route.params.id as string);
	loaded.value = true;
});
</script>

<template>
	<div class="actionButtons">
		<v-btn
			type="button"
			variant="elevated"
			color="error"
			style="margin-left: 8px"
			@click.stop="removePainting"
		>Odstranit</v-btn>
		<v-btn
			v-if="!edit"
			variant="elevated"
			type="button"
			color="info"
			style="margin-left: 8px"
			@click.stop="edit = true;"
		>Upravit</v-btn>
		<v-btn
			v-if="edit"
			variant="elevated"
			type="button"
			color="warning"
			style="margin-left: 8px"
			@click.stop="cancelEdit"
		>Zrušit</v-btn>
		<v-btn
			v-if="edit"
			type="button"
			variant="elevated"
			color="primary"
			style="margin-left: 8px"
			:disabled="!dirty"
			@click.stop="save"
		>Uložit</v-btn>
	</div>
	<v-form ref="paintingForm" v-if="loaded">
		<div class="entryGrid">

			<!-- TRANSLATIONS	-->
			<PaintingTranslations
				v-if="painting"
				:edit="edit"
				:title-cs="painting.painting.painting_title.cs"
				:title-en="painting.painting.painting_title.en"
				:description-cs="painting.painting.painting_description.cs"
				:description-en="painting.painting.painting_description.en"
				@model-update="handlePaintingInfoChange($event)"
			/>

			<!-- INFORMATION -->
			<PaintingInformation
				v-if="painting"
				:image-src="previewUrl"
				:price="painting.painting.price"
				:height="painting.painting.height"
				:width="painting.painting.width"
				:edit="edit"
				@model-update="handlePaintingInfoChange($event)"
			/>
		</div>

		<!-- EXISTING FILES -->
		<v-card style="margin-bottom: 20px">
			<v-card-title>Stávající soubory</v-card-title>
			<v-container v-if="painting">
				<v-row>
					<v-col>
						<div class="imagePreviewWrapper">
							<PaintingImageRow
								v-for="(image, pIndex) in painting.images"
								:key="`existing_image_${pIndex}`"
								:painting-id="route.params.id as string"
								:image-id="image.id"
								:image-url="`${ApiSDK?.staticUrl}/${image.urls[0]}`"
								:current-preview-url="previewUrl"
								:title-cs="image.title.cs"
								:title-en="image.title.en"
								:alt-cs="image.alt.cs"
								:alt-en="image.alt.en"
								:edit="edit"
								@modelUpdate="handleExistingFilesChange($event, pIndex)"
								@preview-select="handlePreviewChange(pIndex, false)"
								@remove-painting-image="handleRemovePaintingImage(pIndex, false)"
							/>
						</div>
					</v-col>
				</v-row>
			</v-container>
		</v-card>

		<!-- NEW FILES -->
		<v-card>
			<v-card-title>Nové soubory</v-card-title>
			<v-container>
				<v-row>
					<v-col>
						<v-file-input
							v-model="newImages"
							label="soubory"
							variant="filled"
							show-size
							multiple
							counter
							chips
							:disabled="!edit || !!newImages.length"
							@update:modelValue="handleFileInput"
						></v-file-input>
					</v-col>
				</v-row>
				<v-divider/>
				<v-row>
					<v-col>
						<div class="imagePreviewWrapper">
							<PaintingImageRow
								v-for="(meta, mIndex) in newImagesMetadata"
								:key="`new_image_${mIndex}`"
								:painting-id="route.params.id as string"
								:image-url="newImagesPreviews[mIndex]"
								:current-preview-url="previewUrl"
								:title-cs="meta.title_cs"
								:title-en="meta.title_en"
								:alt-cs="meta.alt_cs"
								:alt-en="meta.alt_en"
								:edit="edit"
								@modelUpdate="handleNewFilesChange($event, mIndex)"
								@preview-select="handlePreviewChange(mIndex, true)"
								@remove-painting-image="handleRemovePaintingImage(mIndex, true)"
							/>
						</div>
					</v-col>
				</v-row>
			</v-container>
		</v-card>
	</v-form>

	<SavingDialog
		:is-saving="isSaving"
		:save-progress="saveProgress"
		:cb="openDetail"
	/>
</template>