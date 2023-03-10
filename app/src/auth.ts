import {
	createEffect,
	createMemo,
	createResource,
	createSignal,
} from "solid-js";
import { reqInit } from "./api";
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

const [user, { mutate: mutateUser }] = createResource(bearerToken, async () => {
	const resp = await fetch("/api/users/me", reqInit("GET"));
	return UserSchema.parse(await resp.json());
});
createEffect(() => bearerToken() || mutateUser(undefined));
export { user };
