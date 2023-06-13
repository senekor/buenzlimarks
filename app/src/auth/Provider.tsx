import { useState } from "react";
import { AuthContext } from "./context";
import { useQueryClient } from "@tanstack/react-query";

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [token, setToken] = useState<string>();

  const queryClient = useQueryClient();

  async function login(id: string) {
    const resp = await fetch(`api/auth/login/${id}`);
    setToken(await resp.text());
    queryClient.invalidateQueries([]);
  }
  const logout = () => {
    setToken(undefined);
    queryClient.invalidateQueries([]);
  };

  return (
    <AuthContext.Provider value={{ token, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
}
