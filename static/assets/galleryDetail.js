const { handleSwipe } = window.utils;
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

const addOrderEvents = () => {
	/** @type {HTMLButtonElement | null} */
	const buyButton = document.querySelector('#buyButton');
	/** @type {HTMLDialogElement | null} */
	const buyDialog = document.querySelector('#buyDialog');
	if (!buyButton || !buyDialog) return;

	buyButton.addEventListener('click', (e) => {
		e.stopPropagation();
		buyDialog.showModal();
	});

	buyDialog.addEventListener('click', (e) => {
		e.stopPropagation();
		buyDialog.close();
	});
};

const addPhotoEvents = () => {
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
	/** @type {HTMLDivElement | null} */
	const dialogBody = document.querySelector('#imageDialogBody');
	/** @type {NodeListOf<HTMLButtonElement>} */
	const dialogControls = document.querySelectorAll('#imageDialogBody > button');

	if (!dialog || !dialogImage) return;

	// Add Event Listeners
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
	if (dialogBody) {
		dialogBody.addEventListener('keyup', (e) => {
			e.stopPropagation();
			if (e.key !== 'ArrowLeft' && e.key !== 'ArrowRight') return;
			const newImageIndex = currentImageIndex + (
				(e.key === 'ArrowRight') ? 1 : -1
			);
			updateDialogImage(newImageIndex);
		});
		let touchHandler;
		dialogBody.addEventListener('touchstart', (e) => {
			touchHandler = handleSwipe(e);
		});
		dialogBody.addEventListener('touchend', (e) => {
			const isNext = touchHandler(e);
			const newImageIndex = currentImageIndex + (isNext ? 1 : -1);
			updateDialogImage(newImageIndex);
		});
	}

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

	(dialogControls[1]) && dialogControls[1].addEventListener('click', (e) => {
		e.stopPropagation();
		updateDialogImage(currentImageIndex - 1);
	});
	(dialogControls[2]) && dialogControls[2].addEventListener('click', (e) => {
		e.stopPropagation();
		updateDialogImage(currentImageIndex + 1);
	});
};

const initDetailPage = () => {
	addPhotoEvents();
	addOrderEvents();
};

const nextImage = () => {
	updateDialogImage(currentImageIndex + 1);
};

const prevImage = () => {
	updateDialogImage(currentImageIndex - 1);
};

document.addEventListener('DOMContentLoaded', () => {
	initDetailPage();
});