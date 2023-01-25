export function json_minify(json: string) {
	return JSON.stringify(JSON.parse(json));
}
