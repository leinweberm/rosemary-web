<script setup lang="ts">
import { useUserStore } from './stores/userStore';

const userStore = useUserStore();
</script>

<template>
	<v-responsive class="border rounded">
		<v-app>
			<v-app-bar title="rosemary-artist.com">
			</v-app-bar>
			<v-navigation-drawer
				v-if="userStore.getUser"
				:absolute="true"
				:width="300"
			>
				<v-list>
					<v-list-item
						title="Obrazy"
						style="background: lightgray;"
					></v-list-item>
					<v-divider></v-divider>
					<v-list-item
						title="Zobrazit"
						link
					></v-list-item>
					<v-list-item
						title="Vytvorit"
						link
					></v-list-item>
				</v-list>
			</v-navigation-drawer>
			<v-main>
				<v-container>
					<router-view></router-view>
				</v-container>
			</v-main>
		</v-app>
	</v-responsive>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>