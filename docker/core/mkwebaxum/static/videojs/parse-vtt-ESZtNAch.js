import { i as toMediaTextCues } from "./volume-4z08ktsN.js";

//#region ../core/dist/dev/dom/store/parse-vtt.js
/**
* Parse WebVTT with the browser's own parser, through a `<track>` on a video that's never shown. Loaded on demand, so
* players that never read a VTT file don't carry it.
*/
function parseVtt(text, kind, signal) {
	if (signal?.aborted) return Promise.reject(signal.reason);
	const video = document.createElement("video");
	const track = document.createElement("track");
	const url = URL.createObjectURL(new Blob([text], { type: "text/vtt" }));
	video.muted = true;
	video.preload = "none";
	track.kind = "metadata";
	return new Promise((resolve, reject) => {
		const cleanup = new AbortController();
		const settle = () => {
			cleanup.abort();
			URL.revokeObjectURL(url);
			track.remove();
		};
		signal?.addEventListener("abort", () => {
			settle();
			reject(signal.reason);
		}, {
			once: true,
			signal: cleanup.signal
		});
		track.addEventListener("load", () => {
			const cues = toMediaTextCues(track.track.cues, { kind });
			settle();
			resolve(cues);
		}, { once: true });
		track.addEventListener("error", () => {
			settle();
			reject(/* @__PURE__ */ new Error("[vjs-core] could not parse a WebVTT text track"));
		}, { once: true });
		video.append(track);
		track.track.mode = "hidden";
		track.src = url;
	});
}

//#endregion
export { parseVtt };
//# sourceMappingURL=parse-vtt-ESZtNAch.js.map