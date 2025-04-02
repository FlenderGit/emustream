<script lang="ts">
	import DefaultTableSelectable from './DefaultTableSelectable.svelte';

	type A = { id: number; name: string };

	const promise: Promise<A[]> = new Promise((resolve) => {
		const out = [
			{ id: 1, name: 'John Doe' },
			{ id: 2, name: 'Jane Doe' },
			{ id: 3, name: 'Jim Doe' },
			{ id: 4, name: 'Jack Doe' },
			{ id: 5, name: 'Jill Doe' },
			{ id: 6, name: 'Joe Doe' },
			{ id: 7, name: 'Judy Doe' },
			{ id: 8, name: 'Jake Doe' },
			{ id: 9, name: 'Jasmine Doe' },
			{ id: 10, name: 'Jared Doe' },
		] as A[];
		setTimeout(() => {
			resolve(out);
		}, 2000);
	});
</script>

<DefaultTableSelectable {promise} rows={1}>

    <p>Exemple table</p>

    {#snippet loading()}
        <p>Loading...</p>
    {/snippet}

    {#snippet item(data)}
		<div tabindex="0">
			<p>{data.id} - {data.name}</p>

		</div>
    {/snippet}

</DefaultTableSelectable>
