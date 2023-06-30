import { z } from "zod";

export const bookmark = {
  schema: z
    .object({
      id: z.string(),
      name: z.string(),
      url: z.string(),
      widget_id: z.string(),
    })
    .transform(({ widget_id, ...rest }) => ({ widgetId: widget_id, ...rest })),
  plural: "bookmarks",
};
