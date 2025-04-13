import { type User } from "$lib/types/entities";
import { writable } from "svelte/store";

function create_user_store() {
    let user = writable<User | null>(null);
    return {
        get: () => user,
        set: (new_user: User) => (user.set(new_user)),
        logout: () => (user.set(null)),
        subscribe: (run: (value: User | null) => void) => {
            return user.subscribe(run);
        }
    }
}

export const user_store = create_user_store();
