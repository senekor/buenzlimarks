import { PencilIcon, TrashIcon } from "@heroicons/react/24/solid";

import { Bookmark as BookmarkComp } from "./components/Bookmark";
import { Bookmark } from "./models";
import { useEffect, useState } from "react";
import { LogoutButton } from "./auth/LogoutButton";
import { useDeleteEntity, useEntities, useSubmitEntity } from "./api/hooks";
import { FlexSpace } from "./components/FlexSpace";

const bookmarkTmpl: Bookmark = {
  id: "",
  name: "",
  url: "",
  widgetId: "",
};

export function App() {
  // hacky stuff to make sure a widget exists.
  // remove once UI for creating pages and widgets exists.
  const [widgetId, setWidgetId] = useState("");
  const { data: widgets } = useEntities("widget");
  const { mutate: submitPage } = useSubmitEntity("page");
  const { mutate: submitWidget } = useSubmitEntity("widget");
  useEffect(() => {
    if (widgets && widgets.length > 0) {
      setWidgetId(widgets[0].id);
    } else if (widgets) {
      submitPage(
        { id: "", name: "" },
        { onSuccess: (page) => submitWidget({ id: "", name: "", pageId: page.id }) }
      );
    }
  }, [submitPage, submitWidget, widgets]);

  const { data: bookmarks } = useEntities("bookmark");

  const [form, setForm] = useState<Bookmark>(bookmarkTmpl);
  const resetForm = () =>
    setForm({
      ...bookmarkTmpl,
      widgetId: widgetId,
    });

  // hack to add bookmarks to valid widget
  useEffect(() => {
    setForm((oldForm) => ({ ...oldForm, widgetId: widgetId }));
  }, [widgetId]);

  const options = { onSuccess: resetForm };
  const { mutate: submitBookmark } = useSubmitEntity("bookmark", options);
  const { mutate: deleteBookmark } = useDeleteEntity("bookmark", options);

  return (
    <div className="flex flex-col h-screen">
      <h1 className="text-4xl text-orange-500 text-center mt-12 mb-8">
        buenzlimarks
      </h1>
      <LogoutButton />
      <div className="self-center flex flex-col gap-1 my-4">
        {bookmarks?.map((bm: Bookmark) => (
          <div key={bm.id} className="flex w-full gap-1">
            <FlexSpace />
            <BookmarkComp title={bm.name} link={bm.url} />
            <FlexSpace />
            <PencilIcon
              className="w-6 ml-2"
              style={{ color: "white" }}
              onClick={() => setForm(bm)}
            />
            <TrashIcon
              className="w-6"
              style={{ color: "white" }}
              onClick={() => deleteBookmark(bm.id)}
            />
          </div>
        ))}
      </div>
      <input
        className="self-center w-3/4 bg-slate-600 p-1 rounded text-white mb-1"
        placeholder="Name"
        value={form.name}
        onInput={(e) => setForm({ ...form, name: e.currentTarget.value })}
      />
      <input
        className="self-center w-3/4 bg-slate-600 p-1 rounded text-white mb-2"
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
          onClick={() => submitBookmark(form)}
        >
          {!form.id ? "Add" : "Save"}
        </button>
      </div>
    </div>
  );
}
