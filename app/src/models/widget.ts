import { z } from "zod";

export const widget = {
  schema: z
    .object({
      id: z.string(),
      page_id: z.string(),
    })
    .transform(({ page_id, ...rest }) => ({ pageId: page_id, ...rest })),
  plural: "widgets",
};
