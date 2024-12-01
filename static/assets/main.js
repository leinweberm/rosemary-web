let lastScroll = 0;
let navHidden = false;
let isNavBig = true;

const prepareNavbarCheckbox = () => {
	const checkbox = document.querySelector('#navToggleCheckbox');
	checkbox && checkbox.addEventListener('change', (event) => {
		const nav = document.querySelector('#nav');
		const { checked } = event.target;
		nav.style.top = '0';
		document.body.style.height = (checked) ? '100dvh' : 'unset';
		document.body.style.overflow = (checked) ? 'hidden' : 'unset';
	});
};

const prepareNavbarPosition = () => {
	const nav = document.querySelector('#nav');
	nav && window.addEventListener('scroll', () => {
		let scroll = Math.round(window.scrollY);
		console.log('scroll', scroll);
		if (scroll > lastScroll && isNavBig) {
			nav.classList.add('navSmall');
			isNavBig = false;
		} else if (scroll < lastScroll && !isNavBig) {
			nav.classList.remove('navSmall');
			isNavBig = true;
		}
		lastScroll = scroll;
	});
};

const addEventListeners = () => {
	prepareNavbarCheckbox();
	prepareNavbarPosition();
};

document.addEventListener('DOMContentLoaded', () => {
	addEventListeners();
});