<script lang="ts" generics="T extends { id: number}">
	import { getContext, type Snippet } from 'svelte';

	let { setValue } = getContext('selectableArea')

	/* setInterval(() => {
		console.log('setValue')
		setValue(Math.floor(Math.random() * 1000));
	}, 1000); */


	type Props<T> = {
		loading: Snippet;
		promise: Promise<T[]>;
		item: Snippet<[T]>;
        children: Snippet;
		row: number;
	};

    let x = $state(0);
    let y = $state(0);

	let { loading = default_loading, promise, item, row }: Props<T> = $props();
</script>

{#snippet default_loading()}
    <p>Loading</p>
{/snippet}

<div class="flex gap-2">
	{#await promise}
		{@render loading()}
	{:then data}
		{#each data as it (it.id)}
            {@render item(it)}
        {/each}
	{:catch error}
		<p>{error}</p>
	{/await}
</div>
