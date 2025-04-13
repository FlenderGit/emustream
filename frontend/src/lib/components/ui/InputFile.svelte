<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { HTMLInputAttributes } from 'svelte/elements';

	type Props = Omit<HTMLInputAttributes, 'type'> & {};

	const { class: classes, value, ...rest }: Props = $props();
</script>

<label
	class="mx-auto flex max-w-xl cursor-pointer flex-col items-center justify-center border bg-amber-400 py-6 {classes}"
>
	<div class="flex flex-col gap-1">
		<Icon icon="mynaui:upload" class="mx-auto size-10" />
		{value}
	</div>
	<input type="file" class="hidden" {...rest} />
</label>
