<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { PageProps } from './$types';

	const { data }: PageProps = $props();

	const { console_slug, game_slug, game } = data;
</script>

<div class="grid grid-cols-9 gap-4">
	<div class="col-span-6">
		<div>
			<img
				src={game?.cover_url}
				alt={'Image de ' + game.title}
				class="h-80 w-full rounded-2xl object-cover"
			/>
			<div class="m-4 -mt-8 flex-between-center">
				<p class="text-3xl font-semibold">{game.title}</p>
				<a class="flex-between-center" href={game_slug + '/play'}>
					Play
					<Icon icon="mynaui:controller" class="size-12" />
				</a>
			</div>
		</div>
		<p>{game_slug} ({console_slug})</p>
		<a href={'/' + data.console_slug}>Voir d'autres jeux de la console</a>

		<p>Description</p>
		Lorem ipsum, dolor sit amet consectetur adipisicing elit. Earum vero magnam eius enim mollitia ratione
		aperiam eaque necessitatibus minus veniam adipisci tempora harum odio, ex magni accusantium inventore
		dolorem perspiciatis?
	</div>
	<div class="col-span-3">
		<p>Notifications</p>
        <a href="https://dribbble.com/shots/23332203-Gaming-dashboard-design">
            Exemple
        </a>
	</div>
</div>
