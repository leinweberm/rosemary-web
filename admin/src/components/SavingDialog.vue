<script setup lang="ts">
import type { Ref } from 'vue';
const props = defineProps<{
	isSaving: Ref<number> | number,
	saveProgress: Ref<number> | number,
	cb: () => any | Promise<any>
}>();
</script>

<template>
	<dialog v-if="props.isSaving" id="savingDialog" class="elevation-7">
		<div style="width: 100%">
			<h2 style="text-align: center">
				{{ (isSaving === 2) ? 'ULOŽENO' : 'PROBÍHÁ UKLÁDÁNÍ' }}
			</h2>
		</div>
		<div style="width: 100%; display: flex; align-items: center; margin-top: 20px">
			<v-progress-linear
				color="amber"
				height="50"
				:model-value="saveProgress as number"
			>
				<strong>{{ Math.ceil(props.saveProgress as number) }}%</strong>
			</v-progress-linear>
		</div>
		<v-btn
			v-if="props.isSaving === 2"
			variant="elevated"
			size="large"
			color="success"
			style="margin-top: 20px"
			@click.stop="props.cb()"
		>
			OK
		</v-btn>
	</dialog>
</template>