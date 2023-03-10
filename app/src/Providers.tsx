import { JSX } from "solid-js/jsx-runtime";

type Props = {
	children: JSX.Element;
};
export function Providers(p: Props) {
	return <>{p.children}</>;
}
