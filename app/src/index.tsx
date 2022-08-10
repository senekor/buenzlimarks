/* @refresh reload */
// eslint-disable-next-line import/order
import "./index.css";

import { render } from "solid-js/web";

import { App } from "./App";

render(() => <App />, document.getElementById("root") as HTMLElement);
