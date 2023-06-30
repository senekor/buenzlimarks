import { useAuth } from "./context";
import { useSettings } from "../api/hooks";
import { Navigate } from "react-router-dom";

export function AuthGuard({ children }: { children: React.ReactNode }) {
  const { token } = useAuth();

  const { data: settings, isError } = useSettings({
    enabled: Boolean(token),
    retry: false,
  });

  if (!token || isError) return <Navigate to="/login" />;

  if (!settings)
    return (
      // <div className="flex h-screen items-center justify-center">
      //   <div className="text-white text-4xl">Loading...</div>
      // </div>

      // TODO nice loading spinner with gentle contrast
      <></>
    );

  return children;
}
