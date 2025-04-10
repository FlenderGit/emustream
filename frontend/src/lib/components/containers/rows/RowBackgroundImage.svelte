<script lang="ts">
	import type { Game } from "$lib/types/entities";


    type Props = {
        games: Game[];
    }

    const { games }: Props = $props();

    let index = $state(0);

</script>

<!-- Big cover -->
<div>

    <!-- Smaller one with button left right -->
    {#each games as game, i}
        <div class="relative w-full h-64 bg-cover bg-center" style="background-image: url({game.cover_url});">
            <div class="absolute inset-0 bg-black opacity-50"></div>
            <div class="relative z-10 p-4 text-white">
                <h2 class="text-xl font-bold">{game.title}</h2>
            </div>
        </div>
    {/each}

</div>