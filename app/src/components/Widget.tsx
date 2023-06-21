import { useDeleteEntity, useEntities, useSubmitEntity } from "../api/hooks";
import { Bookmark } from "./Bookmark";
import { Widget as WidgetType, Bookmark as BookmarkType } from "../models";
import { FlexSpace } from "./FlexSpace";
import { PencilSquareIcon, XMarkIcon } from "@heroicons/react/24/outline";
import { useCallback, useState } from "react";

function bookmarkTmpl(widgetId: string): BookmarkType {
  return {
    id: "",
    name: "",
    url: "",
    widgetId,
  };
}

export function Widget({ widget: { id, name } }: { widget: WidgetType }) {
  const { data: bookmarks } = useEntities(["bookmark", { widget_id: id }]);

  const { mutate: deleteWidget } = useDeleteEntity("widget");
  const { mutate: submitBookmark } = useSubmitEntity("bookmark");
  const { mutate: deleteBookmark } = useDeleteEntity("bookmark");

  const [form, setForm] = useState(bookmarkTmpl(id));
  const resetForm = useCallback(() => setForm(bookmarkTmpl(id)), [id]);

  return (
    <div className="bg-slate-700 flex flex-col p-4 rounded-lg">
      <div className="flex flex-row justify-between">
        <h2 className="text-3xl pb-2">{name}</h2>
        <XMarkIcon
          className="w-6 ml-4 mb-2"
          onClick={(e) => {
            deleteWidget(id);
            e.stopPropagation();
          }}
        />
      </div>
      {bookmarks?.map((b) => (
        <div key={b.id} className="flex w-full gap-1">
          <FlexSpace />
          <Bookmark bookmark={b} />
          <FlexSpace />
          <PencilSquareIcon
            className="w-6 ml-2"
            style={{ color: "white" }}
            onClick={() => setForm(b)}
          />
          <XMarkIcon
            className="w-6"
            style={{ color: "white" }}
            onClick={() => deleteBookmark(b.id)}
          />
        </div>
      ))}
      <input
        className="self-center w-full bg-slate-600 p-1 rounded text-white mb-1 mt-2"
        placeholder="Name"
        value={form.name}
        onInput={(e) => setForm({ ...form, name: e.currentTarget.value })}
      />
      <input
        className="self-center w-full bg-slate-600 p-1 rounded text-white mb-2"
        placeholder="URL"
        value={form.url}
        onInput={(e) => setForm({ ...form, url: e.currentTarget.value })}
      />
      <div className="self-center flex gap-2">
        {form.id && (
          <button
            className="text-white bg-slate-600 w-fit rounded px-1"
            onClick={resetForm}
          >
            Cancel
          </button>
        )}
        <button
          className="text-white bg-slate-600 w-fit rounded px-1 disabled:text-gray-400"
          disabled={!(form.name && form.url)}
          onClick={() => submitBookmark(form, { onSuccess: resetForm })}
        >
          {!form.id ? "Add" : "Save"}
        </button>
      </div>
    </div>
  );
}
