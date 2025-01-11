<script setup lang="ts">
import { useUserStore } from './stores/userStore';
import {routesOpts} from "./router/router.ts";
import {useRouter} from "vue-router";
import {useRoute} from "vue-router";
import {computed} from "vue";

const userStore = useUserStore();
const router = useRouter();
const route = useRoute();
const mainBackground = computed(() => {
	const value = (route.name === routesOpts.Login)  ? 'rgb(255, 255, 255)' : 'rgb(250, 250, 250)';
	return `background: ${value}`;
});
</script>

<template>
	<v-responsive class="border rounded">
		<v-app>
			<v-app-bar
				v-if="userStore.getUser"
				title="admin.rosemary-artist.com"
				style="cursor: pointer"
				@click.stop="router.push({name: routesOpts.Home})"
			>
				<v-btn
					type="button"
					color="error"
					size="small"
					variant="tonal"
					style="margin-right: 20px !important;"
					@click.stop="userStore.logout()"
				>
					Odhlásit
				</v-btn>
			</v-app-bar>
			<v-navigation-drawer
				v-if="userStore.getUser"
				class="elevation-3"
				:absolute="true"
				:width="300"
			>
				<v-list>
					<v-list-item
						title="Obrazy"
						style="font-weight: 800; background: rgba(250, 250, 250)"
					></v-list-item>
					<v-divider></v-divider>
					<v-list-item
						title="Zobrazit vše"
						style="color: gray;"
						link
						@click.stop="router.push({name: routesOpts.P_LIST})"
					></v-list-item>
					<v-list-item
						title="Vytvořit"
						style="color: gray;"
						link
						@click.stop="router.push({name: routesOpts.P_CREATE})"
					></v-list-item>
				</v-list>
			</v-navigation-drawer>
			<v-main
				:style="`${mainBackground}`"
			>
				<v-container>
					<router-view></router-view>
				</v-container>
			</v-main>
		</v-app>
	</v-responsive>
</template>