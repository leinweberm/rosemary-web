import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export type TUser = {
	id: string;
	username: string;
	access_token: string;
};

export const userStore = defineStore('user', () => {
	const user = ref<TUser | null>(null);

	const getUser = computed(() => user.value);

	function login(username: string, password: string): boolean {
		console.log('username', username, 'password', password);
		return true;
	}

	return {
		getUser,
		login
	}
});