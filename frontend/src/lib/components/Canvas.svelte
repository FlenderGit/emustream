<script lang="ts">
	import { KeyboardController, map_keyboard_gb, TypeAction } from '$lib/types/mapper';
	import { onMount } from 'svelte';

	let canvas: HTMLCanvasElement;

	const WIDTH = 160;
	const HEIGHT = 144;
	const buffer = new Uint8Array(WIDTH * HEIGHT * 4);

	const vertexShader = `
		attribute vec2 a_position;
		varying vec2 v_texCoord;
		void main() {
			gl_Position = vec4(a_position, 0.0, 1.0);
			v_texCoord = a_position * 0.5 + 0.5;
		}
	`;

	const fragmentShader = `
		precision mediump float;
		varying vec2 v_texCoord;
		uniform sampler2D u_texture;
		void main() {
			gl_FragColor = texture2D(u_texture, v_texCoord);
		}
	`;

	function createShader(gl: WebGLRenderingContext, type: number, source: string) {
		const shader = gl.createShader(type);
		gl.shaderSource(shader, source);
		gl.compileShader(shader);
		
		if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
			console.error(gl.getShaderInfoLog(shader));
			gl.deleteShader(shader);
			return null;
		}
		return shader;
	}

	onMount(() => {
		const gl = canvas.getContext('webgl');
		if (!gl) return;

        const controller = new KeyboardController(map_keyboard_gb);
        controller.on(TypeAction.PRESS, console.log);
        controller.on(TypeAction.RELEASE, console.log);

		// Création des shaders avec vérification d'erreurs
		const vs = createShader(gl, gl.VERTEX_SHADER, vertexShader);
		const fs = createShader(gl, gl.FRAGMENT_SHADER, fragmentShader);
		if (!vs || !fs) return;

		// Création du programme
		const program = gl.createProgram();
		gl.attachShader(program, vs);
		gl.attachShader(program, fs);
		gl.linkProgram(program);
		
		if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
			console.error(gl.getProgramInfoLog(program));
			return;
		}
		gl.useProgram(program);

		// Configuration du viewport
		gl.viewport(0, 0, WIDTH, HEIGHT);

		// Création du buffer de position
		const positionBuffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
		gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([
			-1, -1, 1, -1, -1, 1, 1, 1 // Quad couvrant tout l'écran
		]), gl.STATIC_DRAW);

		// Configuration de l'attribut de position
		const positionLocation = gl.getAttribLocation(program, 'a_position');
		gl.enableVertexAttribArray(positionLocation);
		gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0);

		// Configuration de la texture
		const texture = gl.createTexture();
		gl.bindTexture(gl.TEXTURE_2D, texture);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
		gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, WIDTH, HEIGHT, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);

		// Uniform de texture
		const textureLocation = gl.getUniformLocation(program, 'u_texture');
		gl.uniform1i(textureLocation, 0);

		function updateTexture() {
			for (let i = 0; i < buffer.length; i += 4) {
				buffer[i] = Math.random() * 255; // R
				buffer[i + 1] = Math.random() * 255; // G
				buffer[i + 2] = Math.random() * 255; // B
				buffer[i + 3] = 255; // A
			}
			gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, WIDTH, HEIGHT, gl.RGBA, gl.UNSIGNED_BYTE, buffer);
		}

        let frames = 0;
        let lastTime = performance.now();
        gl.clearColor(0, 0, 0, 1);

        function updateFPS() {
            const now = performance.now();
            frames++;
            if (now - lastTime >= 1000) {
                console.log(`FPS: ${frames}`);
                frames = 0;
                lastTime = now;
            }
        }
		function render() {
			gl.clear(gl.COLOR_BUFFER_BIT);
			
			updateTexture();
            updateFPS();
			gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
			
			requestAnimationFrame(render);
		}

		render();
	});
</script>

<canvas
	bind:this={canvas}
	width={WIDTH}
	height={HEIGHT}
	class="h-96"
    style="image-rendering: pixelated;"
></canvas>