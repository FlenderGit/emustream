<script lang="ts">
	import type { Size } from '$lib/types/sizes';

	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';
	import { convert_size_to_fontsize } from '$lib/functions/convert';
	import { goto } from '$app/navigation';
	import { fade } from 'svelte/transition';

	type SidebarLink = {
		name: string;
		icon: string;
		href: string;
		onclick?: (e: MouseEvent) => void;
	};

	const ICONS_SIZE: Size = 'lg';

	const onclick = function (e: MouseEvent) {
		e.preventDefault();
		is_active = false;
		is_hovered = false;
		setTimeout(() => {
			goto('/logout');
		}, 400);
	};

	const links: Array<SidebarLink> = [
		{ name: 'Jeux', href: '/games', icon: 'mynaui:controller' },
		{ name: 'Logout', onclick, icon: 'mynaui:logout', href: '/logout' }
	];

	let is_active = $state(false);
	let is_hovered = $state(false);

	onMount(() => {
		setTimeout(() => (is_active = true), 400);
	});
</script>

<div class="w-16"></div>

<div
	style="view-transition-name: sidebar"
	class="bg-background-light absolute inset-0 z-10 flex w-16 flex-col gap-2 p-2 transition-all duration-500 hover:w-64"
	class:-translate-x-full={!is_active}
	class:opacity-0={!is_active}
	onmouseover={() => (is_hovered = true)}
	onmouseleave={() => (is_hovered = false)}
>
	{#each links as { name, icon, href, onclick }}
		<a
			{href}
			class="hover:bg-primary flex items-center gap-2 overflow-hidden rounded-xl p-1 transition-colors"
			{onclick}
		>
			<Icon {icon} class="shrink-0" font-size={convert_size_to_fontsize(ICONS_SIZE)} />
			<p>{name}</p>
		</a>
	{/each}
</div>

{#if is_hovered}
	<div
		transition:fade
		class="absolute inset-0 bg-neutral-900/50 backdrop-blur-sm transition-all duration-500"
	></div>
{/if}
