import { createRouter, createWebHistory } from "vue-router";
import { useUserStore } from "../stores/userStore";

export enum routesOpts {
	Home = 'Home',
	Login = 'Login',
	P_CREATE = 'PaintingCreate',
}

const routes = [
	{
		name: routesOpts.Home,
		path: '/',
		component: () => import('../views/Home.vue')
	},
	{
		name: routesOpts.Login,
		path: '/login',
		component: () => import('../views/Login.vue')
	},
	{
		name: routesOpts.P_CREATE,
		path: '/paintings/+',
		component: () => import('../views/Paintings/Entry.vue')
	}
];

export const router = createRouter({
	routes,
	history: createWebHistory()
});

router.beforeEach(async (to, _from, next) => {
	const userStore = useUserStore();
	const validAccess = await userStore.authRouteAccess();
	console.log('validAccess', validAccess);
	if (to.name !== 'Login' && !validAccess) {
		sessionStorage.clear();
		next({name: routesOpts.Login})
	} else {
		next();
	}
});