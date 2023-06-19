import { useCallback, useMemo, useState } from "react";
import { AuthContext } from "./context";
import { useQueryClient } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";

const TOKEN_STORAGE_KEY = "buenzlimarks_auth";

function initialToken(): string | undefined {
  const storedToken = localStorage.getItem(TOKEN_STORAGE_KEY);
  if (storedToken) return storedToken;
  if (import.meta.env.MODE === "development") return "buenzli";
  return undefined;
}

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const queryClient = useQueryClient();
  const navigate = useNavigate();

  const [token, setToken] = useState(initialToken);

  const login = useCallback(
    async (url: string) => {
      const resp = await fetch(url);
      if (resp.status !== 200) {
        // TODO this is necessary because the callback components
        // render twice for some reason, meaning one of the login
        // requests fail.
        // Ideally, a failed login would result in an error message
        // or kick back to the login screen. But with this, we have
        // to ignore it, otherwise login is dead on arrival.
        return;
      }
      const newToken = await resp.text();
      localStorage.setItem(TOKEN_STORAGE_KEY, newToken);
      setToken(newToken);
      queryClient.invalidateQueries([]);
      queryClient.clear();
      navigate("/");
    },
    [queryClient, navigate]
  );
  const logout = useCallback(() => {
    localStorage.removeItem(TOKEN_STORAGE_KEY);
    setToken(undefined);
    queryClient.invalidateQueries([]);
    queryClient.clear();
  }, [queryClient]);

  const auth = useMemo(
    () => ({ token, login, logout }),
    [login, logout, token]
  );

  return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
}
