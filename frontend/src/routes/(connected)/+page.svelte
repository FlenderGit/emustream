<script lang="ts">
	import RowPlayed from "$lib/components/containers/rows/RowPlayed.svelte";
	import SelectableArea from "$lib/components/containers/SelectableArea.svelte";
import { onMount } from "svelte";


    // const URL = import.meta.env.VITE_API_URL;
    const URL = "/api/hello";

    let data = $state("Loading...")

    onMount(() => {
        fetch(URL, {
            headers: {
                "Authorization": "Bearer " + "jdhzif",
            },
        })
            .then(t => t.text())
            .then(t => {
                data = t;
            })
    })



</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<p>Jean bref...</p>

<p>Recu: { data }</p>

<SelectableArea>
    <RowPlayed/>
</SelectableArea>

