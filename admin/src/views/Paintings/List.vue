<script setup lang="ts">
import {useRoute, useRouter} from "vue-router";
import {inject, onMounted, ref} from "vue";
import {ApiSDK as SDK, TListPaintingQuery, TPaintingStub} from "../../sdk/api.ts";
import {routesOpts} from "../../router/router.ts";

type TSortBy = 'price' | 'width' | 'height' | 'title' | 'created' | 'sold' | 'description';

type TTableEvent = {
	page: number;
	itemsPerPage: number;
	search: string;
	sortBy: Array<{key: TSortBy, order: 'asc' | 'desc'}>;
}

const router = useRouter();
const route = useRoute();
// @ts-expect-error
const ApiSDK: SDK = inject<SDK>('ApiSDK');
const query  = ref<TListPaintingQuery>({
	limit: 20,
	offset: 0,
	sort: 'created',
	order: 'DESC',
	search: '',
});
const loading = ref<boolean>(false);
const count = ref<number>(0);
const items = ref<TPaintingStub[]>([]);
const currentSort = ref<Array<{key: TSortBy, order: 'asc' | 'desc'}>>([{key: 'created', order: 'desc'}]);
const sortOptions = ['price', 'width', 'height', 'title', 'created', 'sold', 'description'];
const paginationOptions = [
	{title: '5', value: 5},
	{title: '10', value: 10},
	{title: '15', value: 15},
	{title: '20', value: 20},
	{title: '25', value: 25},
]

const fetchPaintings = async () => {
	loading.value = true;
	const data = await ApiSDK?.listPaintings(query.value);
	if (!data) {
		loading.value = false;
		return;
	}
	count.value = data.count;
	items.value = data.rows;
	loading.value = false;
};

const headers = [
	{
		title: 'Náhled',
		key: 'preview',
		align: 'center',
	},
	{
		title: 'Název',
		align: 'start',
		key: 'title',
		sortable: true,
	},
	{
		title: 'Vytvořeno',
		align: 'center',
		key: 'created',
		sortable: true,
	},
	{
		title: 'Šířka',
		key: 'width',
		align: 'end',
		sortable: true,
	},
	{
		title: 'Výška',
		key: 'height',
		align: 'end',
		sortable: true,
	},
	{
		title: 'Cena',
		key: 'price',
		align: 'end',
		sortable: true,
	},
	{
		title: 'Prodáno',
		key: 'sold',
		align: 'center',
		sortable: true,
	}
];

const loadItems = async (event: TTableEvent) => {
	query.value = {
		limit: event.itemsPerPage || 25,
		offset: (event.itemsPerPage * (event.page - 1)),
		search: event.search || '',
		sort: event.sortBy[0]?.key || 'created',
		order: event.sortBy[0]?.order || 'desc',
	};
	currentSort.value[0] = {...event.sortBy[0]};
	await fetchPaintings();
};

const openDetail = async (id: string) => {
	await router.push({
		name: routesOpts.P_DETAIL,
		params: { id },
	});
};

onMounted(async () => {
	if (route.query.limit) {
		let limit = parseInt(route.query.limit as string);
		if (limit > 25) limit = 25;
		if (limit < 0) limit = 0;
		query.value.limit = limit;
	}
	if (route.query.offset) {
		query.value.offset = parseInt(route.query.offset as string);
	}
	if (route.query.sort && sortOptions.includes(route.query.sort as string)) {
		// @ts-expect-error
		query.value.sort = route.query.sort as string;
	}
	if (route.query.order && route.query.order === 'DESC' || route.query.order === 'ASC') {
		query.value.order = (route.query.order === 'DESC') ? 'DESC' : 'ASC';
	}
	if (route.query.search) {
		query.value.search = route.query.search as string;
	}
	await fetchPaintings();
});
</script>

<template>
	<v-container>
		<h2>Obrazy</h2>
	</v-container>
	<v-container>
		<v-data-table-server
			:items="items"
			:items-length="count"
			:loading="loading"
			:search="query.search"
			:fixed-header="true"
			:fixed-footer="true"
			:headers="headers"
			:items-per-page-options="paginationOptions"
			sort-asc-icon="mdi-arrow-up"
			sort-desc-icon="mdi-arrow-down"
			:sort-by="currentSort"
			@update:options="loadItems"
		>
			<template v-slot:item.preview="{ item }">
				<div style="height: 160px; width: 160px; display: flex; position: relative">
					<img
						class="previewPaintingImage"
						:src="`${ApiSDK.staticUrl}/${item.preview.urls[0]}`"
						:alt="item.painting_title.cs"
						@click.stop="openDetail(item.id)"
					>
				</div>
			</template>
			<template v-slot:item.title="{ item }">
				{{ item.painting_title.cs }}
			</template>
			<template v-slot:item.created="{ item }">
				{{ new Date(item.created).toLocaleDateString('cs') }}
			</template>
			<template v-slot:item.sold="{ item }">
				{{ item.data?.sold ?  'ANO' : 'NE' }}
			</template>
		</v-data-table-server>
	</v-container>
</template>

<style scoped>
.previewPaintingImage {
	max-height: 160px;
	max-width: 160px;
	object-fit: contain;
	cursor: pointer;
}

.previewPaintingImage:hover {
	border: 2px solid transparent;
	transform: scale(1.2);
	position: absolute;
	top: 25px;
	transition: transition ease-in-out 200ms;
}
</style>