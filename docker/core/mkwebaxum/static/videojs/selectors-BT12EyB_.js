import { t as pick } from "./pick-BAh7WYWl.js";
import { A as throwNoTargetError, C as controlsFeature, S as fullscreenFeature, b as remotePlaybackFeature, c as errorFeature, d as playbackFeature, f as playbackRateFeature, g as timeFeature, h as streamTypeFeature, j as AbortControllerRegistry, l as liveFeature, m as sourceFeature, n as textTrackFeature, o as audioTrackFeature, p as qualityFeature, r as textCuesFeature, s as bufferFeature, t as volumeFeature, u as metadataFeature, x as pipFeature } from "./volume-4z08ktsN.js";

//#region ../store/dist/dev/core/selector.js
const stateContext = {
	target: throwNoTargetError,
	signals: new AbortControllerRegistry(),
	get: throwNoTargetError,
	set: throwNoTargetError
};
/**
* Create a type-safe selector for a slice's state.
*
* The selector returns the slice's state, or `undefined` if the slice is not configured in the store.
*
* @example
*   ```ts
*   const selectPlayback = createSelector(playbackSlice);
*   selectPlayback(store.state); // { paused, play, pause, ... } | undefined
*   selectPlayback.displayName; // 'playback' (from slice name)
*   ```;
*
* @param slice - The slice to create a selector for.
*/
function createSelector(slice) {
	const initialState = slice.state(stateContext);
	const keys = [...Object.keys(initialState), ...Object.keys(slice.derived ?? {})];
	const firstKey = keys[0];
	if (!firstKey) return Object.assign(() => void 0, { displayName: slice.name });
	return Object.assign((state) => {
		if (!(firstKey in state)) return void 0;
		return pick(state, keys);
	}, { displayName: slice.name });
}

//#endregion
//#region ../core/dist/dev/dom/store/selectors.js
/** Select the audio track state (audioTrackList, selectAudioTrack). */
const selectAudioTrack = createSelector(audioTrackFeature);
/** Select the buffer state (buffered, seekable). */
const selectBuffer = createSelector(bufferFeature);
/** Select the controls state (controlsVisible, userActive, toggleControls, requestControlsLock). */
const selectControls = createSelector(controlsFeature);
/** Select the error state (error, dismissError). */
const selectError = createSelector(errorFeature);
/** Select the fullscreen state (isFullscreen, fullscreenAvailability, requestFullscreen, exitFullscreen). */
const selectFullscreen = createSelector(fullscreenFeature);
/** Select the live state (`liveEdgeStart`, `targetLiveWindow`). */
const selectLive = createSelector(liveFeature);
/** Select resolved content metadata (title, poster). */
const selectMetadata = createSelector(metadataFeature);
/**
* Select the PiP state (isPictureInPicture, pictureInPictureAvailability, requestPictureInPicture,
* exitPictureInPicture).
*/
const selectPiP = createSelector(pipFeature);
/** Select the playback state (paused, ended, play, pause). */
const selectPlayback = createSelector(playbackFeature);
/** Select the playback rate state (playbackRate, playbackRates, setPlaybackRate). */
const selectPlaybackRate = createSelector(playbackRateFeature);
/** Select the quality state (videoRenditionList, activeVideoRendition, selectVideoRendition). */
const selectQuality = createSelector(qualityFeature);
/** Select the remote playback state (remotePlaybackState, remotePlaybackAvailability, promptRemotePlayback). */
const selectRemotePlayback = createSelector(remotePlaybackFeature);
/** Select the source state (currentSrc, canPlay). */
const selectSource = createSelector(sourceFeature);
/** Select the stream type state (`streamType`: `'on-demand' | 'live' | 'unknown'`). */
const selectStreamType = createSelector(streamTypeFeature);
/** Select the text cues state (textCues, watchTextCues). */
const selectTextCues = createSelector(textCuesFeature);
/**
* Select the text track state (textTrackList, subtitlesShowing, toggleSubtitles, selectSubtitlesTrack, chaptersCues,
* thumbnailsTrack).
*/
const selectTextTrack = createSelector(textTrackFeature);
/** Select the time state (currentTime, duration, seek). */
const selectTime = createSelector(timeFeature);
/** Select the volume state (volume, muted, setVolume, setMuted). */
const selectVolume = createSelector(volumeFeature);

//#endregion
export { selectFullscreen as a, selectPiP as c, selectQuality as d, selectRemotePlayback as f, selectVolume as g, selectTime as h, selectError as i, selectPlayback as l, selectTextTrack as m, selectBuffer as n, selectLive as o, selectTextCues as p, selectControls as r, selectMetadata as s, selectAudioTrack as t, selectPlaybackRate as u };
//# sourceMappingURL=selectors-BT12EyB_.js.map