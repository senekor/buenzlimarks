import { useSettings } from "../api/hooks";
import { useAuth } from "./context";

export function LogoutButton() {
  const { logout } = useAuth();

  const { data: settings } = useSettings();

  return (
    <div className="flex flex-row self-center gap-2">
      <div className="text-gray-200">Hello {settings?.name || ""}!</div>
      <button
        className="text-white bg-slate-600 w-fit rounded px-1"
        onClick={logout}
      >
        Logout
      </button>
    </div>
  );
}
