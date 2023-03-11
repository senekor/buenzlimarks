import {
	createContext,
	createEffect,
	createSignal,
	JSXElement,
	useContext,
} from "solid-js";

function uninit(): never {
	throw new Error("auth is uninitialized");
}

type AuthContextData = {
	token: () => string | undefined;
	login: (id: string) => void;
	logout: () => void;
};

const AuthContext = createContext<AuthContextData>({
	token: uninit,
	login: uninit,
	logout: uninit,
});
export const useAuth = () => useContext(AuthContext);

export function AuthProvider(props: { children: JSXElement }) {
	const [token, setToken] = createSignal<string>();

	async function login(id: string) {
		const resp = await fetch(`/api/auth/login/${id}`);
		setToken(await resp.text());
	}
	const logout = () => setToken();

	return (
		<AuthContext.Provider value={{ token, login, logout }}>
			{props.children}
		</AuthContext.Provider>
	);
}
