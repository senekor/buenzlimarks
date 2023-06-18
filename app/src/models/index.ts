import { z } from "zod";
import { bookmark } from "./bookmark";
import { widget } from "./widget";
import { page } from "./page";
import { user } from "./user";
import { settings } from "./settings";

// ################################################################
// definitions

const entities = {
  user,
  bookmark,
  page,
  widget,
};

// ################################################################
// type aliases

type Entities = typeof entities;
export type EntityKey = keyof Entities;
export type Entity<K extends EntityKey> = z.infer<Entities[K]["schema"]>;

export type Bookmark = Entity<"bookmark">;
export type Page = Entity<"page">;
export type User = Entity<"user">;
export type Widget = Entity<"widget">;

export type Settings = z.infer<typeof settings["schema"]>;

// ################################################################
// generic functionality

export function schema<K extends EntityKey>(k: K): Entities[K]["schema"] {
  return entities[k].schema;
}
export function parse<K extends EntityKey>(k: K): (data: unknown) => Entity<K> {
  return schema(k).parse;
}

export function plural<K extends EntityKey>(k: K): string {
  return entities[k].plural;
}
