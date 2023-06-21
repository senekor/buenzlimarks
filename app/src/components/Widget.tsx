import { useEntities } from "../api/hooks";
import { Bookmark } from "./Bookmark";
import { Widget as WidgetType } from "../models";

export function Widget({ widget: { id, name } }: { widget: WidgetType }) {
  const { data: bookmarks } = useEntities(["bookmark", { widget_id: id }]);
  return (
    <div className="bg-slate-700 p-2">
      <h2>{name}</h2>
      {bookmarks?.map((b) => (
        <div>
          <Bookmark key={b.id} bookmark={b} />
        </div>
      ))}
    </div>
  );
}
