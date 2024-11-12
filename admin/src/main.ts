import 'vuetify/styles';
import { createVuetify } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';
import { createApp } from 'vue';
import { createPinia } from 'pinia';

import App from './App.vue';
import { router } from './router/router';
import { ApiSDK } from './sdk/api';

const app = createApp(App);

const vuetify = createVuetify({
	components,
	directives
});

const pinia = createPinia();

const api = new ApiSDK();

app.use(vuetify);
app.use(router);
app.use(pinia);

app.provide('ApiSDK', api);

app.mount('#app');

// createApp(App).mount('#app')