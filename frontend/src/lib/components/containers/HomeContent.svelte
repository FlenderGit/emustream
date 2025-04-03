<script lang="ts">
	import type { Input } from '$lib/types/mapper';
	import ExempleTable from './ExempleTable.svelte';

	let row_selected = $state(0);

    const mapper: Record<string, Input> = {
        'z': 'UP',
        's': 'DOWN',
        'q': 'LEFT',
        'd': 'RIGHT',
    }

	const active_callback = (e: KeyboardEvent): Input => {
		e.preventDefault();
		console.log('onkeypress', e.key);
        return mapper[e.key] || 'ENTER';
	};

	const onexit = (direction: 'UP' | 'DOWN') => {
		console.log('onexit', direction);
		const value = direction === 'UP' ? -1 : 1;
		row_selected = Math.max(0, row_selected + value);
	};

	const disable_callback = () => {};
</script>

<svelte:window onkeypress={active_callback} />

<div>
	{#each { length: 8 }, i}
		<ExempleTable {onexit} oninput={row_selected === i ? active_callback : disable_callback} />
        <div class="size-12" class:bg-red-900={row_selected === i}></div>
	{/each}
</div>
