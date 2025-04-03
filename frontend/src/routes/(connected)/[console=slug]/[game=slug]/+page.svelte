<script lang="ts">
	import type { PageProps } from "./$types";


    const { data }: PageProps = $props();

    const { console_slug, game_slug } = data;

</script>

<p>{game_slug} ({console_slug})</p>
<a href={ "/" + data.console_slug }>Voir d'autres jeux de la console</a>

<p>Description</p>
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Earum vero magnam eius enim mollitia ratione aperiam eaque necessitatibus minus veniam adipisci tempora harum odio, ex magni accusantium inventore dolorem perspiciatis?