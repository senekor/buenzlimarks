import { useEffect } from "react";
import { useAuth } from "./context";
import { useSearchParams } from "react-router-dom";

export function GitHubCallback() {
  const [searchParams] = useSearchParams();

  const { login } = useAuth();

  useEffect(() => {
    // TODO this login request is sent twice. The second time fails
    // obviously, since the auth flow artifacts are only valid once.
    // I have no idea why this components renders twice.
    login(`/api/auth/github/callback?${searchParams}`);
  }, [login, searchParams]);

  return <></>;
}
