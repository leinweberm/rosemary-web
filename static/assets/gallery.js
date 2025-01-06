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
	window.location.href = `/gallery?${filters.toString()}`;
}

const updateBrowserUrl = () => {
	history.replaceState({filters}, 'Rosemary', `/gallery?${filters.toString()}`);
};

const resetFilters = () => {
	filters = new URLSearchParams(getDefaultQuery());
	redirectWithFilters();
}

document.addEventListener('DOMContentLoaded', () => {

});