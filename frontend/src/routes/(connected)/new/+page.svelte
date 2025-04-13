<script lang="ts">
	import Input from '$lib/components/ui/Input.svelte';

	let is_total = $state(false);
</script>

<div>
	<h1 class="title">Ajouter un nouveau jeu</h1>
	<form>
		<input type="checkbox" bind:checked={is_total} />
		<Input value="title" placeholder="Titre du jeu" />

		{#if is_total}
			<p>Show total form</p>
		{/if}
	</form>
</div>
