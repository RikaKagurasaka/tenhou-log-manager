import { createApp } from "vue";
import App from "./App.vue";
import { createPinia } from "pinia";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";
import { router } from "./router";
import { i18n } from "./i18n";
import "./style.css";

const app = createApp(App);
const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);
app.use(i18n);
app.use(router);
app.use(pinia);
app.mount("#app");
