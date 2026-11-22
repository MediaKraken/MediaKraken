import { t as hasMethods } from "./predicate-3rF1m2uv.js";
import { r as isHTMLMediaElement } from "./predicates-BCeOmfec.js";
import { t as NO_TIME_RANGES_YET } from "./constants-DeW3h6u4.js";
import { n as MediaStreamTypes } from "./types-D8a85nGw.js";

//#region ../utils/dist/dom/text-track.js
/**
* Whether a text track is a captions or subtitles track.
*
* @internal
*/
function isCaptionOrSubtitleTrack(track) {
	return track.kind === "captions" || track.kind === "subtitles";
}
/**
* Captions and subtitles tracks in the order menus present them: grouped by kind, keeping source order within a kind.
* Shared so selection fallbacks pick the same track the captions menu lists first.
*
* @internal
*/
function getCaptionOrSubtitleTracks(tracks) {
	return Array.from(tracks).filter(isCaptionOrSubtitleTrack).sort(sortByKind);
}
/**
* Find the `<track>` element that owns the given `TextTrack`. The media can be the element itself or an adapter that
* forwards `querySelectorAll()` to it.
*
* @internal
*/
function findTrackElement(media, track) {
	if (!hasMethods(media, ["querySelectorAll"])) return null;
	for (const el of media.querySelectorAll("track")) if (el.track === track) return el;
	return null;
}
/**
* Add a text track to a media element through a `<track>` child, so removing the element removes the track. A track
* from `HTMLMediaElement.addTextTrack()` can never be removed. The track starts `hidden`, like one from
* `addTextTrack()`, and comes from `addTextTrack()` instead where the element yields no track.
*
* Chromium discards cues added to the track before the element settles, which a `<track>` without `src` does by firing
* `error` once it leaves `disabled` mode.
*
* @internal
*/
function addRemovableTextTrack(media, kind, label, language) {
	const element = media.ownerDocument.createElement("track");
	element.kind = kind;
	if (label) element.label = label;
	if (language) element.srclang = language;
	media.append(element);
	const { track } = element;
	if (!track) {
		element.remove();
		return media.addTextTrack(kind, label, language);
	}
	track.mode = "hidden";
	return track;
}
function sortByKind(a, b) {
	return a.kind > b.kind ? 1 : a.kind < b.kind ? -1 : 0;
}

