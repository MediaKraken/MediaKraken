import { d as isString } from "./predicate-3rF1m2uv.js";

//#region ../media/dist/dev/core/source/adapter-type.js
/**
* Resolve a source's MIME type from its file extension, such as `application/x-mpegurl` for `.m3u8` or `video/mp4` for
* `.mp4`. The query string and hash are ignored.
*
* Returns `null` when the URL has no extension or one that isn't an HLS or DASH manifest or a common video or audio
* file.
*
* @param src - The source URL.
* @public
*/
function resolveMimeType(src) {
	const extension = EXTENSION.exec(src.trim())?.[1].toLowerCase();
	return extension && MIME_TYPES.get(extension) || null;
}
/**
* The query parameter that marks a URL as a player.js embed. player.js is a protocol any host can speak, so the URL has
* to say so; the player.js media removes the parameter before loading the embed.
*
* @internal
*/
const PLAYERJS_PARAM = "playerjs";
/**
* Match a source URL to a service by host alone, without parsing it for an id. A Mux URL must also have a stream path.
* A `playerjs` query parameter marks a player.js embed on any host, ahead of the host itself, and a `wvideo` parameter
* marks a Wistia popover on any other page.
*
* @internal
*/
function matchServiceHost({ hostname, pathname, searchParams }) {
	if (searchParams.has("playerjs")) return "playerjs";
	const hostPath = hostname + pathname;
	return Object.keys(SERVICE_HOSTS).find((service) => SERVICE_HOSTS[service].test(hostPath)) ?? (searchParams.has("wvideo") ? "wistia" : null);
}
/**
* Match a structured source to the service whose media plays it by an id alone, such as a Mux `playbackId`, without a
* URL. An empty id doesn't count.
*
* @internal
*/
function matchServiceId(source) {
	for (const [service, field] of SERVICE_ID_FIELDS) {
		const id = source[field];
		if (isString(id) && id) return service;
	}
	return null;
}
/**
* Map a MIME type to the adapter type that plays it: `hls`, `dash`, `video`, or `audio`. Parameters such as `codecs`
* are ignored.
*
* @internal
*/
function adapterTypeForMimeType(type) {
	const mimeType = (type ?? "").split(";", 1)[0].trim().toLowerCase();
	if (HLS_TYPE.test(mimeType)) return "hls";
	if (mimeType === DASH_TYPE) return "dash";
	return FILE_TYPE.exec(mimeType)?.[1] ?? null;
}
/**
* Parse an absolute URL, or a host-first one such as `youtu.be/<id>`. Paths and bare ids don't parse.
*
* @internal
*/
function parseSourceUrl(src) {
	try {
		return new URL(HOST_FIRST.test(src) ? `https://${src}` : src);
	} catch {
		return null;
	}
}
/** @internal */
const SPOTIFY_URI = /^spotify:/i;
const HOST_FIRST = /^[\w-]+(\.[\w-]+)+\//;
const EXTENSION = /^[^?#]*\.(\w+)([?#]|$)/;
const SERVICE_HOSTS = {
	youtube: /^((www|m|music)\.)?youtube(-nocookie)?\.com\/|^youtu\.be\//,
	vimeo: /^((www|player)\.)?vimeo\.com\//,
	wistia: /^([^/]+\.)?(wistia\.(com|net)|wi\.st)\//,
	mux: /^stream\.mux\.com\/[^/.]+(\.m3u8)?$/,
	cloudflare: /^([^/]+\.)?(cloudflarestream\.com|videodelivery\.net)\//,
	spotify: /^open\.spotify\.com\//,
	soundcloud: /^([^/]+\.)?soundcloud\.com\//,
	mixcloud: /^([^/]+\.)?mixcloud\.com\//,
	tiktok: /^(www\.)?tiktok\.com\//,
	twitch: /^((www|go|player)\.)?twitch\.tv\//,
	dailymotion: /^((www|geo)\.)?dailymotion\.com\/|^dai\.ly\//
};
const SERVICE_ID_FIELDS = [["mux", "playbackId"], ["wistia", "mediaId"]];
const HLS_TYPE = /^(application|audio)\/(vnd\.apple\.|x-)?mpegurl$/;
const DASH_TYPE = "application/dash+xml";
const FILE_TYPE = /^(video|audio)\//;
const MIME_TYPES = /* @__PURE__ */ new Map([
	["m3u8", "application/x-mpegurl"],
	["mpd", DASH_TYPE],
	["mp4", "video/mp4"],
	["webm", "video/webm"],
	["mov", "video/quicktime"],
	["ogv", "video/ogg"],
	["mp3", "audio/mpeg"],
	["m4a", "audio/mp4"],
	["wav", "audio/wav"],
	["ogg", "audio/ogg"],
	["flac", "audio/flac"],
	["aac", "audio/aac"]
]);

//#endregion
export { matchServiceId as a, matchServiceHost as i, SPOTIFY_URI as n, parseSourceUrl as o, adapterTypeForMimeType as r, resolveMimeType as s, PLAYERJS_PARAM as t };
//# sourceMappingURL=adapter-type-C7BiENB5.js.map