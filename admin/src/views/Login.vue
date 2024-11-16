<script setup lang="ts">
import { ref, computed } from 'vue';
import { useUserStore } from '../stores/userStore';
import { useRouter } from 'vue-router';

const username = ref<string>("");
const password = ref<string>("");
const loginForm = ref();
const userStore = useUserStore();
const router = useRouter();

const nameRules = [
	(value: string) => (!!value && !!value.length) || 'Chybí uživatelské jméno',
];

const passwordRules = [
	(value: string) => !!value || 'Chybí heslo',
	// (value: string) => (value.length >= 8) || 'Heslo musí mít aspoň 8 znaků',
	// (value: string) => (value.length <= 32) || 'Heslo musí mít maximálně 32 znaků',
	// (value: string) => /\d/.test(value) || 'Heslo musí obsahovat číslici',
	// (value: string) => /[a-zA-Z]/.test(value) || 'Heslo musí obsahovat písmeno'
];

const login = async () => {
	const valid = await loginForm.value.validate();

	if (!valid) {
		window.alert('Formulář není správně vyplněn!');
		return;
	}

	const loggedIn = await userStore.login(username.value, password.value);
	console.log('loggedIn', loggedIn);

	if (loggedIn) {
		loginForm.value.reset();
		await router.push({name: 'Home'});
	}
};
</script>

<template>
	<v-sheet class="mx-auto center" width="300">
		<v-form ref="loginForm">
			<v-text-field
				v-model="username"
				label="Uživatelské jméno"
				:rules="nameRules"
				required
			></v-text-field>
			<v-text-field
				v-model="password"
				label="Heslo"
				:rules="passwordRules"
				type="password"
				required
			></v-text-field>
			<v-btn
				class="mt-4"
				color="success"
				block
				@click="login()"
			>
				Přihlásit
			</v-btn>
		</v-form>
	</v-sheet>
</template>

<style scoped>
.center {
	transform: translateY(calc(50vh - 100%));
}
</style>