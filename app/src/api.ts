import { createEffect, createMemo, createResource } from "solid-js";
import { z } from "zod";
import { bearerToken } from "./auth";
import { Bookmark, BookmarkSchema } from "./models";

const [bookmarks, { refetch: refetchBookmarks, mutate: mutateBookmarks }] =
	createResource(bearerToken, async (token) => {
		const resp = await fetch("/api/bookmarks", {
			headers: { Authorization: `Bearer ${token}` },
		});
		const data = await resp.json();
		return z.array(BookmarkSchema).parse(data);
	});
createEffect(() => bearerToken() || mutateBookmarks(undefined));
export { bookmarks };

export const useCreateBookmark = createMemo(() => {
	return (onSuccess: () => void) => async (payload: Bookmark) => {
		const resp = await fetch(`/api/bookmarks/${payload.id}`, {
			method: "POST",
			headers: {
				Authorization: `Bearer ${bearerToken()}`,
				"Content-Type": "application/json",
			},
			body: JSON.stringify(payload),
		});
		refetchBookmarks();
		onSuccess();
		const data = await resp.json();
		return BookmarkSchema.parse(data);
	};
});

export const useUpdateBookmark = createMemo(() => {
	return (onSuccess: () => void) => async (payload: Bookmark) => {
		const resp = await fetch(`/api/bookmarks/${payload.id}`, {
			method: "PUT",
			headers: {
				Authorization: `Bearer ${bearerToken()}`,
				"Content-Type": "application/json",
			},
			body: JSON.stringify(payload),
		});
		refetchBookmarks();
		onSuccess();
		const data = await resp.json();
		return BookmarkSchema.parse(data);
	};
});

export const useDeleteBookmark = createMemo(() => {
	return (onSuccess: () => void) => async (id: string) => {
		fetch(`/api/bookmarks/${id}`, {
			method: "DELETE",
			headers: { Authorization: `Bearer ${bearerToken()}` },
		});
		refetchBookmarks();
		onSuccess();
	};
});
