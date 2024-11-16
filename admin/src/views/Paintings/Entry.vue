<script setup lang="ts">
import {useRoute} from "vue-router";
import {onMounted, ref} from "vue";

type TPreviewData = {
	url: string;
	width: string;
	height: string;
};

const isNew = ref<boolean>(false);
const route = useRoute();
const paintingForm = ref(null);
const tab = ref<number>(0);

const title_cs = ref<string>("");
const title_en = ref<string>("");
const description_cs = ref<string>("");
const description_en = ref<string>("");
const price = ref<number>(0);
const width = ref<number>(1);
const height = ref<number>(1);
const previewImage = ref<TPreviewData | null>(null);
const paintingImages = ref<File[]>([]);
const uploadPreviews = ref<TPreviewData[]>([]);

const resizeImage = (width: number, height: number, maxSize: number): [width: number, height: number] => {
	const ratio = width > height ? maxSize / width : maxSize / height;
	const resizedWidth = Math.round(width * ratio);
	const resizedHeight = Math.round(height * ratio);
	return [resizedWidth, resizedHeight];
};

const handleFileInput = () => {
	if (!Array.isArray(paintingImages.value)) return;

	for (let i = 0, len = paintingImages.value.length; i < len; i++) {
		if (paintingImages.value[i].type !== 'image/jpeg') {
			window.alert('Nahrávejte pouze JPEG obrázky!');
			paintingImages.value = [];
			uploadPreviews.value = [];
			break;
		}

		const previewUrl = URL.createObjectURL(paintingImages.value[i]);
		const image = new Image();
		image.src = previewUrl;

		image.onload = () => {
			const [newWidth, newHeight] = resizeImage(
				image.naturalWidth,
				image.naturalHeight,
				240
			);
			uploadPreviews.value.push({
				url: previewUrl,
				width: `${newWidth}px`,
				height: `${newHeight}px`
			});
		};
	}
};

const resizeImagePreviewSkeleton = () => {
	const [newWidth, newHeight] = resizeImage(width.value, height.value, 240);
	const el = document.querySelector('#imageSkeletonBody') as HTMLElement;
	if (!el) return;
	el.style.width = `${newWidth}px`;
	el.style.height = `${newHeight}px`;
};

const handleWidthChange = (event: any) => {
	width.value = parseInt(event.target.value, 10);
	resizeImagePreviewSkeleton();
};

const handleHeightChange = (event: any) => {
	height.value = parseInt(event.target.value, 10);
	resizeImagePreviewSkeleton()
};

const setPreviewImage = (preview: TPreviewData): void => {
	previewImage.value = preview;
};

onMounted(() => {
	if (route.fullPath.includes('/+')) {
		isNew.value = true;
	}
	resizeImagePreviewSkeleton();
})
</script>

<template>
<!--	ACTION BUTTONS -->
	<div class="actionButtons">
		<v-btn
			v-if="!isNew"
			type="button"
			variant="elevated"
			color="error"
			style="margin-left: 8px"
		>Odstranit</v-btn>
		<v-btn
			v-if="!isNew"
			variant="elevated"
			type="button"
			color="info"
			style="margin-left: 8px"
		>Zrušit</v-btn>
		<v-btn
			type="button"
			variant="elevated"
			color="primary"
			style="margin-left: 8px"
		>Uložit</v-btn>
	</div>
<!-- TRANSLATIONS -->
	<v-form ref="paintingForm">
		<div class="entryGrid">
			<v-card>
				<v-card-title style="padding: 10px 20px;">Překlady</v-card-title>
				<v-tabs
					v-model="tab"
					align-tabs="center"
					color="primary"
				>
					<v-tab :value="1">CZ</v-tab>
					<v-tab :value="2">EN</v-tab>
				</v-tabs>
				<v-tabs-window v-model="tab">
					<v-tabs-window-item
						v-for="n in 2"
						:key="n"
						:value="n"
					>
						<v-container fluid>
							<v-row>
								<v-col>
									<v-text-field
										v-if="n == 1"
										type="text"
										label="název"
										v-model="title_cs"
									/>
									<v-text-field
										v-else-if="n == 2"
										type="text"
										label="název"
										v-model="title_en"
									/>
								</v-col>
							</v-row>
							<v-row>
								<v-col>
									<v-textarea
										v-if="n == 1"
										v-model="description_cs"
										label="popis"
										rows="13"
									></v-textarea>
									<v-textarea
										v-else-if="n == 2"
										v-model="description_en"
										label="popis"
										rows="13"
									></v-textarea>
								</v-col>
							</v-row>
						</v-container>
					</v-tabs-window-item>
				</v-tabs-window>
			</v-card>