//#endregion
//#region ../media/dist/dev/dom/html-media-adapter/html-media-adapter.js
/**
* Forwards a media surface to an attached native-like target and re-dispatches the target's events on itself. Every
* getter, setter, and method reaches the target directly; anything that intercepts media members sits above this
* adapter, at the player.
*/
var HTMLMediaAdapter = class extends EventTarget {
	#target = null;
	#eventTypes = /* @__PURE__ */ new Set();
	#streamType = MediaStreamTypes.UNKNOWN;
	get target() {
		return this.#target;
	}
	attach(target) {
		if (!target || this.#target === target) return;
		this.#target = target;
		for (const type of this.#eventTypes) target.addEventListener(type, this.#forwardEvent);
	}
	detach() {
		if (!this.#target) return;
		for (const type of this.#eventTypes) this.#target.removeEventListener(type, this.#forwardEvent);
		this.#target = null;
	}
	destroy() {
		this.detach();
		this.#eventTypes.clear();
	}
	querySelectorAll(selectors) {
		return this.target?.querySelectorAll(selectors) ?? [];
	}
	querySelector(selectors) {
		return this.target?.querySelector(selectors) ?? null;
	}
	addEventListener(type, listener, options) {
		if (!this.#eventTypes.has(type)) {
			this.#eventTypes.add(type);
			this.target?.addEventListener(type, this.#forwardEvent);
		}
		super.addEventListener(type, listener, options);
	}
	removeEventListener(type, listener, options) {
		super.removeEventListener(type, listener, options);
	}
	#forwardEvent = (event) => {
		this.dispatchEvent(new event.constructor(event.type, event));
	};
	/**
	* Current stream type (`'on-demand'`, `'live'`, or `'unknown'`). Defaults to `'unknown'`; detecting hosts update it
	* automatically, and consumers can set it to override detection.
	*/
	get streamType() {
		return this.target?.streamType ?? this.#streamType;
	}
	set streamType(value) {
		if (this.streamType === value) return;
		this.#streamType = value;
		if (this.target) this.target.streamType = value;
		this.dispatchEvent(new Event("streamtypechange"));
	}
	get liveEdgeStart() {
		return this.target?.liveEdgeStart ?? NaN;
	}
	get targetLiveWindow() {
		return this.target?.targetLiveWindow ?? NaN;
	}
	get contentData() {
		return this.target?.contentData;
	}
	get title() {
		return this.target?.title ?? "";
	}
	set title(value) {
		if (this.target) this.target.title = value;
	}
	get controls() {
		return this.target?.controls ?? false;
	}
	set controls(value) {
		if (this.target) this.target.controls = value;
	}
	get paused() {
		return this.target?.paused ?? true;
	}
	get ended() {
		return this.target?.ended ?? false;
	}
	get loop() {
		return this.target?.loop ?? false;
	}
	set loop(value) {
		if (this.target) this.target.loop = value;
	}
	play() {
		return this.target?.play?.() ?? Promise.reject(new DOMException("No media is attached.", "NotSupportedError"));
	}
	pause() {
		this.target?.pause?.();
	}
	get autoplay() {
		return this.target?.autoplay ?? false;
	}
	set autoplay(value) {
		if (this.target) this.target.autoplay = value;
	}
	get currentTime() {
		return this.target?.currentTime ?? 0;
	}
	set currentTime(value) {
		if (this.target) this.target.currentTime = value;
	}
	get duration() {
		return this.target?.duration ?? NaN;
	}
	get seeking() {
		return this.target?.seeking ?? false;
	}
	get src() {
		return this.target?.src ?? "";
	}
	set src(value) {
		if (this.target) this.target.src = value;
	}
	get currentSrc() {
		return this.target?.currentSrc ?? "";
	}
	get readyState() {
		return this.target?.readyState ?? 0;
	}
	get preload() {
		return this.target?.preload ?? "metadata";
	}
	set preload(value) {
		if (this.target) this.target.preload = value;
	}
	get crossOrigin() {
		return this.target?.crossOrigin ?? null;
	}
	set crossOrigin(value) {
		if (this.target) this.target.crossOrigin = value;
	}
	load() {
		return this.target?.load?.();
	}
	canPlayType(type) {
		return this.target?.canPlayType?.(type) ?? "";
	}
	get volume() {
		return this.target?.volume ?? 1;
	}
	set volume(value) {
		if (this.target) this.target.volume = value;
	}
	get muted() {
		return this.target?.muted ?? false;
	}
	set muted(value) {
		if (this.target) this.target.muted = value;
	}
	get defaultMuted() {
		return this.target?.defaultMuted ?? false;
	}
	set defaultMuted(value) {
		if (this.target) this.target.defaultMuted = value;
	}
	get playbackRate() {
		return this.target?.playbackRate ?? 1;
	}
	set playbackRate(value) {
		if (this.target) this.target.playbackRate = value;
	}
	/**
	* The target's narrower rate range, when it reports one; native media accept the browser's range.
	*
	* @internal
	*/
	get playbackRateRange() {
		return this.target?.playbackRateRange;
	}
	get defaultPlaybackRate() {
		return this.target?.defaultPlaybackRate ?? 1;
	}
	set defaultPlaybackRate(value) {
		if (this.target) this.target.defaultPlaybackRate = value;
	}
	get buffered() {
		return this.target?.buffered;
	}
	get seekable() {
		return this.target?.seekable;
	}
	get played() {
		return this.target?.played ?? NO_TIME_RANGES_YET;
	}
	get error() {
		return this.target?.error ?? null;
	}
	get textTracks() {
		return this.target?.textTracks;
	}
	addTextTrack(kind, label, language) {
		return this.target?.addTextTrack?.(kind, label, language);
	}
	/**
	* Removes a track backed by a `<track>` element, such as one from `createTextTrack()` or the page's markup. A track
	* from the native `addTextTrack()` can never be removed, so this leaves it.
	*/
	removeTextTrack(track) {
		const target = this.target;
		if (!target) return;
		if (!isHTMLMediaElement(target) && target.removeTextTrack) target.removeTextTrack(track);
		else findTrackElement(target, track)?.remove();
	}
	get remote() {
		return this.target?.remote;
	}
	get disableRemotePlayback() {
		return this.target?.disableRemotePlayback ?? false;
	}
	set disableRemotePlayback(value) {
		if (this.target) this.target.disableRemotePlayback = value;
	}
};

//#endregion
export { isCaptionOrSubtitleTrack as a, getCaptionOrSubtitleTracks as i, addRemovableTextTrack as n, findTrackElement as r, HTMLMediaAdapter as t };
//# sourceMappingURL=html-media-adapter-BttXG6mg.js.map