<script lang="ts">
	import type { HTMLInputAttributes } from "svelte/elements";


    type Props = HTMLInputAttributes;

    const { ...rest }: Props = $props();

</script>

<label>
    <input type="text" {...rest} />
</label>