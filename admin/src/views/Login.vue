<script setup lang="ts">
import { ref } from 'vue';

const username = ref<string>("");
const password = ref<string>("");
const loginForm = ref();

const nameRules = [
	value => !!value || 'Email je povinné pole',
	value => /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/.test(value) || 'Email musí mít platný formát'
];

const passwordRules = [
	value => !!value || 'Heslo je povinne',
	value => (value.length > 8) || 'Heslo musí mít aspoň 8 znaků',
	value => (value.length < 33) || 'Heslo musí mít maximálně 32 znaků',
	value => /\d/.test(value) || 'Heslo musí obsahovat číslici',
	value => /[a-zA-Z]/.test(value) || 'Heslo musí obsahovat písmeno'
];

const login = async () => {
	const valid = await loginForm.value.validate();
	loginForm.value.reset();
};
</script>

<template>
	<v-sheet class="mx-auto center" width="300">
		<v-form ref="loginForm">
			<v-text-field
				v-model="username"
				label="Email"
				:rules="nameRules"
				type="email"
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