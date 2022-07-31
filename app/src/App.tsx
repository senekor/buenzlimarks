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
  const [userId, setUserId] = createSignal("remo");
  const [bookmarks] = createResource(userId, fetchBookmarks);

  return (
    <div class="flex flex-col dark:bg-slate-800 h-screen">
      <p class="text-4xl text-orange-500 text-center mt-20 mb-8">
        BuenzliMarks
      </p>
      <input value={userId()} onInput={(e) => setUserId((e.target as HTMLInputElement).value)} />
      <For each={bookmarks()}>{(bookmark, i) =>
        <Bookmark
          title={bookmark.name}
          link={bookmark.url}
        />
      }</For>
    </div>
  );
}
