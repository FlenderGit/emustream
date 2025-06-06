export const webgl_bitmap = function (canvas: HTMLCanvasElement, width: number, height: number) {
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
			vec2 flippedTexCoord = vec2(v_texCoord.x, 1.0 - v_texCoord.y);
			gl_FragColor = texture2D(u_texture, flippedTexCoord);
		}
	`;

	function createShader(gl: WebGLRenderingContext, type: number, source: string) {
		const shader = gl.createShader(type);

		if (!shader) {
			console.error('Unable to create shader');
			return null;
		}

		gl.shaderSource(shader, source);
		gl.compileShader(shader);

		if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
			console.error(gl.getShaderInfoLog(shader));
			gl.deleteShader(shader);
			return null;
		}
		return shader;
	}

	const gl = canvas.getContext('webgl');
	if (!gl) return;

	const vs = createShader(gl, gl.VERTEX_SHADER, vertexShader);
	const fs = createShader(gl, gl.FRAGMENT_SHADER, fragmentShader);
	if (!vs || !fs) return;

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
	gl.viewport(0, 0, width, height);

	const positionBuffer = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
	gl.bufferData(
		gl.ARRAY_BUFFER,
		new Float32Array([
			-1,
			-1,
			1,
			-1,
			-1,
			1,
			1,
			1
		]),
		gl.STATIC_DRAW
	);

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
	gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGB, width, height, 0, gl.RGB, gl.UNSIGNED_BYTE, null);

	// Uniform de texture
	const textureLocation = gl.getUniformLocation(program, 'u_texture');
	gl.uniform1i(textureLocation, 0);

    gl.clearColor(0, .4, 0, 1);
    /* gl.clear(gl.COLOR_BUFFER_BIT);
    gl.clear(gl.DEPTH_BUFFER_BIT); */

    return gl;
};
