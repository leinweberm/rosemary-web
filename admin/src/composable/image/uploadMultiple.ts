import {Ref} from "vue";
import {TUploadImagePaintingQuery} from "../../sdk/api.ts";
import {ApiSDK} from "../../sdk/api.ts";

export const uploadPaintingImages = async (
	token: string,
	images: Ref<File[]>,
	metadata: Ref<TUploadImagePaintingQuery[]>,
	painting_id: string,
): Promise<void> => {
	if (images.value.length !== metadata.value.length) {
		throw new Error('Metadata length does not match images length');
	}

	const SDK = new ApiSDK();

	try {
		for (let i = 0, length = images.value.length; i < length; i++) {
			await SDK.uploadPaintingImage(
				images.value[i],
				{...metadata.value[i], painting_id},
				token
			);
		}
	} catch (error) {
		throw error;
	}
}