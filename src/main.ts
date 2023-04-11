import "virtual:uno.css";

import {
    createApp
} from "vue";
import {
    setupStore
} from "@/store";
import "./styles.css";
import App from "./App.vue";

async function setupApp() {
    const app = createApp(App);

    setupStore(app);

    app.mount("#app");
}

setupApp();
