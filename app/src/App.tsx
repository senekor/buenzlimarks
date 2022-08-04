import {
  Component,
  createEffect,
  createResource,
  createSignal,
  For,
  Show,
} from "solid-js";
import Bookmark from "./components/bookmark";
import { Icon } from "solid-heroicons";
import { pencilAlt, trash } from "solid-heroicons/outline";

type BookmarkType = {
  id: number;
  name: string;
  url: string;
};

// const getURL = (userId: string) =>
//   `/api/bookmarks/${userId || "nobody"}`;

const fetchBookmarks = async (): Promise<BookmarkType[]> =>
  fetch("/api/bookmarks").then((resp) => resp.json());

const createBookmark = async (payload: BookmarkType): Promise<BookmarkType> =>
  fetch("/api/bookmarks", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  }).then((resp) => resp.json());

const ubdateBookmark = async (payload: BookmarkType): Promise<BookmarkType> =>
  fetch(`/api/bookmarks/${payload.id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  }).then((resp) => resp.json());

const deleteBookmark = async (id: number): Promise<Response> =>
  fetch(`/api/bookmarks/${id}`, { method: "DELETE" });

const bookmarkTmpl: BookmarkType = {
  id: 0,
  name: "",
  url: "",
};

export default function App() {
  // const [userId, setUserId] = createSignal("");
  const [bookmarks, { refetch }] = createResource(
    createSignal(true)[0],
    fetchBookmarks
  );

  const [form, setForm] = createSignal<BookmarkType>(bookmarkTmpl);
  const resetForm = () => setForm(bookmarkTmpl);

  return (
    <div class="flex flex-col bg-slate-800 h-screen">
      <p class="text-4xl text-orange-500 text-center mt-12 mb-8">
        BuenzliMarks
      </p>
      {/* <input
        class="self-center w-3/4 bg-slate-600 p-1 rounded text-white"
        placeholder="Enter a user name"
        value={userId()}
        onInput={(e) => setUserId((e.target as HTMLInputElement).value)}
      /> */}
      <div class="flex flex-col gap-1 my-4">
        <For each={bookmarks()}>
          {(bm, i) => (
            <div style={{ color: "white" }} class="flex self-center gap-1">
              <Bookmark title={bm.name} link={bm.url} />
              <Icon
                path={pencilAlt}
                class="w-6 ml-2"
                onClick={() => setForm(bm)}
              />
              <Icon
                path={trash}
                class="w-6"
                onClick={() => deleteBookmark(bm.id).then(refetch)}
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
      <div class="self-center flex gap-1">
        <Show when={form().id}>
          <button
            class="text-white bg-slate-600 w-fit rounded px-1"
            onClick={resetForm}
          >
            Cancel
          </button>
        </Show>
        <button
          class="text-white bg-slate-600 w-fit rounded px-1 disabled:text-gray-400"
          disabled={!(form().name && form().url)}
          onClick={() =>
            (form().id ? ubdateBookmark(form()) : createBookmark(form()))
              .then(refetch)
              .then(resetForm)
          }
        >
          {!form().id ? "Add" : "Save"}
        </button>
      </div>
    </div>
  );
}
