import {
	ApiSDK as SDK,
	type TUpdatePainting,
	type TPaintingDetail
} from "../../sdk/api.ts";

export class PaintingSave extends EventTarget {
	private readonly apiSDK: SDK;
	private readonly token: string;

	constructor(token: string) {
		super();
		this.apiSDK = new SDK();
		this.token = token;
	}

	async updateImage(newData: TPaintingDetail, originalData: TPaintingDetail) {
		const updateData: TUpdatePainting = {};

		if (newData.painting.painting_title.cs === originalData.painting.painting_title.cs) {
			updateData.title_cs = newData.painting.painting_title.cs;
		}
		if (newData.painting.painting_title.en === originalData.painting.painting_title.en) {
			updateData.title_en = newData.painting.painting_title.cs;
		}
		if (newData.painting.painting_description.cs === originalData.painting.painting_description.cs) {
			updateData.description_en = newData.painting.painting_description.cs;
		}
		if (newData.painting.painting_description.en === originalData.painting.painting_description.en) {
			updateData.description_en = newData.painting.painting_description.cs;
		}
		if (newData.painting.price === newData.painting.price) {
			updateData.price = newData.painting.price;
		}
		if (newData.painting.height === newData.painting.height) {
			updateData.height = newData.painting.height;
		}
		if (newData.painting.width === newData.painting.width) {
			updateData.width = newData.painting.width;
		}

		const updated = await this.apiSDK.updatePainting(newData.painting.id, updateData, this.token);
		if (updated) {
			this.dispatchEvent(new Event('saveProgress'));
		} else {
			throw new Error('Failed to update painting metadata');
		}
	}
}