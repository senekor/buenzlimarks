/* @refresh reload */
import "./index.css";

import { render } from "solid-js/web";

import { App } from "./App";
import { Providers } from "./Providers";

render(
	() => (
		<Providers>
			<App />
		</Providers>
	),
	document.getElementById("root") as HTMLElement,
);
