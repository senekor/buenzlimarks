import { ApiContext } from "./context";
import { useAuth } from "../auth";
import { Ntt, Typeof, impl } from "../models";
import { z } from "zod";

function path<N extends Ntt>(ntt: N, id?: string) {
  return `api/${impl[ntt].plural}${id ? `/${id}` : ""}`;
}

export function ApiProvider({ children }: { children: React.ReactNode }) {
  const { token } = useAuth();

  function request(
    method: "GET" | "POST" | "PUT" | "DELETE",
    data?: unknown
  ): RequestInit {
    return {
      method,
      headers: {
        Authorization: `Bearer ${token}`,
        ...(data ? { "Content-Type": "application/json" } : {}),
      },
      ...(data ? { body: JSON.stringify(data) } : {}),
    };
  }
  async function get<N extends Ntt>(ntt: N, id: string): Promise<Typeof<N>> {
    const resp = await fetch(path(ntt, id), request("GET"));
    return resp.json().then(impl[ntt].schema.parse);
  }
  async function getAll<N extends Ntt>(ntt: N): Promise<Typeof<N>[]> {
    const resp = await fetch(path(ntt), request("GET"));
    return resp.json().then(z.array(impl[ntt].schema).parse);
  }
  async function post<N extends Ntt>(ntt: N, data: Typeof<N>) {
    const resp = await fetch(path(ntt), request("POST", data));
    return resp.json().then(impl[ntt].schema.parse);
  }
  async function put<N extends Ntt>(ntt: N, data: Typeof<N>) {
    const resp = await fetch(path(ntt), request("PUT", data));
    return resp.json().then(impl[ntt].schema.parse);
  }
  async function del<N extends Ntt>(ntt: N, id: string) {
    const resp = await fetch(path(ntt, id), request("DELETE"));
    await resp;
  }

  return (
    <ApiContext.Provider
      value={{
        get,
        getAll,
        post,
        put,
        del,
      }}
    >
      {children}
    </ApiContext.Provider>
  );
}

export default ApiProvider;
