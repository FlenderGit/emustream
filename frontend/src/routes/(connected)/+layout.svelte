<script lang="ts">
	import Sidebar from "$lib/components/ui/Sidebar.svelte";

	let { children } = $props();
</script>

<div class="flex">
	<Sidebar />

	<div class="flex-1 container mx-auto p-4">
		{@render children()}
	</div>
</div>
