import { z } from "zod";

export const bookmark = {
  schema: z.object({
    id: z.string(),
    name: z.string(),
    url: z.string(),
    widgetId: z.string(),
  }),
  plural: "bookmarks",
};
