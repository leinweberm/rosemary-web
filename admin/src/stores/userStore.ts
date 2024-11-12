import { defineStore } from 'pinia';
import { ref, computed, inject } from 'vue';
import { ApiSDK } from '../sdk/api';

export type TUser = {
	ui: string;
	token: string;
};

export const useUserStore = defineStore('user', () => {
	const user = ref<TUser | null>(null);
	const ApiSDK: ApiSDK | undefined = inject('ApiSDK');

	const getUser = computed(() => user);

	async function login(username: string, password: string): Promise<boolean> {
		if (!ApiSDK) {
			window.alert('Jejda, něco se pokazilo!');
			return false;
		}

		const response = await ApiSDK.login(username, password);

		if (response) {
			user.value = { ui: response.ui, token: response.token };
			return true;
		} else {
			return false;
		}
	}

	return {
		getUser,
		login
	}
});