import { z } from "zod";

export const page = {
  schema: z.object({
    id: z.string(),
    name: z.string(),
  }),
  plural: "pages",
};
