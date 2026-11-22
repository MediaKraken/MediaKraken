import { f as isUndefined, u as isPromise } from "./predicate-3rF1m2uv.js";
import { t as noop } from "./noop-DBLxABor.js";
import { E as getMediaInputActionValue, y as roundPlaybackRate } from "./volume-4z08ktsN.js";
import { a as selectFullscreen, c as selectPiP, g as selectVolume, h as selectTime, l as selectPlayback, u as selectPlaybackRate } from "./selectors-BT12EyB_.js";

//#region ../core/dist/dev/dom/media-actions.js
/** Swallow a rejected promise, since input actions have no caller to report it to. */
function ignoreRejection(result) {
	if (isPromise(result)) result.catch(noop);
}
const MEDIA_INPUT_ACTION_OVERRIDES = {
	togglePaused({ store, state = store.state }) {
		const playback = selectPlayback(state);
		if (!playback) return;
		ignoreRejection(playback.paused && !playback.autoplaying ? playback.play() : playback.pause());
	},
	toggleMuted({ store }) {
		const volume = selectVolume(store.state);
		if (!volume) return;
		volume.setMuted(!(volume.muted || volume.volume === 0));
	},
	toggleFullscreen({ store }) {
		const fullscreen = selectFullscreen(store.state);
		if (!fullscreen) return;
		ignoreRejection(fullscreen.isFullscreen ? fullscreen.exitFullscreen() : fullscreen.requestFullscreen());
	},
	togglePictureInPicture({ store }) {
		const pip = selectPiP(store.state);
		if (!pip) return;
		ignoreRejection(pip.isPictureInPicture ? pip.exitPictureInPicture() : pip.requestPictureInPicture());
	},
	seekStep({ store, value, key }) {
		const step = getMediaInputActionValue("seekStep", key, value);
		const time = selectTime(store.state);
		if (!time) return;
		time.seek(time.currentTime + step);
	},
	volumeStep({ store, value, key }) {
		const step = getMediaInputActionValue("volumeStep", key, value);
		const vol = selectVolume(store.state);
		if (!vol) return;
		vol.setVolume(vol.volume + step);
	},
	speedUp({ store, value }) {
		const rate = selectPlaybackRate(store.state);
		if (!rate) return;
		if (!isUndefined(value)) {
			rate.setPlaybackRate(roundPlaybackRate(rate.playbackRate + value));
			return;
		}
		const { playbackRates, playbackRate } = rate;
		const faster = playbackRates.filter((candidate) => candidate > playbackRate);
		if (faster.length === 0) return;
		rate.setPlaybackRate(Math.min(...faster));
	},
	speedDown({ store, value }) {
		const rate = selectPlaybackRate(store.state);
		if (!rate) return;
		if (!isUndefined(value)) {
			rate.setPlaybackRate(roundPlaybackRate(rate.playbackRate - value));
			return;
		}
		const { playbackRates, playbackRate } = rate;
		const slower = playbackRates.filter((candidate) => candidate < playbackRate);
		if (slower.length === 0) return;
		rate.setPlaybackRate(Math.max(...slower));
	}
};

//#endregion
export { ignoreRejection as n, MEDIA_INPUT_ACTION_OVERRIDES as t };
//# sourceMappingURL=media-actions-BsfUScq4.js.map