function Bookmark({ title, link }: { title: string; link: string }) {
  return (
    <a
      className="text-2xl dark:text-orange-200 dark:hover:text-orange-400 underline"
      href={link}
    >
      {title}
    </a>
  );
}

export default Bookmark;
