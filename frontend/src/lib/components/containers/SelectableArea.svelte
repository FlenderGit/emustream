<script lang="ts" generics="T extends { getNumberLines(): number }">
	import { onMount, setContext, type Snippet } from 'svelte';

	type Props = {
		children: Snippet;
	};

	const { children }: Props = $props();

    let div_ref: HTMLDivElement;

	let value = $state(0);
	setContext('selectableArea', {
		getValue: () => value,
		setValue: (v: number) => {
			value = v;
		},
	});

	/* setInterval(() => {
		console.log('setValue');
		value = Math.floor(Math.random() * 10);
	}, 10); */

	// const URL = import.meta.env.VITE_API_URL;
	// const URL = "/api/hello";

	// let data = $state("Loading...")

	// onMount(() => {
	//     fetch(URL, {
	//         headers: {
	//             "Authorization": "Bearer " + "jdhzif",
	//         },
	//     })
	//         .then(t => t.text())
	//         .then(t => {
	//             data = t;
	//         })
	// })

	const child_selected = $state(0);

	onMount(() => {

		div_ref.focus();

		const childrens = div_ref.children;
		let selected_child = null;


		const getNumberOfRows = (child) => {
			if (child.getAttribute('data-type') === 'row') {
				return 1;
			}

			return 2;
		}
		


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
