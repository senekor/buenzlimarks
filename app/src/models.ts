import { z } from "zod";

export const UserSchema = z.object({
	id: z.string(),
	name: z.union([z.string(), z.null()]),
});
export type User = z.infer<typeof UserSchema>;

export const BookmarkSchema = z.object({
	id: z.string(),
	name: z.string(),
	url: z.string(),
	widget_id: z.string(),
});
export type Bookmark = z.infer<typeof BookmarkSchema>;
