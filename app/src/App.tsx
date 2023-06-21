import { useDeleteEntity, useEntities, useSubmitEntity } from "./api/hooks";
import { Page as PageType } from "./models";
import { Page } from "./components/Page";
import * as Tabs from "@radix-ui/react-tabs";
import { LoadingScreen } from "./components/LoadingScreen";
import { useEffect, useState } from "react";
import { PlusIcon, XMarkIcon } from "@heroicons/react/24/solid";
import { FlexSpace } from "./components/FlexSpace";
import { useAuth } from "./auth/context";
import { ArrowRightOnRectangleIcon } from "@heroicons/react/24/outline";

export function App() {
  const { logout } = useAuth();

  const { data: pages } = useEntities(["page"]);

  const { mutate: submitPage } = useSubmitEntity("page");
  const { mutate: deletePage } = useDeleteEntity("page");

  const [selectedPage, setSelectedPage] = useState<PageType>();
  useEffect(() => {
    if (pages && pages.length > 0) {
      setSelectedPage(pages[0]);
    }
  }, [pages]);

  if (!pages) return <LoadingScreen />;

  return (
    <div className="flex flex-col h-screen text-white">
      <Tabs.Root value={selectedPage?.id || "none"}>
        <Tabs.List className="flex gap-2 p-2">
          {pages?.map((p) => (
            <Tabs.Trigger
              key={p.id}
              className={`bg-slate-600 rounded-lg pl-3 pr-2 py-1 flex flex-row gap-2 ${
                p.id === selectedPage?.id ? "bg-orange-800" : ""
              }`}
              value={p.id}
              onClick={() => setSelectedPage(p)}
            >
              {p.name}
              <XMarkIcon
                className="w-4"
                onClick={(e) => {
                  deletePage(p.id);
                  e.stopPropagation();
                }}
              />
            </Tabs.Trigger>
          ))}
          <FlexSpace />
          <div
            className="bg-slate-600 rounded-full p-1.5"
            onClick={() => submitPage({ id: "", name: "new page" })}
          >
            <PlusIcon className="w-5" />
          </div>
          <div className="bg-slate-600 rounded-full p-1.5" onClick={logout}>
            <ArrowRightOnRectangleIcon className="w-5" />
          </div>
        </Tabs.List>
      </Tabs.Root>
      <div className="self-center flex flex-col gap-1 my-4">
        {selectedPage && <Page page={selectedPage} />}
      </div>
    </div>
  );
}
