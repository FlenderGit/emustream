export function match(value) {
	return /^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(value);
}