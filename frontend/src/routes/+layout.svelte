<script lang="ts">
	import { onNavigate } from '$app/navigation';
	import '../app.css';

	let { children } = $props();

	let div_ref: HTMLDivElement;



	onNavigate((navigation) => {
		if (!document.startViewTransition) return;

		const { from, to } = navigation;
		const url = from?.url.pathname;
		const from_home = url === '/' || url === '/logout';

		console.log('from', from, 'to', to, 'url', url, 'from_home', from_home);

		const dir = from_home ? 'down' : 'up';
		const dir_inverse = from_home ? 'up' : 'down';

		div_ref.classList.add('view-transition-scroll-' + dir_inverse);
		div_ref.classList.remove('view-transition-scroll-' + dir);


		return new Promise((resolve) => {
			document.startViewTransition(async () => {
				resolve();
				await navigation.complete;
			});
		});
	});
</script>

<div bind:this={div_ref} class="grid">
	{@render children()}
</div>
