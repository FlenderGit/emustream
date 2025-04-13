<script lang="ts">
	import type { Game } from '$lib/types/entities';

	type Props = {
        title: string;
        see_more_url?: string;
		games: Game[];
	};

	const { title, games, see_more_url }: Props = $props();

</script>

<div>
    <div class="flex-between-center">
        <p class="title">{title}</p>
        {#if see_more_url}
            <a href={see_more_url}>Voir plus</a>
        {/if}
    </div>
    <div class="grid grid-cols-2 gap-6 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 mt-4">
        {#each games as game}
            <a
                href="/games/{game.slug}"
                class="aspect-video rounded-lg bg-cover bg-center p-4"
                style="background-image: url({game.cover_url});"
            >
                <h2 class="text-xl font-bold">{game.title}</h2>
            </a>
        {/each}
    </div>
    
</div>
