import axios from "axios";
import { TUser } from "../stores/userStore";

export class ApiSDK {
	private url:string = 'http://127.0.0.1:3030';

	constructor() {}

	async login(username: string, password: string): Promise<TUser | null> {
		const response = await axios.post(
			`${this.url}/api/v1.0/users/login`,
			{ username, password },
			{
				headers: {
					'Content-Type': 'application/json'
				}
			}
		);

		if (response.status === 200) {
			return { ui: response.data.ui, token: response.data.token };
		} else if (response.status === 401) {
			window.alert('Neplatné přihlašovací údaje!');
			return null;
		} else {
			window.alert('Jejda, něco se pokazilo!');
			return null;
		}
	}

	async refreshToken(token: string): Promise<any> {
		const response = await axios.get(`${this.url}/api/v1.0/users/refresh/token`, {
			headers: {
				'Authorization': `Bearer ${token}`
			}
		});
		if (response.status === 200 && response.data.token) {
			return response.data.token;
		} else {
			return;
		}
	}

	async createPainting(data: TCreatePainting, token: string): Promise<any> {
		try {
			const response = await axios.post(
				`${this.url}/api/v1.0/paintings`,
				data,
				{headers: {
					'Authorization': `Bearer ${token}`,
					'Content-Type': 'application/json'
				}
			});
			if (response.status !== 201 || response.data.status !== 'Success') {
				window.alert('Nepodařilo se nahrát obraz');
				return;
			} else {
				return response.data;
			}
		} catch (error) {
			console.error(error);
			window.alert('Nepodařilo se nahrát obraz');
			return;
		}
	}

	async uploadPaintingImage(file: File, query: TUploadImagePaintingQuery, token: string): Promise<boolean> {
		const queryObj = new URLSearchParams(query);
		try {
			const response = await axios.postForm(
				`${this.url}/api/v1.0/images?${queryObj.toString()}`,
				{file},
				{headers: {
					'Authorization': `Bearer ${token}`
				}}
			);
			if (response.status !== 201 || response.data.status !== 'Success') {
				window.alert('Nepovedlo se nahrát fotografii obrazu');
				return false;
			}
		} catch (error) {
			console.error(error);
			window.alert('Nepovedlo se nahrát fotografii obrazu');
			return false;
		}
		return true;
	}
}

export type TCreatePainting = {
	price: number;
	width: number;
	height: number;
	title_cs: string;
	title_en: string;
	description_cs: string;
	description_en: string;
};

export type TUploadImagePaintingQuery = {
	preview: 'true' | 'false',
	title_cs: string;
	title_en: string;
	alt_cs: string;
	alt_en: string;
	painting_id: string;
};