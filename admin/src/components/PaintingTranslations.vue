<script setup lang="ts">
import { useDebounceFn } from '@vueuse/core';
import { ref, type Ref } from 'vue';
import type {TPaintingStub} from "../sdk/api.ts";

type TEventKey = keyof TPaintingStub;
export type TEventPaintingTranslations = {
	key: TEventKey;
	lang: 'cs' | 'en';
	value: string;
};

const emits = defineEmits<{
	(e: 'modelUpdate', value: TEventPaintingTranslations): void
}>();
const props = defineProps<{
	edit: boolean,
	titleCs: Ref<string> | string,
	titleEn: Ref<string> | string,
	descriptionCs: Ref<string> | string,
	descriptionEn: Ref<string> | string,
}>()

const tab = ref<number>(0);

const debouncedModelUpdate = useDebounceFn((key: TEventKey, lang: 'cs' | 'en', value: string) => {
	emits('modelUpdate', {key, lang, value});
}, 200);
</script>

<template>
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
		<v-tabs-window-item>
			<v-container fluid>
				<v-row>
					<v-col>
						<v-text-field
							type="text"
							label="Název"
							:readonly="!props.edit"
							:model-value="props.titleCs"
							variant="outlined"
							:bg-color="(props.edit) ? 'grey-lighten-3' : 'transparent'"
							@update:modelValue="debouncedModelUpdate(
								'painting_title',
								'cs',
								$event as string
							)"
						></v-text-field>
					</v-col>
				</v-row>
				<v-row>
					<v-col>
						<v-text-field
							type="text"
							label="Popis"
							:readonly="!edit"
							:model-value="props.descriptionCs"
							variant="outlined"
							:bg-color="(props.edit) ? 'grey-lighten-3' : 'transparent'"
							@update:modelValue="debouncedModelUpdate(
								'painting_description',
								'cs',
								$event as string
							)"
						></v-text-field>
					</v-col>
				</v-row>
			</v-container>
			<v-container fluid>
				<v-row>
					<v-col>
						<v-text-field
							type="text"
							label="Název"
							:readonly="!props.edit"
							:model-value="props.titleEn"
							variant="outlined"
							:bg-color="(props.edit) ? 'grey-lighten-3' : 'transparent'"
							@update:modelValue="debouncedModelUpdate(
								'painting_title',
								'en',
								$event as string
							)"
						></v-text-field>
					</v-col>
				</v-row>
				<v-row>
					<v-col>
						<v-text-field
							type="text"
							label="Popis"
							:readonly="!props.edit"
							:model-value="props.descriptionEn"
							variant="outlined"
							:bg-color="(props.edit) ? 'grey-lighten-3' : 'transparent'"
							@update:modelValue="debouncedModelUpdate(
								'painting_description',
								'en',
								$event as string
							)"
						></v-text-field>
					</v-col>
				</v-row>
			</v-container>
		</v-tabs-window-item>
	</v-tabs-window>
</v-card>
</template>