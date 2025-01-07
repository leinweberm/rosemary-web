const pagePath = 'gallery.html';
const dirty = false;
const getDefaultQuery = () => {
	return {
		sort: 'created',
		order: 'desc',
		limit: '25',
		offset: '0',
	};
};
let filters = new URLSearchParams(getDefaultQuery());

const parseUrlQuery = () => {
	const params = new URLSearchParams(document.location.search);
	const sort = params.get('sort');
	const order = params.get('order');
	const limit = parseInt(params.get('limit'));
	const offset = parseInt(params.get('offset'));
	const search = params.get('search');

	if (sort && ['created', 'sold', 'title', 'price', 'width', 'height'].includes(sort)) {
		filters.set('sort', sort);
	}
	if (order && (order === 'asc' || order === 'desc')) {
		filters.set('order', order);
	}
	if (!isNaN(limit)) {
		filters.set('limit', limit.toString());
	}
	if (!isNaN(offset)) {
		filters.set('offset', offset.toString());
	}
	if (search && search.trim().length) {
		filters.set('search', search);
	}
};

const redirectWithFilters = () => {
	window.location.href = `/${pagePath}?${filters.toString()}`;
}

const resetFilters = () => {
	filters = new URLSearchParams(getDefaultQuery());
	redirectWithFilters();
};

const handleFilterEvent = (event) => {
	event.stopPropagation();
	filters.set(event.target.name, event.target.value);
	redirectWithFilters();
};

const addInputEventListeners = () => {
	const sortInput = document.querySelector('.galleryFilters > select:nth-child(1)');
	if (sortInput) {
		sortInput.addEventListener('change', event => handleFilterEvent(event));
		sortInput.value = filters.get('sort');
	}

	const orderInput = document.querySelector('.galleryFilters > select:nth-child(2)');
	if (orderInput) {
		orderInput.addEventListener('change', event => handleFilterEvent(event));
		orderInput.value = filters.get('order');
	}

	const searchInput = document.querySelector('.galleryFilters > input');
	if (searchInput) {
		searchInput.addEventListener(
			'input',
			window.utils.debounce((event) => {handleFilterEvent(event)}, 300)
		);
		searchInput.value = filters.get('search') || '';
	}
};

const onLoad = () => {
	parseUrlQuery();
	addInputEventListeners();
};

document.addEventListener('DOMContentLoaded', () => {
	onLoad();
});