import { createContext, useContext } from "react";
import { Ntt, Typeof } from "../models";

function uninit(): never {
  throw new Error("auth is uninitialized");
}

type Api = {
  get: <N extends Ntt>(n: N, id: string) => Promise<Typeof<N>>;
  getAll: <N extends Ntt>(n: N) => Promise<Typeof<N>[]>;
  post: <N extends Ntt>(n: N, data: Typeof<N>) => Promise<Typeof<N>>;
  put: <N extends Ntt>(n: N, data: Typeof<N>) => Promise<Typeof<N>>;
  del: <N extends Ntt>(n: N, id: string) => Promise<void>;
};

export const ApiContext = createContext<Api>({
  get: uninit,
  getAll: uninit,
  post: uninit,
  put: uninit,
  del: uninit,
});
export const useApi = () => useContext(ApiContext);
