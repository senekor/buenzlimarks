import { Component, createSignal } from "solid-js";
import Bookmark from "./components/bookmark";

export default function App() {
  return (
    <div class="flex flex-col dark:bg-slate-800 h-screen">
      <p class="text-4xl text-orange-500 text-center mt-20 mb-8">
        BuenzliMarks
      </p>
      <Bookmark
        title="Requirements"
        link="https://github.com/users/remlse/projects/1/views/6"
      />
      <Bookmark
        title="Prioritization"
        link="https://github.com/users/remlse/projects/1/views/7"
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
