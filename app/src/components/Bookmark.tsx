export function Bookmark({ title, link }: { title: string; link: string }) {
  return (
    <a
      className="text-2xl text-orange-200 hover:text-orange-400 underline"
      href={link}
    >
      {title}
    </a>
  );
}
