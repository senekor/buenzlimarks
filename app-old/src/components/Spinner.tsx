export function Spinner() {
  return (
    // taken from
    // https://preline.co/docs/spinners.html
    <div
      className="animate-spin inline-block w-16 h-16 border-[3px] border-current border-t-transparent text-slate-400 rounded-full"
      role="status"
      aria-label="loading"
    >
      <span className="sr-only">Loading...</span>
    </div>
  );
}
