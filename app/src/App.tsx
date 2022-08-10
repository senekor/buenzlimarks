import { createResource, createSignal, For, JSX, Show } from "solid-js";

import jwt_decode from "jwt-decode";
import { Icon } from "solid-heroicons";
import { pencilAlt, trash } from "solid-heroicons/outline";

import Bookmark from "./components/bookmark";

type User = {
  id: string;
  name: string;
};

type BookmarkType = {
  id: string;
  name: string;
  url: string;
};

// const getURL = (userId: string) =>
//   `/api/bookmarks/${userId || "nobody"}`;

const fetchBookmarks = async (): Promise<BookmarkType[]> =>
  fetch("/api/bookmarks")
    .then((resp) => resp.json())
    .catch(() => []) as Promise<BookmarkType[]>;

const createBookmark = async (payload: BookmarkType): Promise<BookmarkType> =>
  fetch("/api/bookmarks", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  }).then((resp) => resp.json()) as Promise<BookmarkType>;

const ubdateBookmark = async (payload: BookmarkType): Promise<BookmarkType> =>
  fetch(`/api/bookmarks/${payload.id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  }).then((resp) => resp.json()) as Promise<BookmarkType>;

const deleteBookmark = async (id: string): Promise<Response> =>
  fetch(`/api/bookmarks/${id}`, { method: "DELETE" });

const [user, setUser] = createSignal<User>();

const readUserFromCookie = () => {
  const jwt = document.cookie.match(jwtRegex)?.[1];
  if (jwt) {
    setUser(jwt_decode(jwt));
  } else {
    setUser(undefined);
  }
};

const login = async (id: string) =>
  fetch(`/api/auth/${id}`).then(readUserFromCookie);

const logout = () => {
  document.cookie =
    "buenzlimarks-auth=none;" +
    "SameSite=Lax;" +
    "expires=Thu, 01 Jan 1970 00:00:01 GMT";
  setUser(undefined);
};

const bookmarkTmpl: BookmarkType = {
  id: "",
  name: "",
  url: "",
};

const jwtRegex = /buenzlimarks-auth=(\w*\.\w*\.\w*)/;

const UserForm = (props: { onLogin: () => void }): JSX.Element => {
  const [userId, setUserId] = createSignal("");
  const submit = () => void login(userId()).then(props.onLogin);
  return (
    <div class="flex flex-row self-center gap-2">
      <input
        class="self-center bg-slate-600 p-1 rounded text-white"
        placeholder="Enter a user name"
        value={userId()}
        onInput={(e) => setUserId((e.target as HTMLInputElement).value)}
        onkeydown={(e) => (e.key == "Enter" ? submit() : null)}
      />
      <button
        class="text-white bg-slate-600 w-fit rounded px-1 disabled:text-gray-400"
        disabled={!userId()}
        onClick={submit}
      >
        Login
      </button>
    </div>
  );
};

export const App = (): JSX.Element => {
  readUserFromCookie();

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
      <Show
        when={user()}
        fallback={<UserForm onLogin={() => void refetch()} />}
      >
        <div class="flex flex-row self-center gap-2">
          <div class="text-gray-200">Hello {user()?.name}!</div>
          <button
            class="text-white bg-slate-600 w-fit rounded px-1"
            onClick={() => {
              logout();
              void refetch();
            }}
          >
            Logout
          </button>
        </div>
      </Show>
      <div class="self-center flex flex-col gap-1 my-4">
        <For each={bookmarks()}>
          {(bm) => (
            <div class="flex w-full gap-1">
              <div class="flex-grow" />
              <Bookmark title={bm.name} link={bm.url} />
              <div class="flex-grow" />
              <Icon
                path={pencilAlt}
                class="w-6 ml-2"
                style={{ color: "white" }}
                onClick={() => setForm(bm)}
              />
              <Icon
                path={trash}
                class="w-6"
                style={{ color: "white" }}
                onClick={() =>
                  void deleteBookmark(bm.id)
                    .then(() => (bm.id === form().id ? resetForm() : null))
                    .then(refetch)
                }
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
        class="self-center w-3/4 bg-slate-600 p-1 rounded text-white mb-2"
        placeholder="URL"
        value={form().url}
        onInput={(e) =>
          setForm({ ...form(), url: (e.target as HTMLInputElement).value })
        }
      />
      <div class="self-center flex gap-2">
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
            void (form().id ? ubdateBookmark(form()) : createBookmark(form()))
              .then(resetForm)
              .then(refetch)
          }
        >
          {!form().id ? "Add" : "Save"}
        </button>
      </div>
    </div>
  );
};
