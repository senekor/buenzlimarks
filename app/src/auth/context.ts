import { createContext, useContext } from "react";

function uninit(): never {
  throw new Error("auth is uninitialized");
}

type Auth = {
  token: string | undefined;
  login: (id: string) => Promise<void>;
  logout: () => void;
};

export const AuthContext = createContext<Auth>({
  token: undefined,
  login: uninit,
  logout: uninit,
});
export const useAuth = () => useContext(AuthContext);
