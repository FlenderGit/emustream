<script lang="ts">
	import { onMount } from "svelte";


    // const URL = import.meta.env.VITE_API_URL;
    const URL = "/api/hello";

    let data = $state("Loading...")

    onMount(() => {
        fetch(URL)
            .then(t => t.text())
            .then(t => {
                data = t;
            })
    })



</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<p>Jean bref...</p>

<p>{ data }</p>