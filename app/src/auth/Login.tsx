import { useCallback, useState } from "react";
import { useAuth } from "./context";
import { FlexSpace } from "../components/FlexSpace";

function DevLogin() {
  const { login } = useAuth();

  const [userId, setUserId] = useState("");
  const submit = useCallback(
    () => login(`/api/auth/login/${userId}`),
    [login, userId]
  );

  return (
    <>
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
    </>
  );
}

export function GithubLogin() {
  return (
    <a
      className="text-white bg-slate-600 w-fit rounded px-4 py-2 text-3xl"
      href="/api/auth/github/login"
    >
      GitHub Login
    </a>
  );
}

export function Login() {
  return (
    <div className="flex flex-col gap-8 h-screen items-center">
      <FlexSpace />
      <img src="logo.svg" height={256} width={256} />
      {import.meta.env.MODE === "development" ? <DevLogin /> : <GithubLogin />}
      <FlexSpace />
      <FlexSpace />
    </div>
  );
}
