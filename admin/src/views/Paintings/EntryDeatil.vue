<script setup lang="ts">
import {useRoute, useRouter} from 'vue-router';
import {inject, onMounted, ref} from "vue";
import {ApiSDK as SDK, TPaintingDetail, TUploadImagePaintingQuery} from "../../sdk/api.ts";
// import {useUserStore} from "../../stores/userStore.ts";
import {routesOpts} from "../../router/router.ts";
import {processInputImageFiles} from "../../composable/image/resize.ts";
// import {uploadPaintingImages} from "../../composable/image/uploadMultiple.ts";
import PaintingImageRow, {TEventPaintingImageRow} from "../../components/PaintingImageRow.vue";
import PaintingTranslations, {TEventPaintingTranslations} from "../../components/PaintingTranslations.vue";
import PaintingInformation, {TEventPaintingInformation} from "../../components/PaintingInformation.vue";

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

const fetchPainting = async (id: string) => {
	const data = await ApiSDK?.getPaintingDetail(id);
	if (data) {
		painting.value = {...data};
		originalPainting.value = {...data};
		previewUrl.value = painting.value.painting.preview.urls[0];
	}
};

const handleFileInput = async () => {
	const {previews, data} = await processInputImageFiles(newImages);
	newImagesPreviews.value = [...previews];
	newImagesMetadata.value = [...data];
}

const handlePaintingTranslationsChange = (event: TEventPaintingTranslations): void => {
	painting.value.painting[event.key][event.lang] = event.value;
	dirty.value = true;
};

const handlePaintingInfoChange = (event: TEventPaintingInformation): void => {
	painting.value.painting[event.key] = event.value;
	dirty.value = true;
};

const handleExistingFilesChange = (event: TEventPaintingImageRow, index: number): void => {
	painting.value?.images[index][event.key] = event.value;
	dirty.value = true;
};

const handleNewFilesChange = (event: TEventPaintingImageRow, index: number): void => {
	newImagesMetadata.value[index][event.key] = event.value;
	dirty.value = true;
};

const handlePreviewChange = (index: number, isNew: boolean): void => {
	if (isNew) {
		newImagesMetadata.value[index].preview = 'true';
		previewUrl.value = newImagesPreviews[index];
	} else {
		painting.value.images[index].preview = true;
		previewUrl.value = `${ApiSDK?.staticUrl}/${painting.value?.images[index].urls[0]}`;
	}

	dirty.value = true;

	const findNewIndex = newImagesMetadata.value.findIndex((el) => el.preview === 'true');
	if (findNewIndex > -1) {
		newImagesMetadata.value[findNewIndex] = 'false';
		return;
	}

	const findExistingIndex =  painting.value.images.findIndex((el) => el.preview);
	if (findExistingIndex > -1) {
		painting.value.images[findExistingIndex].preview = false;
		return;
	}
};

const cancelEdit = async () => {
	edit.value = false;
	newImagesPreviews.value = [];
	newImagesMetadata.value = [];
	newImages.value = [];
	painting.value = originalPainting.value;
};

const save = async () => {
	edit.value = false;
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
				@model-update="handlePaintingTranslationsChange($event)"
			/>

			<!-- INFORMATIONS -->
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
								@preview-select="handlePreviewChange(pIndex, true)"
							/>
						</div>
					</v-col>
				</v-row>
			</v-container>
		</v-card>
	</v-form>
</template>