<!-- INFORMATION -->
			<v-card>
				<v-card-title style="padding: 10px 25px">Informace</v-card-title>
				<v-container>
					<v-row>
						<v-col>
							<div class="imageWrapper">
								<div
									id="imageSkeletonBody"
									class="imageSkeleton"
									style="position: absolute; z-index: 4; border: 2px solid gold"
									:style="{
										background: (previewImage) ? 'transparent' : 'rgb(240, 240, 240)'
									}"
								></div>
								<img
									v-if="previewImage"
									:src="previewImage.url"
									alt="painting preview image"
									:width="previewImage.width"
									:height="previewImage.height"
									style="position: absolute; z-index: 2"
								/>
							</div>
						</v-col>
					</v-row>
					<v-row>
						<v-col>
							<v-text-field
								v-model="price"
								label="cena"
								type="number"
								min="0"
								max="100000"
							></v-text-field>
							<v-text-field
								:model-value="height"
								label="výška (cm)"
								type="number"
								min="0"
								max="300"
								@change="handleHeightChange"
							></v-text-field>
							<v-text-field
								:model-value="width"
								label="šířka (cm)"
								type="number"
								min="0"
								max="300"
								@change="handleWidthChange"
							></v-text-field>
						</v-col>
					</v-row>
				</v-container>
			</v-card>
		</div>
<!--	FILES	-->
		<v-card>
			<v-card-title>Soubory</v-card-title>
			<v-container>
				<v-row>
					<v-col>
						<v-file-input
							v-model="paintingImages"
							label="soubory"
							variant="filled"
							show-size
							multiple
							counter
							chips
							@update:modelValue="handleFileInput"
						></v-file-input>
					</v-col>
				</v-row>
<!--				<template v-if="uploadPreviews.length">-->
					<v-divider></v-divider>
					<v-row>
						<v-col>
							<div class="imagePreviewWrapper">
								<div
									v-for="(preview, index) in uploadPreviews"
									:key="index"
									:class="{
										imagePreviewWrapperSelected: (previewImage && previewImage.url === preview.url),
										imagePreviewItem: (!previewImage || previewImage.url !== preview.url),
									}"
									:style="{
										height: preview.height,
										width: preview.width,
										margin: '10px',
										cursor: 'pointer'
									}"
									@click.stop="setPreviewImage(preview)"
								>
									<img
										:src="preview.url"
										:width="preview.width"
										:height="preview.height"
										alt="upload image"
									/>
								</div>
							</div>
						</v-col>
					</v-row>
<!--				</template>-->
			</v-container>
		</v-card>
	</v-form>
</template>

<style scoped>
* {
	box-sizing: border-box;
}
.actionButtons {
	display: flex;
	width: 100%;
	justify-content: flex-end;
	align-items: flex-end;
	margin-bottom: 20px;
}
.entryGrid {
	display: grid;
	box-sizing: border-box;
	margin-bottom: 20px;
	padding: 0;
	grid-template-columns: calc(100% - 300px) 300px;
	gap: 20px;
	width: calc(100% - 25px);
}
.imageWrapper {
	display: flex;
	height: 250px;
	width: 250px;
	justify-content: center;
	align-items: center;
	margin: auto;
	position: relative;
}
.imageSkeleton {
	display: flex;
}
.imagePreviewWrapper {
	display: flex;
	width: 100%;
	min-height: 240px;
	flex-wrap: wrap;
	align-items: center;
	justify-content: flex-start;
	margin-top: 20px;
}
.imagePreviewItem, .imagePreviewWrapperSelected {
	transition-property: box-shadow;
	transition-duration: 0.28s;
}
.imagePreviewItem:hover {
	box-shadow:
		0px 3px 2px -2px rgba(0, 0, 0, 0.2),
		0px 2px 4px 0px rgba(0, 0, 0, 0.14),
		0px 1px 10px 0px rgba(0, 0, 0, 0.12);
}
.imagePreviewWrapperSelected {
	box-shadow:
		0px 4px 5px -2px rgba(0, 0, 0, 0.5),
		0px 7px 10px 1px rgba(0, 0, 0, 0.5),
		0px 2px 16px 1px rgba(0, 0, 0, 0.5);
}
</style>