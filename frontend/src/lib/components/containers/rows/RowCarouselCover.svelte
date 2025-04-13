<script lang="ts">
	import { get_color_for_tag } from '$lib/functions/convert';
	import { extract_platforms, extract_regions } from '$lib/functions/utils';
	import type { Game } from '$lib/types/entities';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';
	import type { HTMLAttributes } from 'svelte/elements';
	import { crossfade } from 'svelte/transition';

	type Props = {
		games: Game[];
	};

	const { games, class: classes, ...rest }: Props & HTMLAttributes<HTMLDivElement> = $props();

	let index = $state(0);

	function v(i: number) {
		return (i - index) % games.length;
	}

	onMount(() => {
		const interval = setInterval(() => {
			index = (index + 1) % games.length;
		}, 10000);

		return () => clearInterval(interval);
	});

	const advance = (value: number) => {
		index = (index + value) % games.length;
	};
</script>

<div class="relative grid h-96 w-full grid-cols-5 gap-4 {classes}" {...rest}>
	<div class="relative col-span-3">
		<button
			class="left-15 border-whiter/20 absolute top-1/2 z-20 size-10 cursor-pointer rounded-full border-2 backdrop-blur-md"
			onclick={() => advance(-1)}
		>
			<Icon icon="mynaui:chevron-left" class="mx-auto size-9" />
		</button>

		<button
			class="right-15 border-whiter/20 absolute top-1/2 z-20 size-10 cursor-pointer rounded-full border-2 backdrop-blur-md"
			onclick={() => advance(1)}
		>
			<Icon icon="mynaui:chevron-right" class="mx-auto size-9" />
		</button>

		{#each games as game, i (i)}
			<div
				style:transform={'translateX(' + v(i) * 4 + 'rem)'}
				class="absolute-center image-center rounded-4xl flex aspect-video h-96 items-end overflow-hidden transition-all duration-500 outline outline-primary/10"
				style:scale={1 - Math.abs(v(i) * 0.1)}
				style:z-index={3 - Math.abs(v(i))}
				style:background-image={'url(' + game.cover_url + ')'}
				style:opacity={1 - Math.abs(v(i) * 0.4)}
				class:opacity-0={Math.abs(v(i)) > 2}
			>
				<div class="bg-background-light border-whiter w-full border-t-2 p-4">
					<div class="flex-between-center gap-2">
						<p class="font-semibold">{game.title}</p>
						<div class="flex gap-2">
							{#each game.tags as tag}
								<span class="rounded-full px-2 py-1 text-xs {get_color_for_tag(tag)}">{tag}</span>
							{/each}
						</div>
					</div>
					<p class="text-sm font-light">Exemple</p>
				</div>
			</div>
		{/each}
	</div>

	<div class="bg-background-light col-span-2 flex flex-col gap-2 rounded-2xl px-5 py-3">
		<p class="title">{games[index].title}</p>
		<div>
			{#each extract_regions(games[index]) as region}
				<p>{region}</p>
			{/each}
		</div>
		<div>
			{#each extract_platforms(games[index]) as platform}
				<p>{platform}</p>
			{/each}
		</div>

		<div class="flex-1">

		</div>

		<div class="flex justify-end gap-4">
			<a class="btn bg-primary/70" href="#"> Voir le jeu </a>
			<a class="btn bg-primary/70" href="#"> Lancer une partie </a>
		</div>
	</div>
</div>
