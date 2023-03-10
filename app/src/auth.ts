import {
	batch,
	createEffect,
	createMemo,
	createResource,
	createSignal,
} from "solid-js";
import { UserSchema } from "./models";

export const [userId, setUserId] = createSignal<string>();
export const useLogout = createMemo(() => () => setUserId(undefined));

const [bearerToken, { mutate: mutateBearerToken }] = createResource(
	userId,
	async (id) => {
		const resp = await fetch(`/api/auth/login/${id}`);
		return await resp.text();
	},
);
createEffect(() => userId() || mutateBearerToken(undefined));
export { bearerToken };

const [user, { mutate: mutateUser }] = createResource(
	bearerToken,
	async (token) => {
		const resp = await fetch("/api/users/me", {
			headers: { Authorization: `Bearer ${token}` },
		});
		const data = await resp.json();
		return UserSchema.parse(data);
	},
);
createEffect(() => bearerToken() || mutateUser(undefined));
export { user };
