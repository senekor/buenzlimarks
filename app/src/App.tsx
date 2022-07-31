import { Component, createResource, createSignal, For } from "solid-js";
import Bookmark from "./components/bookmark";

type BookmarkType = {
  name: string;
  url: string;
};

const PORT = import.meta.env.DEV ? "3000" : "4000";

const fetchBookmarks = async (userId: string): Promise<BookmarkType[]> =>
  (await fetch(`http://localhost:${PORT}/api/${userId || "z"}`)).json();

export default function App() {
  const [userId, setUserId] = createSignal("");
  const [bookmarks] = createResource(userId, fetchBookmarks);

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
    </div>
  );
}
