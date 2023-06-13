import { useAuth } from ".";
import { useUser } from "../api/context";

export function LogoutButton() {
  const { logout } = useAuth();
  const user = useUser();

  return (
    <div className="flex flex-row self-center gap-2">
      <div className="text-gray-200">Hello {user.name}!</div>
      <button
        className="text-white bg-slate-600 w-fit rounded px-1"
        onClick={logout}
      >
        Logout
      </button>
    </div>
  );
}
