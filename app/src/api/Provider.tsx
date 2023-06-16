import { UserContext } from "./context";
import { useAuth } from "../auth";
import { parse } from "../models";
import { useQuery } from "@tanstack/react-query";
import { Login } from "../auth/Login";
import { path, useRequest } from "./hooks";

export function UserProvider({ children }: { children: React.ReactNode }) {
  const { token } = useAuth();
  const request = useRequest();

  const { data: user } = useQuery(
    ["user"],
    async () => {
      const resp = await fetch(path("user", "me"), request("GET"));
      return resp.json().then(parse("user"));
    },
    { enabled: Boolean(token) }
  );
  if (!token || !user) return <Login />;

  return <UserContext.Provider value={user}>{children}</UserContext.Provider>;
}
