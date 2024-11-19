<script setup lang="ts">
import {useRoute, useRouter} from "vue-router";
import {inject, onMounted, ref} from "vue";
import {type ApiSDK as SDK, TUploadImagePaintingQuery} from "../../sdk/api.ts";
import {routesOpts} from "../../router/router.ts";
import {useUserStore} from "../../stores/userStore.ts";

type TPreviewData = {
	url: string;
	width: string;
	height: string;
};

const router = useRouter();
const userStore = useUserStore();
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
const paintingImagesData = ref<TUploadImagePaintingQuery[]>([]);
const paintingImagesPreviews = ref<TPreviewData[]>([])
const isSaving = ref<number>(0);
const savingProgress = ref<number>(0);
const savingDialogText = ref<string>('PROBÍHÁ UKLÁDÁNÍ');
const paintingId = ref<string>('');
const ApiSDK: SDK | undefined = inject<SDK>('ApiSDK');

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
			paintingImagesPreviews.value = [];
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
			paintingImagesPreviews.value.push({
				url: previewUrl,
				width: `${newWidth}px`,
				height: `${newHeight}px`
			});
			paintingImagesData.value.push({
				preview: 'false',
				title_cs: '',
				title_en: '',
				alt_cs: '',
				alt_en: '',
				painting_id: ''
			});
		};
	}
};

const validateForm = async () => {
	// @ts-expect-error
	const valid = await paintingForm.value.validate();
	if (!valid) {
		window.alert('Nevalidní formulář');
		return false;
	}
	if (!paintingImages.value.length) {
		window.alert('Obraz musí mít alespoň jednu fotku');
		return false;
	}
	if (!previewImage) {
		window.alert('Obraz musí mít náhledovou fotku');
		return false;
	}
	if (!ApiSDK) {
		window.alert('Něco se pokazilo');
		return false;
	}
	return true;
}

const setProgress = (value: number): void => {
	savingProgress.value = ((savingProgress.value + value) > 100)
		? 100
		: (savingProgress.value + value);
}

const save = async () => {
	const validForm = await validateForm();
	if (!validForm) return;
	isSaving.value = 1;

	const step = Math.ceil(100 / (1 + paintingImages.value.length));
	try {
		await userStore.authRouteAccess();
		let token = userStore.getUser?.token;
		if (!token) throw new Error('Nepřihlášený užvatel');
		const { data } = await ApiSDK?.createPainting({
			// @ts-ignore
			price: parseInt(price.value),
			width: width.value,
			height: height.value,
			title_cs: title_cs.value,
			title_en: title_en.value,
			description_cs: description_cs.value,
			description_en: description_en.value,
		}, token);
		paintingId.value = data.id;
	} catch (error) {
		console.error(error);
		window.alert('Nepovedlo se vytvořit obraz');
		return;
	}
	setProgress(step);

	for (let i = 0, len = paintingImages.value.length; i < len; i++) {
		try {
			await userStore.authRouteAccess();
			let token = userStore.getUser?.token;
			if (!token) throw new Error('Nepřihlášený užvatel');
			await ApiSDK?.uploadPaintingImage(
				paintingImages.value[i],
				{
					preview: paintingImagesData.value[i].preview,
					title_cs: paintingImagesData.value[i].title_cs,
					title_en: paintingImagesData.value[i].title_en,
					alt_cs: paintingImagesData.value[i].alt_cs,
					alt_en: paintingImagesData.value[i].alt_en,
					painting_id: paintingId.value,
				},
				token
			);
		} catch (error) {
			console.error(error);
			window.alert('Nepovedlo se nahrát fotku obrazu');
		}
		setProgress(step);
	}

	savingDialogText.value = 'Uloženo';
	isSaving.value = 2;
}

