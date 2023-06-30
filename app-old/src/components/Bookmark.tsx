import { Bookmark as BookmarkType } from "../models";

export function Bookmark({
  bookmark: { name, url },
}: {
  bookmark: BookmarkType;
}) {
  return (
    <a
      className="text-2xl text-orange-200 hover:text-orange-400 underline"
      href={url}
    >
      {name}
    </a>
  );
}
