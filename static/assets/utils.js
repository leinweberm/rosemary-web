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

/**
 * @param {TouchEvent} event -
 * @return {(event: TouchEvent) => number} 0 = left, 1 = right
 */
const handleSwipe = (event) => {
	const startPosition = event.changedTouches[0].screenX;
	return (endEvent) => {
		const endPosition = endEvent.changedTouches[0].screenX;
		const swipeDistance = endPosition - startPosition;
		if (
			(Math.abs(swipeDistance) > -50) &&
			(Math.abs(swipeDistance) < 50)
		) {
			return;
		}
		return (swipeDistance < 0) ? 0 : 1;
	};
};

window.utils = {
	debounce,
	throttle,
	handleSwipe,
};