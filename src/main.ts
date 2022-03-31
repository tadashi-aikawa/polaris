import App from "./App.svelte";
import "carbon-components-svelte/css/white.css";
import "~/assets/app.css";

const app = new App({
  target: document.getElementById("app"),
});

export default app;
