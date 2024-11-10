import {should} from 'chai';
import {createReadStream} from 'fs';
import FormData from 'form-data';
should();

const serviceUrl = 'http://localhost:3030/api/v1.0';

describe('painting images test', function () {
	let token = null;
	let paintingId = null;

	it('create painting with images', async function () {
		const userRequestBody = {
			username: `imageUpload${new Date().getTime()}`,
			password: 'toor',
			secret: 'BibiHvezdaJoyMartinMatildaMichaela'
		};
		const userRequest = new Request(`${serviceUrl}/users`, {
			method: 'POST',
			body: JSON.stringify(userRequestBody),
			headers: {
				'Content-Type': 'application/json',
				'Content-Length': `${Buffer.byteLength(JSON.stringify(userRequestBody))}`
			}
		});
		const userResponse = await fetch(userRequest);
		const userData = await userResponse.json();
		userResponse.status.should.equal(201);
		token = userData.token;

		const paintingRequestBody = {
			price: 20_000,
			title_cs: 'Andel',
			title_en: 'Angel',
			description_cs: 'obraz zobrazujici andela',
			description_en: 'picture of angel',
			width: 60,
			height: 90
		};
		const paintingRequest = new Request(`${serviceUrl}/paintings`, {
			method: 'POST',
			body: JSON.stringify(paintingRequestBody),
			headers: {
				'Authorization': `Bearer ${token}`,
				'Content-Type': 'application/json',
				'Content-Length': `${Buffer.byteLength(JSON.stringify(paintingRequestBody))}`
			}
		});
		const paintingResponse = await fetch(paintingRequest);
		const paintingData = await paintingResponse.json();
		paintingResponse.status.should.equal(201);
		paintingId = paintingData.data.id;

		const formData = new FormData()
		formData.append('image', createReadStream('../images/preview.jpg'));

		const query = new URLSearchParams();
		query.append('preview', 'true');
		query.append('title_cs', 'obrazNahled');
		query.append('title_en', 'imagePreview');
		query.append('alt_cs', 'altNahledovehoObrazku');
		query.append('alt_en', 'altPreviewImages');
		query.append('painting_id', paintingId);

		const request = new Request(`${serviceUrl}/images?${query.toString()}`, {
			method: 'POST',
			body: formData,
			headers: {
				...formData.getHeaders(),
				'Authorization': `Bearer ${token}`
			}
		});

		const response = await fetch(request);
		const data = await response.json();
		console.log('response', response);
		console.log('data', data);
	});
})