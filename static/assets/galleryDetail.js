// @ts-check
const imageSrcs = [];
let currentImageIndex = 0;

/**
 * @param {number} tempIndex -
 * @return {void} imageSrc -
 */
const updateDialogImage = (tempIndex) => {
	/** @type {HTMLImageElement | null} */
	const dialogImage = document.querySelector('#imageDialogImage');
	if (!dialogImage) return;
	let finalIndex = 0;
	const lastIndex = imageSrcs.length - 1;

	if (tempIndex < 0) {
		finalIndex = lastIndex;
	} else if (tempIndex > lastIndex) {
		finalIndex = 0;
	} else {
		finalIndex = tempIndex;
	}

	currentImageIndex = finalIndex;
	let baseUrl = imageSrcs[finalIndex].replace(/_\d+\.jpg$/, '');
	dialogImage.src=`${baseUrl}_${dialogImage.getAttribute('maxWidth')}.jpg`;
};

const initDetailPage = () => {
	/** @type {HTMLDialogElement | null} */
	const dialog = document.querySelector('#imageDialog');
	/** @type {HTMLButtonElement | null} */
	const closeButton = document.querySelector('#imageDialogClose');
	/** @type {HTMLButtonElement | null} */
	const prevButton = document.querySelector('#imageDialogPrev');
	/** @type {HTMLButtonElement | null} */
	const nextButton = document.querySelector('#imageDialogNext');
	/** @type {HTMLImageElement | null} */
	const dialogImage = document.querySelector('#imageDialogImage');
	/** @type {NodeListOf<HTMLImageElement>} */
	const images = document.querySelectorAll('.paintingImage');

	if (!dialog || !dialogImage) return;

	(closeButton) && closeButton.addEventListener('click', () => {
		dialog.close();
	});
	(dialog) && dialog.addEventListener('click', () => {
		dialog.close();
	});
	(prevButton) && prevButton.addEventListener('click', (e) => {
		e.stopPropagation();
		updateDialogImage(currentImageIndex - 1);
	});
	(nextButton) && nextButton.addEventListener('click', (e) => {
		e.stopPropagation();
		updateDialogImage(currentImageIndex + 1);
	});

	images.forEach((image, index) => {
		if (image && image.src) {
			const baseSrc = image.src.replace(/_\d+\.jpg$/, '');
			imageSrcs.push(baseSrc);

			image.addEventListener('click', () => {
				currentImageIndex = index;
				dialogImage.src = `${baseSrc}_${dialogImage.getAttribute('maxWidth')}.jpg`;
				dialog.showModal();
			});
		}
	});
};

document.addEventListener('DOMContentLoaded', () => {
	initDetailPage();
});