import {
  Component,
  createEffect,
  createResource,
  createSignal,
  For,
} from "solid-js";
import Bookmark from "./components/bookmark";
import { Icon } from "solid-heroicons";
import { pencilAlt, trash } from "solid-heroicons/outline";

type BookmarkType = {
  id: string;
  name: string;
  url: string;
};

const getURL = (userId: string) =>
  `/api/bookmarks/${userId || "nobody"}`;

const fetchBookmarks = async (userId: string): Promise<BookmarkType[]> =>
  (await fetch(getURL(userId))).json();

const submitBookmark = async (userId: string, payload: BookmarkType) =>
  await fetch(getURL(userId), {
    method: !payload.id ? "POST" : "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });

const deleteBookmark = async (userId: string, payload: BookmarkType) =>
  await fetch(getURL(userId), {
    method: "DELETE",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });

const bookmarkTmpl: BookmarkType = {
  id: "",
  name: "",
  url: "",
};

export default function App() {
  const [userId, setUserId] = createSignal("");
  const [bookmarks, { refetch }] = createResource(userId, fetchBookmarks);

  const [form, setForm] = createSignal<BookmarkType>(bookmarkTmpl);

  createEffect(() => {
    userId();
    setForm(bookmarkTmpl);
  });

  return (
    <div class="flex flex-col bg-slate-800 h-screen">
      <p class="text-4xl text-orange-500 text-center mt-12 mb-8">
        BuenzliMarks
      </p>
      <input
        class="self-center w-3/4 bg-slate-600 p-1 rounded text-white"
        placeholder="Enter a user name"
        value={userId()}
        onInput={(e) => setUserId((e.target as HTMLInputElement).value)}
      />
      <div class="flex flex-col gap-1 my-4">
        <For each={bookmarks()}>
          {(bm, i) => (
            <div style={{ color: "white" }} class="flex self-center gap-1">
              <Bookmark title={bm.name} link={bm.url} />
              <Icon
                path={pencilAlt}
                class="w-6 ml-2"
                onClick={() =>
                  form().id === bm.id ? setForm(bookmarkTmpl) : setForm(bm)
                }
              />
              <Icon
                path={trash}
                class="w-6"
                onClick={() => {
                  deleteBookmark(userId(), bm);
                  refetch();
                }}
              />
            </div>
          )}
        </For>
      </div>
      <input
        class="self-center w-3/4 bg-slate-600 p-1 rounded text-white mb-1"
        placeholder="Name"
        value={form().name}
        onInput={(e) =>
          setForm({ ...form(), name: (e.target as HTMLInputElement).value })
        }
      />
      <input
        class="self-center w-3/4 bg-slate-600 p-1 rounded text-white mb-1"
        placeholder="URL"
        value={form().url}
        onInput={(e) =>
          setForm({ ...form(), url: (e.target as HTMLInputElement).value })
        }
      />
      <button
        class="text-white bg-slate-600 w-fit self-center rounded px-1"
        disabled={!(form().name || form().url)}
        onClick={() => {
          submitBookmark(userId(), form());
          refetch();
        }}
      >
        {!form().id ? "Add" : "Save"}
      </button>
    </div>
  );
}
