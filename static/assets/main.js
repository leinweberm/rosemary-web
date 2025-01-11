let lastScroll = 0;
let navHidden = false;
let isNavBig = true;

const prepareNavbarPosition = () => {
	const nav = document.querySelector('#nav');
	nav && window.addEventListener('scroll', window.utils.throttle(() => {
		let scroll = Math.round(window.scrollY);
		if (scroll > lastScroll && isNavBig) {
			nav.classList.add('navSmall');
			isNavBig = false;
		} else if (scroll < lastScroll && !isNavBig) {
			nav.classList.remove('navSmall');
			isNavBig = true;
		}
		lastScroll = scroll;
	}, 100));
};

const addEventListeners = () => {
	prepareNavbarPosition();
};

document.addEventListener('DOMContentLoaded', () => {
	addEventListeners();
});