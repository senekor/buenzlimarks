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

export function Widget({
  widget: { id, name, pageId },
}: {
  widget: WidgetType;
}) {
  const { data: bookmarks } = useEntities(["bookmark", { widget_id: id }]);

  const { mutate: deleteWidget } = useDeleteEntity("widget");
  const { mutate: submitWidget } = useSubmitEntity("widget");
  const { mutate: submitBookmark } = useSubmitEntity("bookmark");
  const { mutate: deleteBookmark } = useDeleteEntity("bookmark");

  const [nameForm, setNameForm] = useState<string>();

  const [bookmarkForm, setBookmarkForm] = useState(bookmarkTmpl(id));
  const resetForm = useCallback(() => setBookmarkForm(bookmarkTmpl(id)), [id]);

  return (
    <div className="bg-slate-700 flex flex-col p-4 rounded-lg">
      <div className="flex flex-row  gap-1">
        {nameForm === undefined ? (
          <h2 className="text-3xl pb-2">{name}</h2>
        ) : (
          <input
            className=" bg-slate-600 p-1 rounded text-white mb-3"
            placeholder="Widget name"
            value={nameForm}
            onInput={(e) => setNameForm(e.currentTarget.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                submitWidget(
                  { id, name: nameForm, pageId },
                  { onSuccess: () => setNameForm(undefined) }
                );
              }
            }}
          />
        )}
        <FlexSpace />
        <PencilSquareIcon
          className="w-6 mb-2 ml-2"
          onClick={() => setNameForm(name)}
        />
        <XMarkIcon
          className="w-6 mb-2"
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
            onClick={() => setBookmarkForm(b)}
          />
          <XMarkIcon className="w-6" onClick={() => deleteBookmark(b.id)} />
        </div>
      ))}
      <input
        className="self-center w-full bg-slate-600 p-1 rounded text-white mb-1 mt-2"
        placeholder="Name"
        value={bookmarkForm.name}
        onInput={(e) =>
          setBookmarkForm({ ...bookmarkForm, name: e.currentTarget.value })
        }
      />
      <input
        className="self-center w-full bg-slate-600 p-1 rounded text-white mb-2"
        placeholder="URL"
        value={bookmarkForm.url}
        onInput={(e) =>
          setBookmarkForm({ ...bookmarkForm, url: e.currentTarget.value })
        }
      />
      <div className="self-center flex gap-2">
        {bookmarkForm.id && (
          <button
            className="text-white bg-slate-600 w-fit rounded px-1"
            onClick={resetForm}
          >
            Cancel
          </button>
        )}
        <button
          className="text-white bg-slate-600 w-fit rounded px-1 disabled:text-gray-400"
          disabled={!(bookmarkForm.name && bookmarkForm.url)}
          onClick={() => submitBookmark(bookmarkForm, { onSuccess: resetForm })}
        >
          {!bookmarkForm.id ? "Add" : "Save"}
        </button>
      </div>
    </div>
  );
}
