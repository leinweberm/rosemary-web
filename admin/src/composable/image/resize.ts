import type { Ref } from 'vue';
import {TUploadImagePaintingQuery} from "../../sdk/api.ts";

export const processInputImageFiles = async (
	imageFiles: Ref<File[]>
): Promise<{previews: string[], data: TUploadImagePaintingQuery[]}> => {
	const previews: string[] = [];
	const data: TUploadImagePaintingQuery[] = [];

	if (!Array.isArray(imageFiles.value)) {
		return {previews, data};
	}

	for (let i = 0, length = imageFiles.value.length; i < length; i++) {

		if (imageFiles.value[i].type !== 'image/jpeg') {
			window.alert('Nahrávejte pouze JPEG obrázky!');
			return {previews, data};
		}

		const previewUrl = URL.createObjectURL(imageFiles.value[i]);
		const image = new Image();
		image.src = previewUrl;

		await new Promise((resolve) => {
			image.onload = () => {
				previews.push(previewUrl);
				data.push({
					preview: 'false',
					title_cs: '',
					title_en: '',
					alt_cs: '',
					alt_en: '',
					painting_id: '',
				});
				resolve(true);
			}
		});
	}

	return {previews, data};
};