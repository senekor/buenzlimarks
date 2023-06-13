import { z } from "zod";

export const widget = {
  schema: z.object({
    id: z.string(),
    pageId: z.string(),
  }),
  plural: "widgets",
};
