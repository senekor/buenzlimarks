import {
  UseMutationOptions,
  UseQueryOptions,
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";
import { useAuth } from "../auth";
import { Entity, EntityKey, parse, plural, schema } from "../models";
import { useCallback, useMemo } from "react";
import { z } from "zod";
import { snakeCase } from "change-case";

function keysToSnakeCase(
  data: Record<string, unknown>
): Record<string, unknown> {
  return Object.entries(data).reduce(
    (acc, [key, val]) => ({ ...acc, [snakeCase(key)]: val }),
    {}
  );
}

export function useRequest() {
  const { token } = useAuth();

  return (
    method: "GET" | "POST" | "PUT" | "DELETE",
    data?: Record<string, unknown>
  ): RequestInit => {
    return {
      method,
      headers: {
        Authorization: `Bearer ${token}`,
        ...(data ? { "Content-Type": "application/json" } : {}),
      },
      ...(data ? { body: JSON.stringify(keysToSnakeCase(data)) } : {}),
    };
  };
}

export function path<K extends EntityKey>(k: K, id?: string) {
  return `api/${plural(k)}${id ? `/${id}` : ""}`;
}

// start here for better error handling
type TError = unknown;

export function useEntity<K extends EntityKey, TData = Entity<K>>(
  k: K,
  id: string,
  options?: UseQueryOptions<Entity<K>, TError, TData, string[]>
) {
  const request = useRequest();

  const queryFn = useCallback(async (): Promise<Entity<K>> => {
    const resp = await fetch(path(k, id), request("GET"));
    return resp.json().then(parse(k));
  }, [k, id, request]);

  return useQuery([k, id], queryFn, options);
}

export function useEntities<K extends EntityKey, TData = Entity<K>[]>(
  k: K,
  options?: UseQueryOptions<Entity<K>[], TError, TData, string[]>
) {
  const request = useRequest();

  const queryFn = useCallback(async (): Promise<Entity<K>[]> => {
    const resp = await fetch(path(k), request("GET"));
    return resp.json().then(z.array(schema(k)).parse);
  }, [k, request]);

  return useQuery([k] as string[], queryFn, options);
}

export function useSubmitEntity<K extends EntityKey>(
  k: K,
  options?: UseMutationOptions<Entity<K>, TError, Entity<K>, unknown>
) {
  const request = useRequest();
  const queryClient = useQueryClient();

  const mutationFn = useCallback(
    async (data: Entity<K>): Promise<Entity<K>> => {
      const method = data.id ? "PUT" : "POST";
      const resp = await fetch(path(k), request(method, data));
      return resp.json().then(parse(k));
    },
    [k, request]
  );

  const internalOptions = useMemo(() => {
    const onSuccess = (
      data: Entity<K>,
      variables: Entity<K>,
      context: unknown
    ) => {
      queryClient.invalidateQueries([k]);
      if (options?.onSuccess) options.onSuccess(data, variables, context);
    };
    return {
      ...options,
      onSuccess,
    };
  }, [k, options, queryClient]);

  return useMutation([k], mutationFn, internalOptions);
}

export function useDeleteEntity<K extends EntityKey>(
  k: K,
  options?: UseMutationOptions<void, TError, string, unknown>
) {
  const request = useRequest();
  const queryClient = useQueryClient();

  const mutationFn = useCallback(
    async (id: string) => {
      await fetch(path(k, id), request("DELETE"));
    },
    [k, request]
  );

  const internalOptions = useMemo(() => {
    const onSuccess = (data: void, variables: string, context: unknown) => {
      queryClient.invalidateQueries([k]);
      if (options?.onSuccess) options.onSuccess(data, variables, context);
    };
    return {
      ...options,
      onSuccess,
    };
  }, [k, options, queryClient]);

  return useMutation([k], mutationFn, internalOptions);
}
