/* @refresh reload */
import "./index.css";

import { render } from "solid-js/web";

import { App } from "./App";
import { AuthProvider } from "./auth";
import { ApiProvider } from "./api";

render(
	() => (
		<AuthProvider>
			<ApiProvider>
				<App />
			</ApiProvider>
		</AuthProvider>
	),
	document.getElementById("root") as HTMLElement,
);
