<script lang="ts">
	import Sidebar from "$lib/components/ui/Sidebar.svelte";

	let { children } = $props();
</script>

<div class="flex slide-up">
	<Sidebar />

	<div class="flex-1">
		{@render children()}
	</div>
</div>
