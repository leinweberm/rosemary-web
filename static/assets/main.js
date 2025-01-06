let lastScroll = 0;
let navHidden = false;
let isNavBig = true;

const debounce = (func, delay) => {
	let timeout;
	return (...args) => {
		clearTimeout(timeout);
		timeout = setTimeout(() => func(...args), delay);
	};
}

const throttle = (func, limit) => {
	let lastFunc;
	let lastRan;
	return (...args) => {
		const context = this;
		const now = Date.now();

		if (!lastRan) {
			func.apply(context, args);
			lastRan = now;
		} else {
			clearTimeout(lastFunc);
			lastFunc = setTimeout(() => {
				if (now - lastRan >= limit) {
					func.apply(context, args);
					lastRan = now;
				}
			}, limit - (now - lastRan));
		}
	};
};

const prepareNavbarPosition = () => {
	const nav = document.querySelector('#nav');
	nav && window.addEventListener('scroll', throttle(() => {
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