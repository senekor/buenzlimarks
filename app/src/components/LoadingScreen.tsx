import { FlexSpace } from "./FlexSpace";
import { Spinner } from "./Spinner";

export function LoadingScreen() {
  return (
    <div className="flex flex-col h-screen text-white items-center justify-center">
      <FlexSpace />
      <Spinner />
      <FlexSpace />
      <FlexSpace />
    </div>
  );
}
