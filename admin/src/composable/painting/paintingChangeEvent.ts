import type {TPaintingDetail, TPaintingStub} from "../../sdk/api.ts";
import type {Ref} from 'vue';
import {parseInt} from "lodash";

export type TEventPaintingInformationKey = keyof TPaintingStub | 'lang';
export type TEventPaintingInformation = {
		key: TEventPaintingInformationKey;
		value: string;
		lang?: 'cs' | 'en';
};

export const handleExistingPaintingFormEvent = (
	painting: Ref<TPaintingDetail>,
	event: TEventPaintingInformation
) => {
	if (['painting_description', 'painting_title'].includes(event.key) && !event.lang) {
		console.warn('Invalid lang for type translation', event.lang);
		return;
	}

	if (['price', 'width', 'height'].includes(event.key) && isNaN(parseInt(event.value as string))) {
		console.warn('Invalid value for type number', event.value);
		return;
	}

	console.log('event', event);

	switch (event.key) {
		case 'painting_description':
			// @ts-expect-error handled by if statement above
			painting.value.painting.painting_description[event.lang] = event.value as string || '';
			break;
		case 'painting_title':
			// @ts-expect-error handled by if statement above
			painting.value.painting.painting_title[event.lang] = event.value as string || '';
			break;
		case 'price':
			painting.value.painting.price = parseInt(event.value);
			break;
		case 'width':
			painting.value.painting.width = parseInt(event.value);
			break;
		case 'height':
			painting.value.painting.height = parseInt(event.value);
			break;
		default:
			break;
	}
};