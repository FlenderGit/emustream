import { Controller, EmptyController, type Control } from "$lib/types/mapper";
import { writable } from "svelte/store";

export const main_controller = writable<Controller<Control>>(new EmptyController());