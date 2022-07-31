import { Component, createResource, createSignal, For } from "solid-js";
import Bookmark from "./components/bookmark";

type BookmarkType = {
  id: number;
  name: string;
  url: string;
};

const PORT = import.meta.env.DEV ? "3000" : "4000";

const fetchBookmarks = async (userId: string): Promise<BookmarkType[]> =>
  (await fetch(`http://localhost:${PORT}/api/${userId || "nobody"}`)).json();

const postBookmarks = async (userId: string, payload: BookmarkType) =>
  await fetch(`http://localhost:${PORT}/api/${userId || "nobody"}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(payload),
  });

const bookmarkTmpl: BookmarkType = {
  id: 0,
  name: "",
  url: "",
};

export default function App() {
  const [userId, setUserId] = createSignal("");
  const [bookmarks, { refetch }] = createResource(userId, fetchBookmarks);

  const [form, setForm] = createSignal<BookmarkType>(bookmarkTmpl);

  return (
    <div class="flex flex-col bg-slate-800 h-screen">
      <p class="text-4xl text-orange-500 text-center mt-12 mb-8">
        BuenzliMarks
      </p>
      <input
        class="self-center w-fit bg-slate-600 p-1 rounded text-white"
        placeholder="Enter a user name"
        value={userId()}
        onInput={(e) => setUserId((e.target as HTMLInputElement).value)}
      />
      <div class="flex flex-col gap-1 my-4">
        <For each={bookmarks()}>
          {(bookmark, i) => (
            <Bookmark title={bookmark.name} link={bookmark.url} />
          )}
        </For>
      </div>
      <input
        class="self-center w-fit bg-slate-600 p-1 rounded text-white mb-1"
        placeholder="Name"
        value={form().name}
        onInput={(e) =>
          setForm({ ...form(), name: (e.target as HTMLInputElement).value })
        }
      />
      <input
        class="self-center w-fit bg-slate-600 p-1 rounded text-white mb-1"
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
          postBookmarks(userId(), form());
          refetch();
        }}
      >
        Add
      </button>
    </div>
  );
}