const openDetail = () => {
	router.push({
		name: routesOpts.P_DETAIL,
		params: {id: paintingId.value},
	});
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

const setPreviewImage = (index: number): void => {
	if (previewImage.value) {
		const findPreviewSource = paintingImagesPreviews.value.findIndex((el) => el.url === previewImage.value?.url);
		if (findPreviewSource) {
			paintingImagesData.value[findPreviewSource].preview = 'false';
		}
	}
	previewImage.value = paintingImagesPreviews.value[index];
	paintingImagesData.value[index].preview = 'true';
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
			@click.stop="save"
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
							:disabled="!!paintingImages.length"
							@update:modelValue="handleFileInput"
						></v-file-input>
					</v-col>
				</v-row>
					<v-divider></v-divider>
					<v-row>
						<v-col>
							<div class="imagePreviewWrapper">
								<template
									v-for="(_, index) in paintingImages"
									:key="index"
								>
									<div class="previewImageRow">
										<div style="height: 240px; width: 240px; align-items: center; justify-content: center; display: flex">
											<div
												:class="{
													imagePreviewWrapperSelected: (previewImage && previewImage.url === paintingImagesPreviews[index].url),
													imagePreviewItem: (!previewImage || previewImage.url !== paintingImagesPreviews[index].url),
												}"
												:style="{
													cursor: 'pointer',
													height: paintingImagesPreviews[index].height,
													width: paintingImagesPreviews[index].width,
												}"
												@click.stop="setPreviewImage(index)"
											>
												<img
													:src="paintingImagesPreviews[index].url"
													alt="upload image preview"
													:height="paintingImagesPreviews[index].height"
													:width="paintingImagesPreviews[index].width"
												>
											</div>
										</div>
										<div class="imageMetaData">
											<v-row>
												<v-col>
													<v-text-field
														v-model="paintingImagesData[index].title_cs"
														type="text"
														label="Název CZ"
													></v-text-field>
												</v-col>
											</v-row>
											<v-row>
												<v-col>
													<v-text-field
														v-model="paintingImagesData[index].alt_cs"
														type="text"
														label="Alt CZ"
													></v-text-field>
												</v-col>
											</v-row>
										</div>
										<div class="imageMetaData">
											<v-row>
												<v-col>
													<v-text-field
														v-model="paintingImagesData[index].title_en"
														type="text"
														label="Název EN"
													></v-text-field>
												</v-col>
											</v-row>
											<v-row>
												<v-col>
													<v-text-field
														v-model="paintingImagesData[index].alt_en"
														type="text"
														label="Alt EN"
													></v-text-field>
												</v-col>
											</v-row>
											<v-row
												style="position: absolute; bottom: 0; right: 20px"
											>
												<v-col>
													<v-btn
														variant="tonal"
														color="error"
														size="small"
													>Odstranit</v-btn>
												</v-col>
											</v-row>
										</div>
									</div>
									<v-divider
										v-if="index !== (paintingImages.length - 1)"
										style="margin: 10px 0"
									></v-divider>
								</template>
							</div>
						</v-col>
					</v-row>
			</v-container>
		</v-card>
	</v-form>
	<dialog v-if="isSaving" id="savingDialog" class="elevation-7">
		<div style="width: 100%">
			<h2 style="text-align: center">{{ savingDialogText }}</h2>
		</div>
		<div style="width: 100%; display: flex; align-items: center; margin-top: 20px">
			<v-progress-linear
				color="amber"
				height="50"
				v-model="savingProgress"
			>
				<strong>{{ Math.ceil(savingProgress) }}%</strong>
			</v-progress-linear>
		</div>
		<v-btn
			v-if="isSaving === 2"
			variant="elevated"
			size="large"
			color="success"
			style="margin-top: 20px"
			@click.stop="openDetail"
		>
			OK
		</v-btn>
	</dialog>
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
.previewImageRow {
	display: flex;
	width: 100%;
	height: 240px;
}
.imageMetaData {
	display: flex;
	flex-direction: column;
	width: calc((100% - 250px) / 2);
	height: 240px;
	padding: 10px 20px;
	position: relative;
}

dialog {
	position: absolute;
	flex-direction: column;
	align-items: center;
	justify-content: center;
	display: flex;
	z-index: 10;
	top: 50dvh;
	left: 50%;
	transform: translate(-50%, -50%);
	width: 500px;
	height: 250px;
	border: none;
	padding: 20px;
}
</style>