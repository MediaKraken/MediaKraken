import { w as getGestureCoordinator } from "./volume-4z08ktsN.js";
import { n as getHotkeyCoordinator } from "./hotkey-CLJUSXP5.js";
import { a as isCaptionOrSubtitleTrack } from "./html-media-adapter-BttXG6mg.js";
import { a as selectFullscreen, c as selectPiP, g as selectVolume, h as selectTime, l as selectPlayback, m as selectTextTrack, u as selectPlaybackRate } from "./selectors-BT12EyB_.js";
import { t as hasPlaybackStarted } from "./playback-xsEPXni2.js";
import { n as getTransitionFlags } from "./transition-CzuKD0-9.js";

//#region ../core/dist/dev/core/ui/indicator/lifecycle.js
/** @internal */
var IndicatorCloseController = class {
	#timer = null;
	#close;
	#getDelay;
	constructor(close, getDelay) {
		this.#close = close;
		this.#getDelay = getDelay;
	}
	arm() {
		this.clear();
		this.#timer = setTimeout(() => {
			this.#timer = null;
			this.#close();
		}, this.#getDelay());
	}
	clear() {
		if (this.#timer === null) return;
		clearTimeout(this.#timer);
		this.#timer = null;
	}
	close() {
		this.clear();
		this.#close();
	}
	destroy() {
		this.clear();
	}
};
/** @internal */
var IndicatorVisibilityCoordinator = class {
	#handles = /* @__PURE__ */ new Set();
	register(handle) {
		this.#handles.add(handle);
		return () => this.#handles.delete(handle);
	}
	show(handle) {
		for (const nextHandle of this.#handles) if (nextHandle !== handle) nextHandle.close();
	}
};
/** @internal */
function getIndicatorCloseDelay(props) {
	return props.closeDelay ?? 800;
}
/** @internal */
function isIndicatorPresent(current, transition) {
	return current.open || transition.active;
}
/** @internal */
function getRenderedIndicatorState(current, snapshot, transition) {
	const payload = current.open ? current : snapshot;
	return {
		...payload,
		open: current.open && transition.active,
		generation: current.open ? current.generation : payload.generation,
		...getTransitionFlags(transition.status)
	};
}

//#endregion
//#region ../core/dist/dev/dom/ui/input-action.js
/** @internal */
function toInputActionEvent(event) {
	return {
		action: event.action,
		value: event.value,
		source: event.source,
		key: "key" in event.event ? event.event.key : void 0,
		repeat: "repeat" in event.event ? event.event.repeat : void 0
	};
}
/** @internal */
function getMediaSnapshot(store) {
	if (!store) return {};
	const state = store.state;
	const time = selectTime(state);
	const playback = selectPlayback(state);
	const textTrack = selectTextTrack(state);
	return {
		paused: playback && playback.paused && !playback.autoplaying,
		started: playback && hasPlaybackStarted(playback),
		volume: selectVolume(state)?.volume,
		muted: selectVolume(state)?.muted,
		playbackRate: selectPlaybackRate(state)?.playbackRate,
		isFullscreen: selectFullscreen(state)?.isFullscreen,
		subtitlesShowing: textTrack?.subtitlesShowing,
		subtitlesAvailable: textTrack ? (textTrack.textTrackList ?? []).some(isCaptionOrSubtitleTrack) : void 0,
		isPictureInPicture: selectPiP(state)?.isPictureInPicture,
		currentTime: time?.currentTime,
		duration: time?.duration,
		seeking: time?.seeking
	};
}
/** @internal */
function subscribeToInputActions(container, callback) {
	const handleEvent = (event) => callback(toInputActionEvent(event));
	const gestureUnsubscribe = getGestureCoordinator(container).subscribe(handleEvent);
	const hotkeyUnsubscribe = getHotkeyCoordinator(container).subscribe(handleEvent);
	return () => {
		gestureUnsubscribe();
		hotkeyUnsubscribe();
	};
}
const indicatorVisibilityCoordinators = /* @__PURE__ */ new WeakMap();
/** @internal */
function getIndicatorVisibilityCoordinator(container) {
	let coordinator = indicatorVisibilityCoordinators.get(container);
	if (!coordinator) {
		coordinator = new IndicatorVisibilityCoordinator();
		indicatorVisibilityCoordinators.set(container, coordinator);
	}
	return coordinator;
}

//#endregion
export { getIndicatorCloseDelay as a, IndicatorCloseController as i, getMediaSnapshot as n, getRenderedIndicatorState as o, subscribeToInputActions as r, isIndicatorPresent as s, getIndicatorVisibilityCoordinator as t };
//# sourceMappingURL=input-action-DNt5WtxN.js.map