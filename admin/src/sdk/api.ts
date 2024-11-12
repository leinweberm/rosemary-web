import axios from "axios";
import { TUser } from "../stores/userStore";
// import { TUser } from "../stores/userStore";

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
}