const debounce = (func, delay) => {
	let timeout;
	return function(...args) {
		const context = this; // Capture the correct context
		clearTimeout(timeout);
		timeout = setTimeout(() => func.apply(context, args), delay);
	};
};

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

window.utils = {
	debounce,
	throttle,
};