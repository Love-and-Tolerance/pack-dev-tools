export function json_fmt(json: string) {
	return JSON.stringify(JSON.parse(json), null, "\t");
}
