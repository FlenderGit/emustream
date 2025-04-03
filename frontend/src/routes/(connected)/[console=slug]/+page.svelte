<script lang="ts">
	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
</script>

{#await data.console}
	<p>Loading</p>
{:then d}
	<p>Presentation of { d.name }</p>
    <img src={d.image} alt={d.name} class="w-1/2" />

    <a href={ data.console_slug + "/pokemon" }>Voir jeux</a>
{:catch error}
	<p>{error}</p>
{/await}

<!-- <p>Console: { data.console.name } { data.console_slug }</p>
<p>Description</p>
Lorem ipsum dolor sit amet consectetur adipisicing elit. Perspiciatis, asperiores. Unde minima amet qui aspernatur, repudiandae, ullam recusandae deserunt doloremque nemo facere itaque ex velit deleniti dolores reiciendis quae eaque.

<p>List of games</p>
<ul>
    <li>Game 1</li>
    <li>Game 2</li>
    <li>Game 3</li>
</ul> -->
