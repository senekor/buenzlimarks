import { createContext, useContext } from "react";
import { User } from "../models";

export const UserContext = createContext<User>({
  id: "uninit",
  name: null,
});
export const useUser = (): User => useContext(UserContext);
