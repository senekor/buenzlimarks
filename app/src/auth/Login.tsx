import { useState } from "react";
import { useAuth } from ".";
import { FlexSpace } from "../components/FlexSpace";

export function Login() {
  const [userId, setUserId] = useState("");

  const { login } = useAuth();
  const submit = () => login(userId);

  return (
    <div className="flex flex-col gap-8 h-screen items-center bg-slate-800">
      <FlexSpace />
      <img src="logo.svg" />
      <input
        className="bg-slate-600 p-2 rounded text-white text-center text-3xl"
        placeholder="Enter a user name"
        value={userId}
        onInput={(e) => setUserId(e.currentTarget.value)}
        onKeyDown={(e) => (e.key === "Enter" ? submit() : null)}
      />
      <button
        className="text-white bg-slate-600 w-fit rounded px-4 py-2 disabled:text-gray-400 text-3xl"
        disabled={!userId}
        onClick={submit}
      >
        Login
      </button>
      <FlexSpace />
      <FlexSpace />
    </div>
  );
}
