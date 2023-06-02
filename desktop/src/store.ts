import { writable } from "svelte/store";
import type { User } from "./bindings/User";

export const user = writable<User>();
