<script lang="ts">
	import InputFile from '$lib/components/ui/InputFile.svelte';
	import { webgl_bitmap } from '$lib/functions/webgl/webgl_bitmap';
	import { main_controller } from '$lib/stores/controller';
	import {
		GameboyAdapter,
		KeyboardController,
		type KeyboardKeys,
		type Mapper
	} from '$lib/types/mapper';
	import __wbg_init, { Gameboy, type GameboyKey } from '$lib/wasm/gameboy';
	import { onMount } from 'svelte';

	let gameboy: Gameboy | null = $state(null);
	let wasm = $state();
    let title = $state("");

	const load_game = function (e) {
		const file = e.target.files[0];

		if (file) {
			const reader = new FileReader();
			reader.onload = () => {
				const rom = new Uint8Array(reader.result as ArrayBuffer);
				gameboy = new Gameboy(rom, false);

				const mapper: Mapper<GameboyKey, KeyboardKeys> = {
					a: 'j',
					b: 'k',
					select: 'i',
					start: 'l',
					up: 'z',
					down: 's',
					left: 'q',
					right: 'd'
				};

				$main_controller = new KeyboardController();
				const console_adapter = new GameboyAdapter(mapper);

				$main_controller.on_key_pressed = function (key) {
					key = console_adapter.get_key(key);
					console.log('Key pressed:', key);
					gameboy?.press_key(key);
				};
				$main_controller.on_key_released = function (key) {
					key = console_adapter.get_key(key);
					console.log('Key pressed:', key);
					gameboy?.release_key(key);
				};

				canvas.width = 160;
				canvas.height = 144;

				console.log('Controller:', $main_controller);
				title = gameboy.get_game_name();

				/* main_controller.subscribe((c) => {
					c.on_key_pressed = (key) => {
						key = console_adapter.get_key(key);
                        console.log('Key pressed:', key);
                        gameboy?.press_key(key);
					};
					c.on_key_released = (key) => {
                        key = console_adapter.get_key(key);
                        console.log('Key released:', key);
                        gameboy?.release_key(key);
					};
				}); */

				const ptr = gameboy.get_screen_ptr();
				const gl = webgl_bitmap(canvas, 160, 144);

				console.log('Gameboy:', canvas);

				const buffer = new Uint8Array(wasm.memory.buffer, ptr, 160 * 144 * 3);

				console.log('PTR:', ptr);
				console.log('Screen data:', buffer);

				let last_time = performance.now();

				const update_texture = () => {
					for (let i = 0; i < buffer.length; i += 4) {
						buffer[i] = Math.random() * 255; // R
						buffer[i + 1] = Math.random() * 255; // G
						buffer[i + 2] = Math.random() * 255; // B
						buffer[i + 3] = 255; // A
					}
				};

				const render = (timestamp: number) => {
					const delta = (timestamp - last_time) / 1000;
					last_time = timestamp;

					gameboy?.render(delta);

					gl?.clear(gl.COLOR_BUFFER_BIT);
					gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, 160, 144, gl.RGB, gl.UNSIGNED_BYTE, buffer);
					gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);

					requestAnimationFrame(render);
				};
				render(performance.now());
			};
			reader.readAsArrayBuffer(file);
		}
	};

	let canvas: HTMLCanvasElement;

	onMount(async () => {
		wasm = await __wbg_init();
		console.log('WASM:', canvas);
		const gl = webgl_bitmap(canvas, 160, 144);
		console.log('GL:', gl);
	});
</script>


{#if gameboy}
	<!-- <Canvas use:webgl_bitmap width={160} height={144} /> -->
{:else}
	<InputFile accept=".gb, .gbc" value="Drop a file here" onchange={load_game} />
{/if}

<p class="text-2xl font-bold">{title}</p>
<div class="rounded-2xl bg-neutral-800 p-4 mx-auto">
    <canvas bind:this={canvas} class="h-96 rounded" style="image-rendering: pixelated;"></canvas>

</div>
