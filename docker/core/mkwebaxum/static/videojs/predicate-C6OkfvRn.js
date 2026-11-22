import { c as isObject, f as isUndefined, i as isFunction, r as isBoolean } from "./predicate-3rF1m2uv.js";
import { t as MediaReadyState } from "./types-D8a85nGw.js";

//#region ../media/dist/dev/core/predicate.js
function hasMetadata(media) {
	return media.readyState >= MediaReadyState.HAVE_METADATA;
}
/** @internal */
function getTimeRangeEnd(media) {
	if (Number.isFinite(media.duration) && media.duration > 0) return media.duration;
	const end = media.seekable.at(-1)?.[1];
	if (end === void 0 || !Number.isFinite(end) || end <= 0) return 0;
	return end;
}
/** @internal */
function hasTimeRange(media) {
	return getTimeRangeEnd(media) > 0;
}
/**
* Whether playback is running without waiting for data.
*
* @internal
*/
function isMediaPlaying(media) {
	return !!media && !media.paused && !media.ended && !media.waiting;
}
/** @internal */
function isMediaAutoplayCapable(value) {
	return isObject(value) && isBoolean(value.autoplay);
}
function isMediaPauseCapable(value) {
	if (!isObject(value)) return false;
	const media = value;
	return !isUndefined(media.paused) && !isUndefined(media.ended) && isFunction(media.pause);
}
function isMediaSeekCapable(value) {
	if (!isObject(value)) return false;
	const media = value;
	return !isUndefined(media.currentTime) && !isUndefined(media.duration) && !isUndefined(media.seeking);
}
function isMediaSourceCapable(value) {
	if (!isObject(value)) return false;
	const media = value;
	return !isUndefined(media.src) && !isUndefined(media.currentSrc) && !isUndefined(media.readyState) && isFunction(media.load);
}
function isMediaVolumeCapable(value) {
	if (!isObject(value)) return false;
	const media = value;
	return !isUndefined(media.volume) && !isUndefined(media.muted);
}
/**
* Whether the media reports a mute at all, which is a narrower question than `isMediaVolumeCapable`: an embed can take
* a mute command while offering no way to set a level.
*
* @internal
*/
function isMediaMutedCapable(value) {
	if (!isObject(value)) return false;
	return !isUndefined(value.muted);
}
function isMediaPlaybackRateCapable(value) {
	if (!isObject(value)) return false;
	return !isUndefined(value.playbackRate);
}
/**
* Only `requestPictureInPicture` is required. A native video element carries it but leaves exiting to `document`, so
* demanding the pair would rule out the one media that most certainly can.
*
* @internal
*/
function isMediaPictureInPictureCapable(value) {
	if (!isObject(value)) return false;
	return isFunction(value.requestPictureInPicture);
}
function isMediaBufferCapable(value) {
	if (!isObject(value)) return false;
	const media = value;
	return !isUndefined(media.buffered) && !isUndefined(media.seekable);
}
function isMediaErrorCapable(value) {
	if (!isObject(value)) return false;
	return !isUndefined(value.error);
}
function isMediaTextTrackCapable(value) {
	if (!isObject(value)) return false;
	return !isUndefined(value.textTracks);
}
function isMediaVideoRenditionCapable(value) {
	if (!isObject(value)) return false;
	return !isUndefined(value.videoRenditions);
}
function isMediaAudioTrackCapable(value) {
	if (!isObject(value)) return false;
	return !isUndefined(value.audioTracks);
}
function isMediaVideoDimensionsCapable(value) {
	if (!isObject(value)) return false;
	const media = value;
	return !isUndefined(media.videoWidth) && !isUndefined(media.videoHeight);
}
function isMediaRemotePlaybackCapable(value) {
	if (!isObject(value)) return false;
	return isObject(value.remote);
}
function isMediaStreamTypeCapable(value) {
	if (!isObject(value)) return false;
	return !isUndefined(value.streamType);
}
/** @internal */
function isMediaContentDataCapable(value) {
	if (!isObject(value)) return false;
	return !isUndefined(value.contentData);
}
function isMediaLiveCapable(value) {
	if (!isObject(value)) return false;
	const media = value;
	return !isUndefined(media.liveEdgeStart) && !isUndefined(media.targetLiveWindow);
}
/** @internal */
function isQuerySelectorAllCapable(value) {
	return isObject(value) && "querySelectorAll" in value && isFunction(value.querySelectorAll);
}
/**
* Whether `value` is an adapter fronting a JS playback engine (an hls.js instance, a dash.js player, an SPF
* composition). Narrows to the caller's type so an adapter keeps its own members alongside `engine`.
*
* @internal
*/
function isEngineAdapter(value) {
	if (!isObject(value)) return false;
	const adapter = value;
	return "engine" in adapter && isFunction(adapter.destroy);
}

//#endregion
export { isMediaVolumeCapable as C, isMediaVideoRenditionCapable as S, isMediaSeekCapable as _, isMediaAudioTrackCapable as a, isMediaTextTrackCapable as b, isMediaContentDataCapable as c, isMediaMutedCapable as d, isMediaPauseCapable as f, isMediaRemotePlaybackCapable as g, isMediaPlaying as h, isEngineAdapter as i, isMediaErrorCapable as l, isMediaPlaybackRateCapable as m, hasMetadata as n, isMediaAutoplayCapable as o, isMediaPictureInPictureCapable as p, hasTimeRange as r, isMediaBufferCapable as s, getTimeRangeEnd as t, isMediaLiveCapable as u, isMediaSourceCapable as v, isQuerySelectorAllCapable as w, isMediaVideoDimensionsCapable as x, isMediaStreamTypeCapable as y };
//# sourceMappingURL=predicate-C6OkfvRn.js.map