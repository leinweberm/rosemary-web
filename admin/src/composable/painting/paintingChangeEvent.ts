import type {TPaintingDetail, TPaintingStub} from "../../sdk/api.ts";
import type {Ref} from 'vue';

export type TEventPaintingInformationKey = keyof TPaintingStub;
export type TEventPaintingInformation = {
		key: TEventPaintingInformationKey;
		value: string | number;
		lang: 'cs' | 'en';
};

export const handleExistingPaintingFormEvent = (
	painting: Ref<TPaintingDetail>,
	event: TEventPaintingInformation
) => {
	switch (event.key) {
		case 'painting_description':
			painting.value.painting.painting_description[event.lang] = event.value as string || '';
			break;
		case 'painting_title':
			painting.value.painting.painting_title[event.lang] = event.value as string || '';
			break;
		case 'price':
			painting.value.painting.price = event.value as number;
			break;
		case 'width':
			painting.value.painting.width = event.value as number;
			break;
		case 'height':
			painting.value.painting.height = event.value as number;
			break;
		default:
			break;
	}
};