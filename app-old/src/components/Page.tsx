import { useEntities, useSubmitEntity } from "../api/hooks";
import { Widget } from "./Widget";
import { Page as PageType } from "../models";
import { PlusIcon } from "@heroicons/react/24/outline";

export function Page({ page: { id } }: { page: PageType }) {
  const { data: widgets } = useEntities(["widget", { page_id: id }]);

  const { mutate: submitWidget } = useSubmitEntity("widget");

  return (
    <div className="flex flex-col gap-2 ite">
      {widgets?.map((w) => (
        <Widget key={w.id} widget={w} />
      ))}
      <div
        className="bg-slate-600 rounded-full p-1.5 w-8 self-center mt-2"
        onClick={() => submitWidget({ id: "", name: "new widget", pageId: id })}
      >
        <PlusIcon className="w-5" />
      </div>
    </div>
  );
}
