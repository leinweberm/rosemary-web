import axios from "axios";
import { TUser } from "../stores/userStore";

export class ApiSDK {
	private url:string = 'http://localhost';
	staticUrl:string = 'http://static.localhost';

	constructor() {}

	async login(username: string, password: string): Promise<TUser | null> {
		const response = await axios.post(
			`${this.url}/api/v1.0/users/login`,
			{ username, password },
			{
				headers: {
					'Content-Type': 'application/json',
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
				'Authorization': `Bearer ${token}`,
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
				{
					headers: {
						'Authorization': `Bearer ${token}`,
						'Content-Type': 'application/json',
					}
				}
			);
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
				{
					headers: {
						'Authorization': `Bearer ${token}`,
					}
				}
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

	async listPaintings(query: TListPaintingQuery): Promise<TPaginatedResult<TPaintingStub>> {
		// @ts-expect-error
		const queryObj = new URLSearchParams(query);
		try {
			const response = await axios.get<TPaginatedResult<TPaintingStub>>(
					`${this.url}/api/v1.0/paintings?${queryObj.toString()}`,
				{
					headers: {}
				}
			);
			return response.data;
		} catch (error) {
			console.error(error);
			window.alert('Jejda, něco se pokazilo!');
			return {count: 0, rows: []};
		}
	}

	async getPaintingDetail(id: string): Promise<TPaintingDetail | null> {
		try {
			const response = await axios.get(
				`${this.url}/api/v1.0/paintings/${id}`,
				{headers: {}}
			);
			return response.data;
		}	catch (error) {
			console.error(error);
			window.alert('Jejda, něco se pokazilo!');
			return null;
		}
	}

	async updatePainting(id: string, data: TUpdatePainting, token: string): Promise<bool> {
		try {
			const response = await axios.patch(
				`${this.url}/api/v1.0/paintings/${id}`,
				data,
				{
					headers: {
						'Authorization': `Bearer ${token}`,
					}
				}
			);
			return true;
		} catch (error) {
			console.error(error);
			window.alert('Jejda, něco se pokazilo!');
			return false;
		}
	}
}

export type TTranslation = {
	cs: string;
	en: string;
};

export type TPaintingImage = {
	id: string;
	preview: boolean;
	alt: TTranslation;
	title: TTranslation;
	painting_id: string;
	status: 'CREATED' | 'PROCESSED';
	file_location: string | null;
	urls: string[];
};

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

export type TListPaintingQuery = {
		limit ?: number;
		offset?: number;
		order?: 'ASC' | 'DESC' | 'asc' | 'desc';
		sort?: 'price' | 'width' | 'height' | 'title' | 'created' | 'sold' | 'description';
		search?: string;
};

export type TPaginatedResult<T> = {
	count: number;
	rows: Array<T>;
};

export type TPaintingStub = {
	id: string;
	created: string;
	deleted: string | null;
	price: number;
	painting_title: TTranslation;
	painting_description: TTranslation;
	width: number;
	height: number;
	data: {
		sold?: boolean;
	};
	preview: TPaintingImage;
}

export type TPaintingDetail = {
	painting: TPaintingStub;
	images: TPaintingImage[];
};

export type TUpdatePainting = {
	title_cs?: string;
	title_en?: string;
	description_cs?: string;
	description_en?: string;
	height?: number;
	width?: number;
	price?: number;
};