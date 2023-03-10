import { createEffect, createMemo, createResource } from "solid-js";
import { z } from "zod";
import { bearerToken } from "./auth";
import { Bookmark, BookmarkSchema } from "./models";

export function reqInit(
	method: "GET" | "POST" | "PUT" | "DELETE",
	data?: unknown,
): RequestInit {
	return {
		method,
		headers: {
			Authorization: `Bearer ${bearerToken()}`,
			...(data ? { "Content-Type": "application/json" } : {}),
		},
		...(data ? { body: JSON.stringify(data) } : {}),
	};
}

const [bookmarks, { refetch: refetchBookmarks, mutate: mutateBookmarks }] =
	createResource(bearerToken, async (token) => {
		const resp = await fetch("api/bookmarks", reqInit("GET"));
		return z.array(BookmarkSchema).parse(await resp.json());
	});
createEffect(() => bearerToken() || mutateBookmarks(undefined));
export { bookmarks };

export function useCreateBookmark(onSuccess: () => void) {
	return async (data: Bookmark) => {
		const resp = await fetch("api/bookmarks", reqInit("POST", data));
		refetchBookmarks();
		onSuccess();
		return BookmarkSchema.parse(await resp.json());
	};
}

export function useUpdateBookmark(onSuccess: () => void) {
	return async (data: Bookmark) => {
		const resp = await fetch(`api/bookmarks/${data.id}`, reqInit("PUT", data));
		refetchBookmarks();
		onSuccess();
		return BookmarkSchema.parse(await resp.json());
	};
}

export function useDeleteBookmark(onSuccess: () => void) {
	return async (id: string) => {
		await fetch(`api/bookmarks/${id}`, reqInit("DELETE"));
		refetchBookmarks();
		onSuccess();
	};
}
