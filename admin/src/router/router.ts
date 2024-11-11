import { createRouter, createWebHistory } from "vue-router";

const routes = [
	{
		name: 'Home',
		path: '/',
		component: () => import('../views/Home.vue')
	},
	{
		name: 'Login',
		path: '/login',
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