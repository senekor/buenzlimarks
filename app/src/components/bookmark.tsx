import { Component } from "solid-js";

type Props = {
  title: string;
  link: string;
};

export default function Bookmark(props: Props) {
  return (
    <a
      class="text-2xl dark:text-orange-200 dark:hover:text-orange-400 text-center underline my-1"
      href={props.link}
    >
      {props.title}
    </a>
  );
};
