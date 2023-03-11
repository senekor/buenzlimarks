import {
	createContext,
	createEffect,
	createResource,
	createSignal,
	JSXElement,
	useContext,
} from "solid-js";
import { z } from "zod";
import { useAuth } from "./auth";
import { Bookmark, BookmarkSchema, User, UserSchema } from "./models";

function uninit(): never {
	throw new Error("api is uninitialized");
}

export type Api = {
	user: () => User | undefined;
	bookmarks: () => Bookmark[];
	widgetId: () => string;
	useCreateBookmark: (onSuccess: () => void) => (bm: Bookmark) => void;
	useUpdateBookmark: (onSuccess: () => void) => (bm: Bookmark) => void;
	useDeleteBookmark: (onSuccess: () => void) => (id: string) => void;
};

const ApiContext = createContext<Api>({
	user: uninit,
	bookmarks: uninit,
	widgetId: uninit,
	useCreateBookmark: uninit,
	useUpdateBookmark: uninit,
	useDeleteBookmark: uninit,
});
export const useUser = () => useContext(ApiContext).user;
export const useBookmarks = () => useContext(ApiContext).bookmarks;
export const useWidgetId = () => useContext(ApiContext).widgetId;

export function useCreateBookmark(onSuccess: () => void) {
	return useContext(ApiContext).useCreateBookmark(onSuccess);
}
export function useUpdateBookmark(onSuccess: () => void) {
	return useContext(ApiContext).useUpdateBookmark(onSuccess);
}
export function useDeleteBookmark(onSuccess: () => void) {
	return useContext(ApiContext).useDeleteBookmark(onSuccess);
}

export function ApiProvider(props: { children: JSXElement }) {
	const { token } = useAuth();

	function request(
		method: "GET" | "POST" | "PUT" | "DELETE",
		data?: unknown,
	): RequestInit {
		return {
			method,
			headers: {
				Authorization: `Bearer ${token()}`,
				...(data ? { "Content-Type": "application/json" } : {}),
			},
			...(data ? { body: JSON.stringify(data) } : {}),
		};
	}

	const [user, { mutate: mutateUser }] = createResource(token, async () => {
		const resp = await fetch("/api/users/me", request("GET"));
		return UserSchema.parse(await resp.json());
	});
	createEffect(() => token() || mutateUser());

	const [widgetId, setWidgetId] = createSignal("");

	const [bookmarks, { refetch: refetchBookmarks, mutate: mutateBookmarks }] =
		createResource(
			token,
			async () => {
				const resp = await fetch("api/bookmarks", request("GET"));
				// return z.array(BookmarkSchema).parse(await resp.json());

				// hacky stuff because there are no "get widgets" / "get bookmarks" endpoints
				// TODO delete when endpoints are ready
				const bookmarks = z.array(BookmarkSchema).parse(await resp.json());
				if (bookmarks.length !== 0) {
					setWidgetId(bookmarks[0].widget_id);
					return bookmarks;
				}
				const pageResp = await fetch("api/pages", request("POST", { id: "" }));
				const page = await pageResp.json();
				const widgetResp = await fetch(
					"api/widgets",
					request("POST", { id: "", page_id: page.id }),
				);
				const widget = await widgetResp.json();
				setWidgetId(z.string().parse(widget.id));
				return bookmarks;
			},
			{ initialValue: [] },
		);
	createEffect(() => token() || mutateBookmarks([]));

	function useCreateBookmark(onSuccess: () => void) {
		return async (data: Bookmark) => {
			const resp = await fetch("api/bookmarks", request("POST", data));
			refetchBookmarks();
			onSuccess();
			return BookmarkSchema.parse(await resp.json());
		};
	}

	function useUpdateBookmark(onSuccess: () => void) {
		return async (data: Bookmark) => {
			const resp = await fetch(
				`api/bookmarks/${data.id}`,
				request("PUT", data),
			);
			refetchBookmarks();
			onSuccess();
			return BookmarkSchema.parse(await resp.json());
		};
	}

	function useDeleteBookmark(onSuccess: () => void) {
		return async (id: string) => {
			await fetch(`api/bookmarks/${id}`, request("DELETE"));
			refetchBookmarks();
			onSuccess();
		};
	}

	return (
		<ApiContext.Provider
			value={{
				user,
				bookmarks,
				widgetId,
				useCreateBookmark,
				useUpdateBookmark,
				useDeleteBookmark,
			}}
		>
			{props.children}
		</ApiContext.Provider>
	);
}
