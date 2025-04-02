<script lang="ts">
	import type { Size } from '$lib/types/sizes';

	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';
	import { convert_size_to_fontsize } from '$lib/ts/convert';
	import { goto } from '$app/navigation';

	type SidebarLink = {
		name: string;
		icon: string;
	} & ({ href: string } | { onclick: () => void });

	const ICONS_SIZE: Size = 'lg';

    const onclick = function() {
		is_active = false;
        setTimeout(() => {
			goto('/login');
        }, 400);
    }

	const links: Array<SidebarLink> = [
        { name: 'Jeux', href: '/games', icon: 'mynaui:controller' },
        { name: 'Logout', onclick, icon: 'mynaui:logout'}
    ];

	let is_active = $state(false);

	onMount(() => {
		setTimeout(() => (is_active = true), 400);
	});

</script>

<div class="w-16"></div>

<div
	class="bg-secondary absolute inset-0 w-16 p-2 transition-all duration-500 flex flex-col hover:w-64"
	class:-translate-x-full={!is_active}
>
	{#each links as link}
		{#if 'href' in link}
			<a href={link.href} class="flex p-1 items-center gap-2 overflow-hidden hover:bg-amber-300 rounded-xl">
				<Icon icon={link.icon} class="shrink-0" font-size={convert_size_to_fontsize(ICONS_SIZE)} />
				<p>{link.name}</p>
			</a>
		{:else}
			<div onclick={link.onclick} class="flex p-1 items-center gap-2 overflow-hidden">
				<Icon icon={link.icon} class="shrink-0" font-size={convert_size_to_fontsize(ICONS_SIZE)} />
				<p>{link.name}</p>
            </div>
		{/if}
	{/each}
</div>
