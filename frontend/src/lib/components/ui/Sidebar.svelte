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
		{ name: 'Home', href: '/', icon: 'mynaui:home' },
		{ name: 'Profile', href: '/profile', icon: 'mynaui:user' },
		{ name: 'Jeux', href: '/games', icon: 'mynaui:controller' },
		{ name: 'New', href: '/new', icon: 'mynaui:plus' },
	];

	let is_active = $state(false);
	let is_hovered = $state(false);

	onMount(() => {
		setTimeout(() => (is_active = true), 400);
	});
</script>

<div class="w-12"></div>

{#snippet SidebarLink(link: SidebarLink)}
	<a
		href={link.href}
		class="hover:bg-primary flex items-center gap-2 overflow-hidden rounded-xl px-2 py-1 transition-colors"
		onclick={link.onclick}
	>
		<Icon icon={link.icon} class="shrink-0 size-6" font-size={convert_size_to_fontsize(ICONS_SIZE)} />
		<p class="text-sm">{link.name}</p>
	</a>
{/snippet}

<div
	style="view-transition-name: sidebar"
	class="bg-background-light border-r border-whiter/10 fixed inset-0 z-10 w-12 flex flex-col justify-between p-2 transition-all duration-500 hover:w-48"
	class:-translate-x-full={!is_active}
	class:opacity-0={!is_active}
	onmouseover={() => (is_hovered = true)}
	onmouseleave={() => (is_hovered = false)}
>
	<div class="flex flex-col gap-2">
		{#each links as link}
			{@render SidebarLink(link)}
		{/each}
	</div>
	{@render SidebarLink({
		name: 'Logout',
		icon: 'mynaui:logout',
		href: '/logout',
		onclick
	})}

	
</div>

{#if is_hovered}
	<div
		transition:fade
		class="absolute inset-0 bg-neutral-900/50 backdrop-blur-xs transition-all duration-500"
	></div>
{/if}
