<script setup lang="ts">
import { useDebounceFn } from '@vueuse/core';
import {type Ref} from 'vue';
import type {TEventPaintingImageRow, TEventPaintingImageRowKey} from "../composable/image/imageChangeEvent.ts";

// const emits = defineEmits(['modelUpdate']);
const emits = defineEmits<{
	(e: 'modelUpdate', value: TEventPaintingImageRow ): void,
	(e: 'previewSelect', value: boolean): void,
	(e: 'removePaintingImage'): void,
}>();
const props = defineProps<{
	paintingId?: string,
	imageId?: string,
	imageUrl: string,
	currentPreviewUrl?: Ref<string> | string,
	titleCs: Ref<string> | string,
	titleEn: Ref<string> | string,
	altCs: Ref<string> | string,
	altEn: Ref<string> | string,
	edit: Ref<boolean> | boolean,
}>();

const debouncedModelUpdate = useDebounceFn((key: TEventPaintingImageRowKey, value: string) => {
	emits('modelUpdate', {key, value});
}, 200);
</script>

<template>
	<div class="previewImageRow" style="position: relative;" v-if="imageUrl">
		<div
			class="previewImageRowInner"
			@click.stop="emits('previewSelect', true)"
		>
			<div
				:class="{
					imagePreviewWrapperSelected: (props.currentPreviewUrl && imageUrl.includes(props.currentPreviewUrl as string)),
					imagePreviewItem: (props.currentPreviewUrl || !imageUrl.includes(props.currentPreviewUrl as string)),
				}"
				style="cursor: pointer; display: flex"
			>
				<img
					class="paintingPaintingImage"
					:src="props.imageUrl"
					alt="pole"
				>
			</div>
		</div>
		<div class="imageMetaData">
			<v-row>
				<v-col>
					<v-text-field
						:model-value="props.titleCs"
						type="text"
						variant="outlined"
						:readonly="!edit"
						:bg-color="(edit) ? 'grey-lighten-3' : 'transparent'"
						label="Název CZ"
						@update:modelValue="debouncedModelUpdate('title_cs', $event as string)"
					></v-text-field>
				</v-col>
			</v-row>
			<v-row>
				<v-col>
					<v-text-field
						:model-value="props.altCs"
						type="text"
						variant="outlined"
						:readonly="!edit"
						:bg-color="(edit) ? 'grey-lighten-3' : 'transparent'"
						label="Alt CZ"
						@update:modelValue="debouncedModelUpdate('alt_cs', $event as string)"
					></v-text-field>
				</v-col>
			</v-row>
		</div>
		<div class="imageMetaData">
			<v-row>
				<v-col>
					<v-text-field
						:model-value="props.titleEn"
						type="text"
						variant="outlined"
						:readonly="!edit"
						:bg-color="(edit) ? 'grey-lighten-3' : 'transparent'"
						label="Název EN"
						@update:modelValue="debouncedModelUpdate('title_en', $event as string)"
					></v-text-field>
				</v-col>
			</v-row>
			<v-row>
				<v-col>
					<v-text-field
						:model-value="props.altEn"
						type="text"
						variant="outlined"
						:readonly="!edit"
						:bg-color="(edit) ? 'grey-lighten-3' : 'transparent'"
						label="Alt EN"
						@update:modelValue="debouncedModelUpdate('alt_en', $event as string)"
					></v-text-field>
				</v-col>
			</v-row>
		</div>
		<v-btn
			class="removePaintingImageButton"
			variant="tonal"
			color="error"
			@click="emits('removePaintingImage')"
		>Odstranit</v-btn>
	</div>
	<v-divider style="margin: 20px 0"/>
</template>

<style scoped>
.removePaintingImageButton {
	z-index: 5;
	position: absolute;
	right: 30px;
	bottom: 0;
}
</style>