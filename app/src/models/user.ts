import { z } from "zod";

export const user = {
  schema: z.object({
    id: z.string(),
    provider: z.enum(["dev", "github"]),
  }),
  plural: "users",
};
