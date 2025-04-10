<script lang="ts">
	import type { Game } from '$lib/types/entities';

	type Props = {
		games: Game[];
	};

	const { games }: Props = $props();

    const games_2 = $derived(
        [
            ...games,
            ...games,
            ...games,
            ...games
        ]
    )

</script>

<div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-4">
    {#each games_2 as game}
        <div class="bg-gray-200 p-4 rounded-lg shadow-md">
            <h2 class="text-xl font-bold">{game.title}</h2>
        </div>
    {/each}
</div>