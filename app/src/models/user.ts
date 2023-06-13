import { z } from "zod";

export const user = {
  schema: z.object({
    id: z.string(),
    name: z.union([z.string(), z.null()]),
  }),
  plural: "users",
};
