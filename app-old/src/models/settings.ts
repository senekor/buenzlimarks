import { z } from "zod";

export const settings = {
  schema: z.object({
    name: z.string(),
  }),
};
