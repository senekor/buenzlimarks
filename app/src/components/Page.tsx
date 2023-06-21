import { useEntities } from "../api/hooks";
import { Widget } from "./Widget";
import { Page as PageType } from "../models";

export function Page({ page: { id, name } }: { page: PageType }) {
  const { data: widgets } = useEntities(["widget", { page_id: id }]);
  return (
    <div className="bg-slate-600 p-2 flex gap-2">
      <h2>{name}</h2>
      {widgets?.map((w) => (
        <div>
          <Widget key={w.id} widget={w} />
        </div>
      ))}
    </div>
  );
}
