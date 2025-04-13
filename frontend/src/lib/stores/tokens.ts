import { type AuthTokens } from "$lib/types/auth";
import { writable } from "svelte/store";

export const token_store = writable<AuthTokens | null>(null);