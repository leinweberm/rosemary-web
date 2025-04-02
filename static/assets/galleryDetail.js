// @ts-check

const imageSrcs = [];

const openLargeImage = (event, src) => {
	console.log(event.target.src);
	console.log('src', src);
}

const initDetailPage = () => {
	/** @type {NodeListOf<HTMLImageElement>} */
	const images = document.querySelectorAll('.paintingImage');
	console.log('images', images);
	// images.forEach((image) => {
		// console.log('image', image.src);
		// if (image && image.src) {
		// const baseSrc = image.src.replace(/_\d+\.jpg$/, '');
		// }
	// });
};

document.addEventListener('DOMContentLoaded', () => {
	initDetailPage();
});