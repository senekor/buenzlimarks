import { Component } from "solid-js";

type Props = {
  title: string;
  link: string;
};

const Bookmark: Component<Props> = (props) => {
  return (
    <a
      class="text-2xl dark:text-orange-200 dark:hover:text-orange-400 text-center underline my-1"
      href={props.link}
    >
      {props.title}
    </a>
  );
};

export default Bookmark;
