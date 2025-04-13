<script lang="ts">
	import { chunk_list } from '$lib/functions/utils';
	import type { Game } from '$lib/types/entities';
	type Props = {
        title: string;
		games: Game[];
	};

	const { title, games }: Props = $props();


	const splited_games = $derived(chunk_list(games, 3));
	$inspect(splited_games);

	let index = $state(0);

	setInterval(() => {
		index = (index + 1) % splited_games.length;
		console.log(index);
	}, 10000);
</script>

<!-- Big cover -->
<div
	class="relative flex h-96 w-full items-end image-center p-4 rounded-2xl bg-blend-color bg-background-light/30"
	style="background-image: url({games[index].cover_url});"
>
    <p class="title absolute top-4 left-4">{title}</p>
	<!-- Smaller one with button left right -->
	{#each splited_games as chunk, i}
		{#if i === index}
			<div class="grid grid-cols-3 gap-4 mx-auto"
            >
				{#each chunk as game}
					<a href="/games/{game.slug}" style="background-image: url({game.cover_url});" class="image-center aspect-video h-40 rounded-xl">
						{game.title}
					</a>
				{/each}
			</div>
		{/if}
	{/each}
</div>
