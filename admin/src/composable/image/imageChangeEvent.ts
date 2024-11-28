import type {TPaintingImage, TUploadImagePaintingQuery} from "../../sdk/api.ts";

export type TEventPaintingImageRowKey = keyof TUploadImagePaintingQuery;
export type TEventPaintingImageRow = {
	key: TEventPaintingImageRowKey;
	value: string;
}

export const handleExistingImageFormEvent = (
	image: TPaintingImage,
	event: TEventPaintingImageRow,
): TPaintingImage => {
	switch (event.key) {
		case 'title_cs':
			image.title.cs = event.value;
			break;
		case 'title_en':
			image.title.en = event.value;
			break;
		case 'alt_cs':
			image.alt.cs = event.value;
			break;
		case 'alt_en':
			image.alt.en = event.value;
			break;
		default:
			break;
	}
	return image;
};

export const handleNewImageFormEvent = (
	image: TUploadImagePaintingQuery,
	event: TEventPaintingImageRow,
): TUploadImagePaintingQuery => {
	switch (event.key) {
		case 'title_cs':
			image.title_cs = event.value;
			break;
		case 'title_en':
			image.title_en = event.value;
			break;
		case 'alt_cs':
			image.alt_cs = event.value;
			break;
		case 'alt_en':
			image.alt_en = event.value;
			break;
		default:
			break;
	}
	return image;
}