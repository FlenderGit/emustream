<script lang="ts">
	import Canvas from "$lib/components/Canvas.svelte";
	import { controller_store } from "$lib/stores/controller";
	import { TypeAction } from "$lib/types/mapper";
	import { Gameboy } from "$lib/wasm/gameboy";

    let game_loaded = $state(false);

    const load_game = function(e) {
        const file = e.target.files[0];
        if (file) {
            const reader = new FileReader();
            reader.onload = () => {
                const rom = new Uint8Array(reader.result as ArrayBuffer);

                const gameboy = new Gameboy(rom, false);
                const ptr = gameboy.get_screen_ptr();
                /* const screen_data = new Uint8Array(gameboy.memory.buffer, ptr, 160 * 144 * 4);
                gameboy.render() */

                console.log("PTR:", ptr);

                game_loaded = true;
            };
            reader.readAsArrayBuffer(file);
        }
    }

</script>

{#if game_file}
    <Canvas width={160} height={144} />
{:else}
    <input type="file" accept=".gb,.gbc" onchange={load_game} />
{/if}



