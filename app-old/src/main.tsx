import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App.tsx";
import "./index.css";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AuthGuard } from "./auth/AuthGuard.tsx";
import { AuthProvider } from "./auth/Provider.tsx";
import { createBrowserRouter, Outlet, RouterProvider } from "react-router-dom";
import { Login } from "./auth/Login.tsx";
import { GitHubCallback } from "./auth/GitHubCallback.tsx";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";

const queryClient = new QueryClient();

const router = createBrowserRouter([
  {
    path: "/",
    element: (
      <AuthProvider>
        <Outlet />
      </AuthProvider>
    ),
    children: [
      {
        index: true,
        element: (
          <AuthGuard>
            <App />
          </AuthGuard>
        ),
      },
      {
        path: "login",
        element: <Login />,
      },
      {
        path: "auth/github/callback",
        element: <GitHubCallback />,
      },
    ],
  },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <ReactQueryDevtools />
      <RouterProvider router={router} />
    </QueryClientProvider>
  </React.StrictMode>
);
