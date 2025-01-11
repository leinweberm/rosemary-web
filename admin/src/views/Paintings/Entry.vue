<script setup lang="ts">
import {useRoute, useRouter} from "vue-router";
import {inject, ref} from "vue";
import {type ApiSDK as SDK, TUploadImagePaintingQuery} from "../../sdk/api.ts";
import {routesOpts} from "../../router/router.ts";
import {useUserStore} from "../../stores/userStore.ts";
import PaintingInformation from "../../components/PaintingInformation.vue";
import PaintingTranslations from "../../components/PaintingTranslations.vue";
import PaintingImageRow from "../../components/PaintingImageRow.vue";
import {TEventPaintingInformation} from "../../composable/painting/paintingChangeEvent.ts";
import {processInputImageFiles} from "../../composable/image/resize.ts";
import {handleNewImageFormEvent, TEventPaintingImageRow} from "../../composable/image/imageChangeEvent.ts";
import {PaintingSave} from "../../composable/painting/paintingSave.ts";
import SavingDialog from "../../components/SavingDialog.vue";

const router = useRouter();
const userStore = useUserStore();
const route = useRoute();
const paintingForm = ref(null);
const title_cs = ref<string>("");
const title_en = ref<string>("");
const description_cs = ref<string>("");
const description_en = ref<string>("");
const price = ref<number>(0);
const width = ref<number>(1);
const height = ref<number>(1);
const newImages = ref<File[]>([]);
const newImagesPreviews = ref<string[]>([]);
const newImagesMetadata = ref<TUploadImagePaintingQuery[]>([]);
const previewUrl = ref<string>('');
const isSaving = ref<number>(0);
const paintingId = ref<string>('');
const ApiSDK: SDK | undefined = inject<SDK>('ApiSDK');
const saveSteps = ref<number>(0);
const saveProgress = ref<number>(0);

const handleFileInput = async () => {
	const {previews, data} = await processInputImageFiles(newImages);
	newImagesPreviews.value = [...previews];
	newImagesMetadata.value = [...data];
};

const handleNewFilesChange = (event: TEventPaintingImageRow, index: number): void => {
	if (newImagesMetadata.value[index]) {
		newImagesMetadata.value[index] = handleNewImageFormEvent(newImagesMetadata.value[index], event);
	}
};

const handlePreviewChange = (index: number): void => {
	const findPreviewIndex = newImagesMetadata.value.findIndex((el) => el.preview === 'true');
	if (findPreviewIndex > -1) {
		newImagesMetadata.value[findPreviewIndex].preview = 'false';
	}
	newImagesMetadata.value[index].preview = 'true';
	previewUrl.value = newImagesPreviews.value[index];
}

const handlePaintingInfoChange = (event: TEventPaintingInformation): void => {
	if (event.key === 'height') {
		height.value = parseInt(event.value as string);
	} else if (event.key === 'width') {
		width.value = parseInt(event.value as string);
	} else if (event.key === 'painting_title' && event.lang && event.lang === 'cs') {
		title_cs.value = event.value as string;
	} else if (event.key === 'painting_title' && event.lang && event.lang === 'en') {
		title_en.value = event.value as string;
	} else if (event.key === 'painting_description' && event.lang && event.lang === 'cs') {
		description_cs.value = event.value as string;
	} else if (event.key === 'painting_description' && event.lang && event.lang === 'en') {
		description_en.value = event.value as string;
	} else if (event.key === 'price') {
		price.value = parseInt(event.value as string);
	}
};

const handleRemoveImage = (index: number): void => {
	newImages.value.splice(index, 1);
	newImagesPreviews.value.splice(index, 1);
	newImagesMetadata.value.splice(index, 1);
};

const validateForm = async (): Promise<boolean> => {
	// @ts-expect-error
	const valid = await paintingForm.value.validate();
	if (!valid) {
		window.alert('Nevalidní formulář');
		return false;
	}
	if (!newImages.value.length) {
		window.alert('Obraz musí mít alespoň jednu fotku');
		return false;
	}
	if (!previewUrl) {
		window.alert('Obraz musí mít náhledovou fotku');
		return false;
	}
	if (!ApiSDK) {
		window.alert('Něco se pokazilo');
		return false;
	}
	return true;
};

const save = async (): Promise<void> => {
	const valid = await validateForm();
	if (!valid) return;

	isSaving.value = 1;
	await userStore.authRouteAccess();
	const token = userStore.getUser?.token;
	if (!token) return;

	saveSteps.value = 1 + newImages.value.length;
	const stepValue = Math.ceil(100 / saveSteps.value);
	const saveHandler = new PaintingSave(token);
	saveHandler.addEventListener('saveProgress', () => {
		saveProgress.value = ((saveProgress.value + stepValue) > 100)
			? 100
			: (saveProgress.value += stepValue);
	});

	try {
		const created = await saveHandler.createPainting({
			title_cs: title_cs.value,
			title_en: title_en.value,
			description_cs: description_cs.value,
			description_en: description_en.value,
			price: price.value,
			width: width.value,
			height: height.value
		});

		paintingId.value = created.data.id;
		for (let i = 0, length = newImages.value.length; i < length; i++) {
			await saveHandler.uploadImage(
				newImages.value[i],
				newImagesMetadata.value[i],
				created.data.id as string
			);
		}
	} catch (error) {
		saveHandler.removeEventListener('saveProgress', null);
		return;
	}

	saveHandler.removeEventListener('saveProgress', null);
	isSaving.value = 2;
};

const openDetail = (): void => {
	router.push({
		name: routesOpts.P_DETAIL,
		params: {id: paintingId.value},
	});
};
</script>

<template>
	<!--	ACTION BUTTONS -->
	<div class="actionButtons">
		<v-btn
			type="button"
			variant="elevated"
			color="primary"
			style="margin-left: 8px"
			@click.stop="save"
		>Uložit</v-btn>
	</div>

	<v-form ref="paintingForm">
		<div class="entryGrid">
			<!-- TRANSLATIONS -->
			<PaintingTranslations
				:edit="true"
				:title-cs="title_cs"
				:title-en="title_en"
				:description-cs="description_cs"
				:description-en="description_en"
				@model-update="handlePaintingInfoChange($event)"
			/>

			<!-- INFORMATION	-->
			<PaintingInformation
				:price="price"
				:height="height"
				:width="width"
				:edit="true"
				:image-src="previewUrl"
				@model-update="handlePaintingInfoChange($event)"
			/>
		</div>

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
							:disabled="!!newImages.length"
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
								:edit="true"
								@modelUpdate="handleNewFilesChange($event, mIndex)"
								@preview-select="handlePreviewChange(mIndex)"
								@remove-painting-image="handleRemoveImage(mIndex)"
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