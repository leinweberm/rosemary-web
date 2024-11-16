import { defineStore } from 'pinia';
import { ref, computed, inject } from 'vue';
import { ApiSDK } from '../sdk/api';
import { jwtDecode } from 'jwt-decode';
import { useRouter} from "vue-router";
import { routesOpts} from "../router/router.ts";

export type TUser = {
	ui: string;
	token: string;
	tokenExpiration?: number;
};

export const useUserStore = defineStore('user', () => {
	const user = ref<TUser | null>(null);
	const ApiSDK: ApiSDK | undefined = inject('ApiSDK');
	const router = useRouter();

	const getUser = computed(() => {
		console.log('user computed', user.value);
		return user.value;
	});

	async function authRouteAccess(): Promise<boolean | undefined> {
		const now = (new Date().getTime()) / 1000;

		if (!ApiSDK) {
			user.value = null;
			return;
		}
		console.log('auth guard - apiSDK OK');

		if (!user.value) {
			let checkSession = sessionStorage.getItem('user');
			if (checkSession) {
				checkSession = (JSON.parse(checkSession));
				// @ts-expect-error
				if (checkSession && checkSession.token && checkSession.ui && checkSession.tokenExpiration) {
					// @ts-expect-error
					user.value = {ui: checkSession.ui, token: checkSession.token, tokenExpiration: checkSession.tokenExpiration};
					console.log('auth guard - loaded user from session OK');
				}
			}
		}

		if (!user.value || !user.value.token || !user.value.tokenExpiration) {
			console.log('no valid user provided');
			user.value = null;
			return;
		}

		if (now >= user.value.tokenExpiration) {
			console.log('expired token', now, user.value.tokenExpiration);
			console.log(new Date(now).toISOString(), new Date(user.value.tokenExpiration).toISOString());
			user.value = null;
			return false;
		}

		if ((user.value.tokenExpiration - now) < 600) {
			const newToken = await ApiSDK.refreshToken(user.value.token);
			if (!newToken) {
				user.value = null;
				return;
			}
			const newDecoded = jwtDecode(newToken);
			user.value.token = newToken;
			user.value.tokenExpiration = newDecoded.exp;
			return true;
		} else if ((user.value.tokenExpiration - now) > 600) {
			return true;
		}

		user.value = null;
		return false;
	};

	async function login(username: string, password: string): Promise<boolean> {
		if (!ApiSDK) {
			window.alert('Jejda, něco se pokazilo!');
			return false;
		}

		const response = await ApiSDK.login(username, password);

		if (response) {
			const decoded = jwtDecode(response.token);
			user.value = {
				ui: response.ui,
				token: response.token,
				tokenExpiration: decoded.exp,
			}
			sessionStorage.setItem('user', JSON.stringify(user.value));
			return true;
		} else {
			return false;
		}
	}

	async function logout(): Promise<void> {
		sessionStorage.clear();
		user.value = null;
		await router.push({name: routesOpts.Login});
	}

	return {
		getUser,
		login,
		authRouteAccess,
		logout,
	}
});