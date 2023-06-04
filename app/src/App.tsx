import { PencilIcon, TrashIcon } from "@heroicons/react/24/solid";

import Bookmark from "./components/Bookmark";
import { Bookmark as BookmarkType, Ntt } from "./models";
import { useAuth } from "./auth/context";
import { useMutation, useQuery } from "@tanstack/react-query";
import { useEffect, useState } from "react";
import { useApi } from "./api";

const bookmarkTmpl: BookmarkType = {
  id: "",
  name: "",
  url: "",
  widgetId: "",
};

function UserForm() {
  const [userId, setUserId] = useState("");

  const { login } = useAuth();
  const submit = () => login(userId);

  return (
    <div className="flex flex-row self-center gap-2">
      <input
        className="self-center bg-slate-600 p-1 rounded text-white"
        placeholder="Enter a user name"
        value={userId}
        onInput={(e) => setUserId(e.currentTarget.value)}
        onKeyDown={(e) => (e.key === "Enter" ? submit() : null)}
      />
      <button
        className="text-white bg-slate-600 w-fit rounded px-1 disabled:text-gray-400"
        disabled={!userId}
        onClick={submit}
      >
        Login
      </button>
    </div>
  );
}

function App() {
  const { logout } = useAuth();

  const api = useApi();
  const { data: user } = useQuery(["user"], () => api.get(Ntt.User, "me"));

  // hacky stuff to make sure a widget exists.
  // remove once UI for creating pages and widgets exists.
  const [widgetId, setWidgetId] = useState("");
  useQuery(["widgets"], async () => {
    const widgets = await api.getAll(Ntt.Widget);
    if (widgets.length > 0) {
      setWidgetId(widgets[0].id);
    } else {
      const page = await api.post(Ntt.Page, { id: "" });
      const widget = await api.post(Ntt.Widget, { id: "", pageId: page.id });
      setWidgetId(widget.id);
    }
  });

  const { data: bookmarks } = useQuery(["bookmarks"], () =>
    api.getAll(Ntt.Bookmark)
  );

  const [form, setForm] = useState<BookmarkType>(bookmarkTmpl);
  const resetForm = () =>
    setForm({
      ...bookmarkTmpl,
      widgetId: widgetId,
    });

  // hack to add bookmarks to valid widget
  useEffect(() => {
    setForm((oldForm) => ({ ...oldForm, widgetId: widgetId }));
  }, [widgetId]);

  const { mutate: createBookmark } = useMutation(
    (it: BookmarkType) => api.post(Ntt.Bookmark, it),
    { onSuccess: () => resetForm }
  );
  const { mutate: updateBookmark } = useMutation(
    (it: BookmarkType) => api.put(Ntt.Bookmark, it),
    { onSuccess: () => resetForm }
  );
  const { mutate: deleteBookmark } = useMutation(
    (it: BookmarkType) => api.del(Ntt.Bookmark, it.id),
    { onSuccess: () => resetForm }
  );

  const submitBookmark = () =>
    form.id ? updateBookmark(form) : createBookmark(form);

  return (
    <div className="flex flex-col bg-slate-800 h-screen">
      <p className="text-4xl text-orange-500 text-center mt-12 mb-8">
        buenzlimarks
      </p>
      {user ? (
        <div className="flex flex-row self-center gap-2">
          <div className="text-gray-200">Hello {user?.name}!</div>
          <button
            className="text-white bg-slate-600 w-fit rounded px-1"
            onClick={logout}
          >
            Logout
          </button>
        </div>
      ) : (
        <UserForm />
      )}
      <div className="self-center flex flex-col gap-1 my-4">
        {bookmarks?.map((bm: BookmarkType) => (
          <div className="flex w-full gap-1">
            <div className="flex-grow" />
            <Bookmark title={bm.name} link={bm.url} />
            <div className="flex-grow" />
            {/* <Icon
								path={pencil}
								className="w-6 ml-2"
								style={{ color: "white" }}
								onClick={() => setForm(bm)}
							/> */}
            <TrashIcon
              className="w-6"
              style={{ color: "white" }}
              onClick={() => deleteBookmark(bm)}
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
          onClick={submitBookmark}
        >
          {!form.id ? "Add" : "Save"}
        </button>
      </div>
    </div>
  );
}

export default App;
