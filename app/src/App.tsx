import { Component, createSignal } from "solid-js";
import Bookmark from "./components/bookmark";

const App: Component = () => {
  return (
    <div class="flex flex-col dark:bg-slate-800 h-screen">
      <p class="text-4xl text-orange-500 text-center mt-20 mb-8">
        Buenzlimarks
      </p>
      <Bookmark
        title="Requirements"
        link="https://github.com/remlse/buenzlimarks/issues?q=is%3Aissue+is%3Aopen+label%3A%22User+story%22%2C%22Epic%22"
      />
      <Bookmark
        title="Tasks"
        link="https://github.com/users/remlse/projects/1/views/2"
      />
      <Bookmark
        title="Server Tasks"
        link="https://github.com/users/remlse/projects/1/views/4"
      />
    </div>
  );
};

export default App;
