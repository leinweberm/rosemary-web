import {
	ApiSDK as SDK,
	type TUpdatePainting,
	type TPaintingDetail, TUpdatePaintingImage, TPaintingImage, TUploadImagePaintingQuery, TCreatePainting
} from "../../sdk/api.ts";
import {isEmpty} from "lodash";

export class PaintingSave extends EventTarget {
	private readonly apiSDK: SDK;
	private readonly token: string;

	constructor(token: string) {
		super();
		this.apiSDK = new SDK();
		this.token = token;
	}

	async createPainting(data: TCreatePainting): Promise<any> {
		const result = await this.apiSDK.createPainting(data, this.token);
		if (result) {
			this.dispatchEvent(new Event('saveProgress'));
			return result;
		} else {
			throw new Error('Failed to create painting');
		}
	}

	async updatePainting(newData: TPaintingDetail, originalData: TPaintingDetail): Promise<void> {
		const updateData: TUpdatePainting = {};

		if (newData.painting.painting_title.cs !== originalData.painting.painting_title.cs) {
			updateData.title_cs = newData.painting.painting_title.cs;
		}
		if (newData.painting.painting_title.en !== originalData.painting.painting_title.en) {
			updateData.title_en = newData.painting.painting_title.cs;
		}
		if (newData.painting.painting_description.cs !== originalData.painting.painting_description.cs) {
			updateData.description_en = newData.painting.painting_description.cs;
		}
		if (newData.painting.painting_description.en !== originalData.painting.painting_description.en) {
			updateData.description_en = newData.painting.painting_description.cs;
		}
		if (newData.painting.price !== originalData.painting.price) {
			updateData.price = newData.painting.price;
		}
		if (newData.painting.height !== originalData.painting.height) {
			updateData.height = newData.painting.height;
		}
		if (newData.painting.width !== originalData.painting.width) {
			updateData.width = newData.painting.width;
		}

		if (isEmpty(updateData)) {
			console.log('no valid fiels for update');
			this.dispatchEvent(new Event('saveProgress'));
			return;
		}

		const updated = await this.apiSDK.updatePainting(newData.painting.id, updateData, this.token);
		if (updated) {
			this.dispatchEvent(new Event('saveProgress'));
		} else {
			throw new Error('Failed to update painting metadata');
		}
	}

	async removeImage(id: string): Promise<void> {
		const removed = await this.apiSDK.removePaintingImage(id, this.token);
		if (removed) {
			this.dispatchEvent(new Event('saveProgress'));
		} else {
			throw new Error('Failed to remove painting metadata');
		}
	}

	async updateImage(oldImage: TPaintingImage, updatedImage: TPaintingImage): Promise<void> {
		const updateData: TUpdatePaintingImage = {};

		if (updatedImage.title.cs !== oldImage.title.cs) {
			updateData.title_cs = updatedImage.title.cs;
		}
		if (updatedImage.title.en !== oldImage.title.en) {
			updateData.title_en = updatedImage.title.en;
		}
		if (updatedImage.alt.cs !== oldImage.alt.cs) {
			updateData.alt_cs = updatedImage.alt.cs;
		}
		if (updatedImage.alt.en === oldImage.alt.en) {
			updateData.alt_en = updatedImage.alt.en;
		}
		if (updatedImage.preview !== oldImage.preview) {
			updateData.preview = updatedImage.preview;
		}

		if (isEmpty(updateData)) {
			this.dispatchEvent(new Event('saveProgress'));
			return;
		}

		const updated = await this.apiSDK.updatePaintingImage(
			updateData,
			updatedImage.id,
			this.token
		);
		if (updated) {
			this.dispatchEvent(new Event('saveProgress'));
		} else {
			throw new Error('Failed to update painting metadata');
		}
	}

	async uploadImage(file: File, meta: TUploadImagePaintingQuery, id: string): Promise<void> {
		const uploaded = await this.apiSDK.uploadPaintingImage(
			file,
			{
				preview: meta.preview,
				title_cs: meta.title_cs,
				title_en: meta.title_en,
				alt_cs: meta.alt_cs,
				alt_en: meta.alt_en,
				painting_id: id,
			},
			this.token
		);
		if (uploaded) {
			this.dispatchEvent(new Event('saveProgress'));
		} else {
			throw new Error('Failed to update painting metadata');
		}
	}
}