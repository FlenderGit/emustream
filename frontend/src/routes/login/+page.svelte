<script>
	import { convert_size_to_fontsize } from "$lib/functions/convert";


	import Icon from "@iconify/svelte";

</script>
<main class="from-background bg-gradient-to-t to-primary/25 flex flex-col flex-center">
    
    <div class="flex items-center gap-2 text-7xl font-semibold">
        <Icon icon="mynaui:controller" font-size={convert_size_to_fontsize("6xl")} />
        <p>Emustream</p>
    </div>

    <div>
        <a href="/">Login</a>
    </div>

</main>
