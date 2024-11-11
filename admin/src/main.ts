import 'vuetify/styles';
import { createVuetify } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';
import { createApp } from 'vue';
import { createPinia } from 'pinia';

import App from './App.vue';
import { router } from './router/router';

const app = createApp(App);

const vuetify = createVuetify({
	components,
	directives
});

const pinia = createPinia();

app.use(vuetify);
app.use(router);
app.use(pinia);

app.mount('#app');

// createApp(App).mount('#app')