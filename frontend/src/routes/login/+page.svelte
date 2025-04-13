<script lang="ts">
	import { goto } from "$app/navigation";
	import { fetch_login } from "$lib/functions/api";
	import { convert_size_to_fontsize } from "$lib/functions/convert";
	import { token_store } from "$lib/stores/tokens";


	import Icon from "@iconify/svelte";

    const login = async function (e: MouseEvent) {
        e.preventDefault();

        try {
            const { token } = await fetch_login("Flender", "test");
            console.log("Tokens:", token);
            $token_store = {
                access: token,
                refresh: token,
            }
            goto("/");
        } catch (error) {
            console.error("Error:", error);
            return;
        }

        console.log("Login clicked");
    };

</script>
<main class="from-background bg-gradient-to-t to-primary/25 flex flex-col flex-center">
    
    <div class="flex items-center gap-2 text-7xl font-semibold">
        <Icon icon="mynaui:controller" font-size={convert_size_to_fontsize("6xl")} />
        <p>Emustream</p>
    </div>

    <div>
        <a href="/login" class="btn bg-slate-700" onclick={login}>Login</a>
    </div>

</main>
