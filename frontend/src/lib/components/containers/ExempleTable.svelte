<script lang="ts">
	import BaseContainerSelectable from "./BaseContainerSelectable.svelte";


	let data = $state([])

	type Props = {
		onexit: (direction: 'UP' | 'DOWN') => void;
		oninput: (input: string) => Input;
	}

	const { onexit, oninput }: Props = $props()

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
		];
		setTimeout(() => {
			resolve(out);
		}, 2000);
	});

	const onkeypress = (input: any) => {
		console.log('oninput', input);
	};
	
</script>

<BaseContainerSelectable {promise} rows={1} {onkeypress} {onexit} {oninput} bind:data={data}>

	<div class="grid grid-cols-6">
		{#await promise}
			<p>Loading...</p>
		{:then data} 
			{#each data as d}
				<p>{d}</p>
			{/each}
		{:catch error}
			<p>{error}</p>
		{/await}
	</div>

</BaseContainerSelectable>
