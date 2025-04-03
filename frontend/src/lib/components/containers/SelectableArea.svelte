<script lang="ts" generics="T extends { id: number }[]">
	import type { Input } from '$lib/types/mapper';
	import { onMount, type Snippet } from 'svelte';

	type Props<T> = {
		children: Snippet;
		promise: Promise<T>;
		data: T;
		oninput: (input: Input) => void;			// Use by the container
		onkeypress: (input: Input) => void;			// Use by the row/table
		rows: number;								// Use by the row/table

		// Container
		// ls of callback[id_selected]
		// <my-row bind:my-fn />

		// ? Comment detecter l'event de touche
		// ! $state
		// bind sur container <--
	};

	let { children, promise, data = $bindable(), onkeypress = $bindable(), oninput = $bindable(), rows }: Props<T> = $props();

	onkeypress = (input: Input) => {
		if (input === 'UP' && y === 0) {
			// Emit onexit('UP') to parent
		} else if (input === 'DOWN' && y === rows - 1) {
			// Emit onexit('DOWN') to parent
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

		}
	};

	onMount(() => {
		promise.then((t) => {
			data = t;
		});
	});

	let x = $state(0);
	let y = $state(0);

	onMount(() => {
		div_ref.focus();

		const childrens = div_ref.children;
		let selected_child = null;

		const getNumberOfRows = (child) => {
			if (child.getAttribute('data-type') === 'row') {
				return 1;
			}

			return 2;
		};

		const cb = () => {};

		window.addEventListener('keydown', cb);

		return () => {
			window.removeEventListener('keydown', cb);
		};
	});
</script>

<div data-test="fheuzi" bind:this={div_ref}>
	{@render children()}
</div>
