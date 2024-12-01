<script setup lang="ts">
import { useDebounceFn } from '@vueuse/core';
import type {TEventPaintingInformation, TEventPaintingInformationKey} from "../composable/painting/paintingChangeEvent.ts";


const emits = defineEmits<{
	(e: 'modelUpdate', value: TEventPaintingInformation): void
}>();
const props = defineProps<{
	imageSrc?: string,
	price: number,
	height: number,
	width: number,
	edit: boolean,
}>();

const debouncedModelUpdate = useDebounceFn((key: TEventPaintingInformationKey, value: string) => {
	emits('modelUpdate', {key, value});
}, 1000);
</script>

<template>
<v-card>
	<v-card-title style="padding: 10px 25px">Informace</v-card-title>
	<v-container>
		<v-row>
			<v-col>
				<div class="imageWrapper">
					<img
						v-if="props.imageSrc"
						:src="props.imageSrc"
						alt="this specific image preview"
						class="paintingPaintingImage"
					>
				</div>
			</v-col>
		</v-row>
		<v-row>
			<v-col>
				<v-text-field
					:model-value="props.price"
					label="cena"
					type="number"
					min="0"
					max="100000"
					:readonly="!props.edit"
					variant="outlined"
					:bg-color="(props.edit) ? 'grey-lighten-3' : 'transparent'"
					@update:modelValue="debouncedModelUpdate('price', $event)"
				></v-text-field>
				<v-text-field
					:model-value="props.height"
					label="výška (cm)"
					type="number"
					min="0"
					max="300"
					:readonly="!edit"
					variant="outlined"
					:bg-color="(edit) ? 'grey-lighten-3' : 'transparent'"
					@update:modelValue="debouncedModelUpdate('height', $event)"
				></v-text-field>
				<v-text-field
					:model-value="props.width"
					label="šířka (cm)"
					type="number"
					min="0"
					max="300"
					:readonly="!edit"
					variant="outlined"
					:bg-color="(edit) ? 'grey-lighten-3' : 'transparent'"
					@update:modelValue="debouncedModelUpdate('width', $event)"
				></v-text-field>
			</v-col>
		</v-row>
	</v-container>
</v-card>
</template>