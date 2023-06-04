import { z } from "zod";

export enum Ntt {
  Bookmark,
  Page,
  User,
  Widget,
}

export const impl = {
  [Ntt.Bookmark]: {
    schema: z.object({
      id: z.string(),
      name: z.string(),
      url: z.string(),
      widgetId: z.string(),
    }),
    plural: "bookmarks",
  },
  [Ntt.Page]: {
    schema: z.object({
      id: z.string(),
    }),
    plural: "pages",
  },
  [Ntt.User]: {
    schema: z.object({
      id: z.string(),
      name: z.union([z.string(), z.null()]),
    }),
    plural: "users",
  },
  [Ntt.Widget]: {
    schema: z.object({
      id: z.string(),
      pageId: z.string(),
    }),
    plural: "widgets",
  },
};

export type Typeof<N extends Ntt> = z.infer<(typeof impl)[N]["schema"]>;

export type Bookmark = Typeof<Ntt.Bookmark>;
export type Page = Typeof<Ntt.Page>;
export type User = Typeof<Ntt.User>;
export type Widget = Typeof<Ntt.Widget>;
