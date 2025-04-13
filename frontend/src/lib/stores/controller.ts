import type { Control, Controller } from "$lib/types/mapper";
import { writable } from "svelte/store";

export const controller_store = writable<Controller<Control, Control> | null>(null);