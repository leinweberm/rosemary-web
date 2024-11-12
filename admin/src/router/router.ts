import { createRouter, createWebHistory } from "vue-router";
import { useUserStore } from "../stores/userStore";

const routes = [
	{
		name: 'Home',
		path: '/',
		// @ts-ignore
		component: () => import('../views/Home.vue')
	},
	{
		name: 'Login',
		path: '/login',
		// @ts-ignore
		component: () => import('../views/Login.vue')
	}
];

export enum routesOpts {
	Home = 'Home',
	Login = 'Login'
};

export const router = createRouter({
	routes,
	history: createWebHistory()
});

router.beforeEach((to, _from, next) => {
	const userStore = useUserStore();
	if (to.name !== 'Login' && !userStore.getUser) {
		next({name: 'Login'})
	} else {
		next();
	}
});