import { LogoutButton } from "./auth/LogoutButton";
import { useEntities } from "./api/hooks";
import { Page as PageType } from "./models";
import { Page } from "./components/Page";

export function App() {
  const { data: pages } = useEntities("page");

  return (
    <div className="flex flex-col h-screen text-white">
      <h1 className="text-4xl text-orange-500 text-center mt-12 mb-8">
        buenzlimarks
      </h1>
      <LogoutButton />
      <div className="self-center flex flex-col gap-1 my-4">
        {pages?.map((p: PageType) => (
          <Page key={p.id} page={p} />
          // <div key={bm.id} className="flex w-full gap-1">
          //   <FlexSpace />
          //   <BookmarkComp title={bm.name} link={bm.url} />
          //   <FlexSpace />
          //   <PencilIcon
          //     className="w-6 ml-2"
          //     style={{ color: "white" }}
          //     onClick={() => setForm(bm)}
          //   />
          //   <TrashIcon
          //     className="w-6"
          //     style={{ color: "white" }}
          //     onClick={() => deleteBookmark(bm.id)}
          //   />
          // </div>
        ))}
      </div>
      {/* <input
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
      </div> */}
    </div>
  );
}
