<script lang="ts" generics="T extends { id: number }[]">
	import type { Input } from '$lib/types/mapper';
	import { onMount, type Snippet } from 'svelte';

	type Props<T> = {
		promise: Promise<T>;
		data: T;

		children: Snippet;
		oninput: (input: Input) => void; // Use by the container
		onkeypress: (input: Input) => void; // Use by the row/table
		rows: number; // Use by the row/table

		loading?: Snippet;
		onexit: (direction: 'UP' | 'DOWN') => void;
		// Container
		// ls of callback[id_selected]
		// <my-row bind:my-fn />

		// ? Comment detecter l'event de touche
		// ! $state
		// bind sur container <--
	};

	let {
		promise,
		data = $bindable(),
		children,
		onkeypress = $bindable(),
		oninput = $bindable(),
		rows,
		loading = default_loading,
		onexit
	}: Props<T> = $props();

	onkeypress = (input: Input) => {
		if (input === 'UP' && y === 0) {
			onexit(input);
		} else if (input === 'DOWN' && y === rows - 1) {
			onexit(input);
		} else if (input === 'LEFT' && x === 0) {
			// Set sidebar_active to true
			// } else if (input === 'RIGHT' && data.length > ) {
		} else {
			if (input === 'UP') {
				y--;
			} else if (input === 'DOWN') {
				y++;
			} else if (input === 'LEFT') {
				x--;
			} else if (input === 'RIGHT') {
				x++;
			}

			onkeypress(input);
		}
	};

	let status: 'loading' | 'error' | 'success' = $state('loading');
	let error: Error | null = $state(null);
	onMount(() => {
		promise
			.then((t) => {
				data = t;
				status = 'success';
			})
			.catch((e) => {
				error = e;
				status = 'error';
			});
	});

	let x = $state(0);
	let y = $state(0);
</script>

{#snippet default_loading()}
	<p>Loading...</p>
{/snippet}

{#if status === 'loading'}
	{@render loading()}
{:else if status === 'error'}
	<p>{error}</p>
{:else if status === 'success'}
	{@render children()}
{/if}