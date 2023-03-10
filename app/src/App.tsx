import {
	createEffect,
	createMemo,
	createResource,
	createSignal,
	For,
	Show,
} from "solid-js";

import { Icon } from "solid-heroicons";
import { pencil, trash } from "solid-heroicons/outline";

import Bookmark from "./components/Bookmark";
import {
	bookmarks,
	useCreateBookmark,
	useDeleteBookmark,
	useUpdateBookmark,
} from "./api";
import { Bookmark as BookmarkType, UserSchema } from "./models";
import { useLogout, setUserId, user } from "./auth";

const bookmarkTmpl: BookmarkType = {
	id: "",
	name: "",
	url: "",
	widget_id: "",
};

function UserForm() {
	const [userIdForm, setUserIdForm] = createSignal("");
	const submit = () => setUserId(userIdForm());
	return (
		<div class="flex flex-row self-center gap-2">
			<input
				class="self-center bg-slate-600 p-1 rounded text-white"
				placeholder="Enter a user name"
				value={userIdForm()}
				onInput={(e) => setUserIdForm(e.currentTarget.value)}
				onkeydown={(e) => (e.key === "Enter" ? submit() : null)}
			/>
			<button
				class="text-white bg-slate-600 w-fit rounded px-1 disabled:text-gray-400"
				disabled={!userIdForm()}
				onClick={submit}
			>
				Login
			</button>
		</div>
	);
}

export function App() {
	const widget_id = () => bookmarks()?.[0]?.widget_id || "";

	const [form, setForm] = createSignal<BookmarkType>(bookmarkTmpl);
	const resetForm = () =>
		setForm({
			...bookmarkTmpl,
			widget_id: widget_id(),
		});

	// hack to add bookmarks to valid widget
	createEffect(() => {
		if (widget_id) {
			setForm((oldForm) => ({ ...oldForm, widget_id: widget_id() }));
		}
	});

	const createBookmark = () => useCreateBookmark()(resetForm);
	const updateBookmark = () => useUpdateBookmark()(resetForm);
	const deleteBookmark = () => useDeleteBookmark()(resetForm);

	return (
		<div class="flex flex-col bg-slate-800 h-screen">
			<p class="text-4xl text-orange-500 text-center mt-12 mb-8">
				buenzlimarks
			</p>
			<Show when={user()} fallback={<UserForm />}>
				<div class="flex flex-row self-center gap-2">
					<div class="text-gray-200">Hello {user()?.name}!</div>
					<button
						class="text-white bg-slate-600 w-fit rounded px-1"
						onClick={useLogout()}
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
								path={pencil}
								class="w-6 ml-2"
								style={{ color: "grey" }}
								// onClick={() => setForm(bm)}
							/>
							<Icon
								path={trash}
								class="w-6"
								style={{ color: "white" }}
								onClick={() => deleteBookmark()(bm.id)}
							/>
						</div>
					)}
				</For>
			</div>
			<input
				class="self-center w-3/4 bg-slate-600 p-1 rounded text-white mb-1"
				placeholder="Name"
				value={form().name}
				onInput={(e) => setForm({ ...form(), name: e.currentTarget.value })}
			/>
			<input
				class="self-center w-3/4 bg-slate-600 p-1 rounded text-white mb-2"
				placeholder="URL"
				value={form().url}
				onInput={(e) => setForm({ ...form(), url: e.currentTarget.value })}
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
						form().id ? updateBookmark()(form()) : createBookmark()(form())
					}
				>
					{!form().id ? "Add" : "Save"}
				</button>
			</div>
		</div>
	);
}
