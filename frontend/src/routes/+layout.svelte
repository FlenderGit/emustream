<script lang="ts">
	import { onNavigate } from '$app/navigation';
	import '../app.css';

	let { children } = $props();

	let div_ref: HTMLDivElement;

	onNavigate((navigation) => {
		if (!document.startViewTransition) return;

		const { from, to } = navigation;
		const from_url = from?.url.pathname;
		const to_url = to?.url.pathname; 

		let animation = 'scroll-left';

		if (to_url === '/login') {
			animation = 'scroll-up';
		} else if (from_url === '/login') {
			animation = 'scroll-down';
		} else if (from_url?.length > to_url?.length) {
			animation = 'scroll-right';
		} else {
			animation = 'scroll-left';
		}
		console.log("Selected:", animation);


		// Revode all class that start with view-transition-
		div_ref.classList.forEach((className) => {
			if (className.startsWith('view-transition-')) {
				div_ref.classList.remove(className);
			}
		});

		div_ref.classList.add('view-transition-' + animation);

		return new Promise((resolve) => {
			document.startViewTransition(async () => {
				resolve();
				await navigation.complete;
			});
		});
	});
</script>

<div bind:this={div_ref} class="flex flex-col">
	{@render children()}
</div>
