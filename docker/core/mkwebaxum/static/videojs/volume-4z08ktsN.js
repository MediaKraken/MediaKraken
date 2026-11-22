import { c as isObject, d as isString, f as isUndefined, i as isFunction, n as isAbortError, o as isNull } from "./predicate-3rF1m2uv.js";
import { t as deepEqual } from "./deep-equal-mHeggeG2.js";
import { r as getLocaleKey, s as DEFAULT_LOCALE, t as findLocaleKeys } from "./i18n-ByQGZLGb.js";
import { t as listen$1 } from "./listen-CO63BggB.js";
import { t as noop } from "./noop-DBLxABor.js";
import { r as resolveEventTarget, t as anyAbortSignal } from "./abort-DsWgOZ8b.js";
import { a as isPointInElement } from "./layout-BqK_K8jO.js";
import { n as hideOpenPopovers } from "./popover-CL3vBHX-.js";
import { a as isCaptionOrSubtitleTrack, i as getCaptionOrSubtitleTracks, n as addRemovableTextTrack, r as findTrackElement } from "./html-media-adapter-BttXG6mg.js";
import { t as isWebKitAirPlayCapable } from "./webkit-DXmoZ2Wz.js";
import { a as getRegisteredMedia, n as getMediaElement } from "./media-target-BNborqvj.js";
import { n as MediaStreamTypes, r as toMediaCrossOrigin, t as MediaReadyState } from "./types-D8a85nGw.js";
import { C as isMediaVolumeCapable, S as isMediaVideoRenditionCapable, _ as isMediaSeekCapable, a as isMediaAudioTrackCapable, b as isMediaTextTrackCapable, c as isMediaContentDataCapable, d as isMediaMutedCapable, f as isMediaPauseCapable, g as isMediaRemotePlaybackCapable, l as isMediaErrorCapable, m as isMediaPlaybackRateCapable, n as hasMetadata, o as isMediaAutoplayCapable, p as isMediaPictureInPictureCapable, s as isMediaBufferCapable, u as isMediaLiveCapable, v as isMediaSourceCapable, w as isQuerySelectorAllCapable, x as isMediaVideoDimensionsCapable, y as isMediaStreamTypeCapable } from "./predicate-C6OkfvRn.js";
import { t as isInteractionLocked } from "./interaction-lock-1oRjjx9B.js";
import { t as clamp } from "./number-D2skb7-A.js";

//#region ../utils/dist/dom/interactive.js
/** @internal */
const INTERACTIVE_SELECTOR = [
	"button",
	"input",
	"select",
	"textarea",
	"a[href]",
	"[role=\"button\"]",
	"[role=\"menu\"]",
	"[role=\"menuitem\"]",
	"[role=\"menuitemcheckbox\"]",
	"[role=\"menuitemradio\"]",
	"[role=\"slider\"]",
	"[data-interactive]"
].join(",");
/** @internal */
const EDITABLE_SELECTOR = [
	"textarea",
	"select",
	"input:not([type])",
	...[
		"text",
		"search",
		"url",
		"tel",
		"email",
		"password",
		"number"
	].map((type) => `input[type="${type}"]`),
	"[contenteditable]:not([contenteditable=\"false\"])"
].join(",");
/** @internal */
function isEditableElement(el) {
	return el.matches(EDITABLE_SELECTOR);
}
/**
* Whether the keyboard event target is an editable element (input, textarea, etc).
*
* @internal
*/
function isEditableTarget(event) {
	const target = resolveEventTarget(event);
	return target instanceof Element && isEditableElement(target);
}
/**
* Whether the event originated from an interactive control (button, slider, etc).
*
* @internal
*/
function isInteractiveTarget(event) {
	const target = resolveEventTarget(event);
	if (!(target instanceof Element)) return false;
	return target.closest(INTERACTIVE_SELECTOR) !== null;
}
const ACTIVATION_KEYS = /* @__PURE__ */ new Set([" ", "Enter"]);
/**
* Selector for elements that use Space/Enter as a native activation key. Narrower than `INTERACTIVE_SELECTOR` —
* excludes editable elements like `input`, `textarea`, `select` where Space/Enter is text input, not activation.
*/
const ACTIVATABLE_SELECTOR = "button,a[href],[role=\"slider\"],[role=\"button\"]";
/**
* Whether the event is an activation key on an activatable element (button, link, slider).
*
* @internal
*/
function isInteractiveActivation(event) {
	if (!ACTIVATION_KEYS.has(event.key)) return false;
	const target = resolveEventTarget(event);
	return target instanceof Element && target.matches(ACTIVATABLE_SELECTOR);
}

//#endregion
//#region ../utils/dist/time/ranges.js
/**
* Converts a time ranges object to an array of [start, end] tuples.
*
* @internal
*/
function serializeTimeRanges(ranges) {
	const result = [];
	for (let i = 0; i < ranges.length; i++) result.push([ranges.start(i), ranges.end(i)]);
	return result;
}

//#endregion
//#region ../utils/dist/events/listen.js
/**
* Add an event listener and return a cleanup function to remove it.
*
* The DOM-free counterpart of `listen` from `@videojs/utils/dom`, for code that must run against any event target.
*
* @example
*   ```ts
*   const cleanup = listen(media, 'play', () => console.log('playing'));
*   cleanup(); // Remove listener
*   ```;
*
* @internal
*/
function listen(target, type, listener, options) {
	target.addEventListener(type, listener, options);
	return () => target.removeEventListener(type, listener);
}
/**
* Wait for an event to occur on a target. Rejects with the signal's reason if `options.signal` aborts first.
*
* Removes its listener itself rather than relying on `once`, which `EventTargetLike` emitters need not support.
*
* @internal
*/
function onEvent(target, type, options) {
	return new Promise((resolve, reject) => {
		const signal = options?.signal;
		if (signal?.aborted) {
			reject(signal.reason ?? "Aborted");
			return;
		}
		const onAbort = () => {
			target.removeEventListener(type, onFire);
			reject(signal?.reason ?? "Aborted");
		};
		const onFire = (event) => {
			target.removeEventListener(type, onFire);
			signal?.removeEventListener("abort", onAbort);
			resolve(event);
		};
		signal?.addEventListener("abort", onAbort, { once: true });
		target.addEventListener(type, onFire);
	});
}

//#endregion
//#region ../store/dist/dev/core/abort-controller-registry.js
/** @internal */
var AbortControllerRegistry = class {
	#base;
	#keys = /* @__PURE__ */ new Map();
	/** The attach-scoped signal. Aborts on detach or reattach. */
	get base() {
		return (this.#base ??= new AbortController()).signal;
	}
	/** Clears all keyed signals, leaving base intact. */
	clear() {
		for (const controller of this.#keys.values()) controller.abort();
		this.#keys.clear();
	}
	/** Resets base and clears all keyed signals. */
	reset() {
		this.clear();
		this.#base?.abort();
		this.#base = void 0;
	}
	/** Creates a new signal for the key, superseding any previous signal. */
	supersede(key) {
		this.#keys.get(key)?.abort();
		const controller = new AbortController();
		this.#keys.set(key, controller);
		return anyAbortSignal([this.base, controller.signal]);
	}
};

//#endregion
//#region ../store/dist/dev/core/errors.js
/** @internal */
var StoreError = class extends Error {
	code;
	cause;
	constructor(code, options) {
		super(options?.message ?? code);
		this.name = "StoreError";
		this.code = code;
		this.cause = options?.cause;
	}
};
/** @internal */
function throwNoTargetError() {
	throw new StoreError("NO_TARGET");
}
/** @internal */
function throwDestroyedError() {
	throw new StoreError("DESTROYED");
}

//#endregion
//#region ../core/dist/dev/core/feature.js
/**
* Create a feature definer for one target type. Platforms bind their full target with it, so features that need more
* than `media`, such as a container, can read it.
*
* @internal
*/
function createPlayerFeatureFactory() {
	function define(definition) {
		const preserved = Object.values(definition.config ?? {}).map((entry) => entry.state);
		return {
			...definition,
			...preserved.length > 0 ? { preserve: preserved } : {}
		};
	}
	return define;
}
/**
* Define a platform-neutral player feature. Its target holds only `media`, so the feature composes into any platform's
* player store.
*
* @internal
*/
const definePlayerFeature$1 = createPlayerFeatureFactory();
/**
* Merge the configuration declarations from the selected player features.
*
* @internal
*/
function combinePlayerFeatureConfigs(features) {
	const definitions = features.map((feature) => feature.config ?? {});
	{
		const seen = /* @__PURE__ */ new Set();
		for (const definition of definitions) for (const key of Object.keys(definition)) {
			if (seen.has(key)) console.warn(`[vjs-core] duplicate config key "${key}" — later feature overwrites earlier one`);
			seen.add(key);
		}
	}
	return Object.assign({}, ...definitions);
}
/**
* Forward one configuration input through its feature-owned private action.
*
* @internal
*/
function setPlayerConfigValue(store, entry, value) {
	const action = store[entry.action];
	if (typeof action !== "function") throw new TypeError(`Missing config action "${String(entry.action)}"`);
	action(value);
}

//#endregion
//#region ../core/dist/dev/dom/feature.js
/**
* Define a player feature against the web target, including its `container`. A feature that only reads `media` should
* use `definePlayerFeature` from `@videojs/core` instead, so it also composes into non-DOM players.
*
* @internal
*/
const definePlayerFeature = createPlayerFeatureFactory();

//#endregion
//#region ../core/dist/dev/dom/media-action-value.js
function getMediaInputActionValue(action, key, value) {
	if (!isUndefined(value)) return value;
	const normalizedKey = key?.toLowerCase();
	if (action === "seekStep") return normalizedKey === "arrowleft" || normalizedKey === "j" ? -10 : 10;
	if (action === "volumeStep") return normalizedKey === "arrowdown" ? -.05 : 5 / 100;
}

//#endregion
//#region ../core/dist/dev/dom/gesture/action-value.js
/**
* Resolves the effective value for a gesture action from its explicit value and region.
*
* @internal
*/
function getGestureActionValue(action, region, value) {
	if (action === "seekStep" && isUndefined(value) && region === "left") return -10;
	return getMediaInputActionValue(action, void 0, value);
}

//#endregion
//#region ../core/dist/dev/dom/gesture/region.js
/**
* Determine which named region a pointer position falls into.
*
* Regions divide the container width equally based on how many are active: - `left` + `right` → halves (50% / 50%) -
* `left` + `center` + `right` → thirds (33% / 34% / 33%)
*
* Single region: `left` covers the left half, `right` the right half, and `center` covers the full surface. Partial
* two-region combos (e.g. `left` + `center`) use the same natural zones — positions outside all active zones return
* `null` so full-surface gestures can handle them.
*/
function resolveRegion(clientX, containerRect, activeRegions) {
	const relativeX = clientX - containerRect.left;
	const width = containerRect.width;
	if (width === 0) return null;
	const ratio = relativeX / width;
	if (activeRegions.size === 2 && activeRegions.has("left") && activeRegions.has("right")) return ratio < .5 ? "left" : "right";
	if (activeRegions.size === 3) {
		if (ratio < 1 / 3) return "left";
		if (ratio < 2 / 3) return "center";
		return "right";
	}
	if (activeRegions.has("left") && ratio < .5) return "left";
	if (activeRegions.has("right") && ratio >= .5) return "right";
	if (activeRegions.has("center")) {
		if (activeRegions.size === 1) return "center";
		if (ratio >= 1 / 3 && ratio < 2 / 3) return "center";
	}
	return null;
}

//#endregion
//#region ../core/dist/dev/dom/gesture/coordinator.js
const TAP_THRESHOLD$1 = 250;
/** @internal */
var GestureCoordinator = class {
	#target;
	#bindings = [];
	#recognizers = /* @__PURE__ */ new Set();
	#disconnect = null;
	#subscribers = /* @__PURE__ */ new Set();
	constructor(target) {
		this.#target = target;
	}
	get bindings() {
		return this.#bindings;
	}
	subscribe(callback) {
		this.#subscribers.add(callback);
		return () => this.#subscribers.delete(callback);
	}
	/**
	* Whether a registered binding claims this tap for the given action. A claimed tap belongs to the gesture layer, so
	* callers should leave it alone. Taps on interactive targets (buttons, sliders) are never claimed — the same
	* filtering the pointerup listener applies. A disabled binding still claims: disabling a gesture opts out of the
	* action, it doesn't hand the tap back to a fallback handler.
	*/
	claimsTap(event, action) {
		if (isInteractionLocked(this.#target)) return true;
		if (isInteractiveTarget(event)) return false;
		return this.#bindings.some((b) => b.type === "tap" && b.action === action && (!b.pointer || b.pointer === event.pointerType));
	}
	add(binding) {
		const value = getGestureActionValue(binding.action ?? "", binding.region, binding.value);
		const wrapped = {
			...binding,
			value,
			onActivate: (event) => {
				if (this.#subscribers.size > 0) {
					const activateEvent = {
						type: binding.type,
						source: "gesture",
						action: binding.action,
						value,
						region: binding.region,
						pointer: binding.pointer,
						event
					};
					for (const cb of this.#subscribers) try {
						cb(activateEvent);
					} catch (error) {
						console.warn("[vjs-gesture] subscribe callback threw:", error);
					}
				}
				binding.onActivate(event);
			}
		};
		this.#bindings.push(wrapped);
		this.#recognizers.add(wrapped.recognizer);
		this.#connect();
		let removed = false;
		return () => {
			if (removed) return;
			removed = true;
			const idx = this.#bindings.indexOf(wrapped);
			if (idx !== -1) this.#bindings.splice(idx, 1);
			this.#maybeDisconnect();
		};
	}
	#connect() {
		if (this.#disconnect) return;
		this.#disconnect = new AbortController();
		const { signal } = this.#disconnect;
		let pointerDownTime = 0;
		listen$1(this.#target, "pointerdown", (event) => {
			if (isInteractionLocked(this.#target)) {
				pointerDownTime = 0;
				return;
			}
			if (event.button !== 0) return;
			pointerDownTime = Date.now();
			for (const binding of this.#bindings) binding.onPointerDown?.();
		}, { signal });
		listen$1(this.#target, "pointerup", (event) => {
			if (isInteractionLocked(this.#target)) {
				pointerDownTime = 0;
				return;
			}
			if (event.button !== 0) return;
			if (Date.now() - pointerDownTime > TAP_THRESHOLD$1) return;
			if (isInteractiveTarget(event)) return;
			const pointerType = event.pointerType;
			const clientX = event.clientX;
			const target = this.#target;
			const bindings = this.#bindings;
			const matches = { resolve: (type) => matchBindings(bindings, type, pointerType, clientX, target) };
			for (const recognizer of this.#recognizers) recognizer.handleUp(matches, event);
		}, { signal });
	}
	#maybeDisconnect() {
		if (this.#bindings.length > 0) return;
		for (const recognizer of this.#recognizers) recognizer.reset();
		this.#recognizers.clear();
		this.#disconnect?.abort();
		this.#disconnect = null;
	}
};
const coordinators = /* @__PURE__ */ new WeakMap();
/**
* Look up the gesture coordinator for a target element, if one exists.
*
* @internal
*/
function findGestureCoordinator(target) {
	return coordinators.get(target);
}
/** @internal */
function getGestureCoordinator(target) {
	let coordinator = coordinators.get(target);
	if (!coordinator) {
		coordinator = new GestureCoordinator(target);
		coordinators.set(target, coordinator);
	}
	return coordinator;
}
function matchBindings(bindings, type, pointerType, clientX, target) {
	const rect = target.getBoundingClientRect();
	const activeRegions = getActiveRegions(bindings, type, pointerType);
	const region = activeRegions.size > 0 ? resolveRegion(clientX, rect, activeRegions) : null;
	const matches = [];
	for (const binding of bindings) {
		if (binding.disabled) continue;
		if (binding.type !== type) continue;
		if (binding.pointer && binding.pointer !== pointerType) continue;
		if (binding.region) {
			if (binding.region !== region) continue;
		} else if (region !== null) continue;
		matches.push(binding);
	}
	return matches;
}
function getActiveRegions(bindings, type, pointerType) {
	const regions = /* @__PURE__ */ new Set();
	for (const binding of bindings) {
		if (binding.disabled) continue;
		if (binding.type !== type) continue;
		if (binding.pointer && binding.pointer !== pointerType) continue;
		if (binding.region) regions.add(binding.region);
	}
	return regions;
}

//#endregion
//#region ../core/dist/dev/dom/presentation/remote-playback.js
function resolveRemote(media) {
	const target = media;
	if (isObject(target.remote) && "state" in target.remote && "prompt" in target.remote) return target.remote;
}
function isRemotePlaybackConnected(media) {
	return resolveRemote(media)?.state === "connected";
}
function isRemotePlaybackConnecting(media) {
	return resolveRemote(media)?.state === "connecting";
}
async function requestRemotePlayback(media) {
	const remote = resolveRemote(media);
	if (!remote) throw new DOMException("Remote playback not supported", "NotSupportedError");
	return remote.prompt();
}

//#endregion
//#region ../core/dist/dev/dom/store/features/controls.js
const IDLE_DELAY = 2e3;
const TAP_THRESHOLD = 250;
const TOUCH_SETTLE_DELAY = 500;
/** How long after a control releases pointer capture a `mouseleave` inside the container is treated as spurious. */
const CAPTURE_RELEASE_DELAY = 100;
const controlsActionsByRequest = /* @__PURE__ */ new WeakMap();
const controlsFeature = definePlayerFeature({
	name: "controls",
	state: ({ get, set }) => {
		const fallbackRequestControlsLock = () => {
			set({ controlsVisible: true });
			return () => {};
		};
		const fallbackToggleControls = (forceShow) => {
			const next = forceShow ?? !get().userActive;
			set({
				userActive: next,
				controlsVisible: next
			});
			return next;
		};
		const actions = createControlsActions(fallbackRequestControlsLock, fallbackToggleControls);
		controlsActionsByRequest.set(actions.requestControlsLock, actions);
		return {
			userActive: true,
			controlsVisible: true,
			requestControlsLock: actions.requestControlsLock,
			toggleControls: actions.toggleControls
		};
	},
	attach({ target, signal, get, set }) {
		const { media, container } = target;
		if (!isMediaPauseCapable(media) || isNull(container)) {
			if (isNull(container)) console.warn("[vjs] controlsFeature requires a container element for activity tracking.");
			return;
		}
		let idleTimer;
		let controlsLockCount = 0;
		const computeVisible = (userActive) => {
			return controlsLockCount > 0 || userActive || media.paused || isRemotePlaybackConnected(media) || isRemotePlaybackConnecting(media);
		};
		function clearIdle() {
			clearTimeout(idleTimer);
			idleTimer = void 0;
		}
		function scheduleIdle() {
			clearIdle();
			if (controlsLockCount > 0) return;
			idleTimer = setTimeout(setInactive, IDLE_DELAY);
		}
		function setActive() {
			if (!get().userActive) set({
				userActive: true,
				controlsVisible: true
			});
			scheduleIdle();
		}
		function setInactive() {
			clearIdle();
			set({
				userActive: false,
				controlsVisible: computeVisible(false)
			});
		}
		function requestControlsLock() {
			controlsLockCount++;
			clearIdle();
			if (!get().controlsVisible) set({ controlsVisible: true });
			let released = false;
			return () => {
				if (released || signal.aborted) return;
				released = true;
				controlsLockCount--;
				if (controlsLockCount === 0) setActive();
			};
		}
		function toggleControls(forceShow) {
			if (forceShow ?? !get().controlsVisible) setActive();
			else setInactive();
			return get().controlsVisible;
		}
		const actions = controlsActionsByRequest.get(get().requestControlsLock);
		actions.setDelegates(requestControlsLock, toggleControls);
		let pointerDownTime = 0;
		let lastTouchAt = 0;
		const isRecentTouch = () => lastTouchAt > 0 && Date.now() - lastTouchAt < TOUCH_SETTLE_DELAY;
		let lastCaptureReleaseAt = 0;
		const isRecentCaptureRelease = () => lastCaptureReleaseAt > 0 && Date.now() - lastCaptureReleaseAt < CAPTURE_RELEASE_DELAY;
		function onPointerDown(event) {
			pointerDownTime = Date.now();
			if (event.pointerType === "touch") lastTouchAt = pointerDownTime;
		}
		function onPointerUp(event) {
			if (event.pointerType === "touch") lastTouchAt = Date.now();
			if (event.pointerType === "touch" && Date.now() - pointerDownTime < TAP_THRESHOLD) {
				if (findGestureCoordinator(container)?.claimsTap(event, "toggleControls")) return;
				const isMediaOrContainer = [getRegisteredMedia(media), container].includes(event.target);
				if (get().controlsVisible && isMediaOrContainer) setInactive();
				else setActive();
			} else setActive();
		}
		const onPlaybackChange = () => {
			const { userActive } = get();
			set({ controlsVisible: computeVisible(userActive) });
			if (!media.paused && userActive) scheduleIdle();
		};
		function onPointerMove(event) {
			if (event.pointerType === "touch") {
				if (get().userActive) scheduleIdle();
				return;
			}
			setActive();
		}
		listen$1(container, "pointermove", onPointerMove, { signal });
		listen$1(container, "pointerdown", onPointerDown, { signal });
		listen$1(container, "pointerup", onPointerUp, { signal });
		listen$1(container, "lostpointercapture", () => {
			lastCaptureReleaseAt = Date.now();
		}, { signal });
		listen$1(container, "keydown", setActive, { signal });
		listen$1(container, "keyup", setActive, { signal });
		listen$1(container, "focusin", () => {
			if (isRecentTouch()) return;
			setActive();
		}, { signal });
		listen$1(container, "mouseleave", (event) => {
			if (isRecentTouch()) return;
			if (isRecentCaptureRelease() && event instanceof MouseEvent && isPointInElement(container, event)) return;
			setInactive();
		}, { signal });
		listen$1(media, "play", onPlaybackChange, { signal });
		listen$1(media, "pause", onPlaybackChange, { signal });
		listen$1(media, "ended", onPlaybackChange, { signal });
		if (isMediaRemotePlaybackCapable(media)) {
			const onCastChange = () => {
				const { userActive } = get();
				set({ controlsVisible: computeVisible(userActive) });
			};
			listen$1(media.remote, "connect", onCastChange, { signal });
			listen$1(media.remote, "connecting", onCastChange, { signal });
			listen$1(media.remote, "disconnect", onCastChange, { signal });
		}
		signal.addEventListener("abort", () => {
			actions.reset();
			controlsLockCount = 0;
			clearIdle();
		}, { once: true });
		scheduleIdle();
	}
});
function createControlsActions(fallbackRequestControlsLock, fallbackToggleControls) {
	let requestControlsLockDelegate = fallbackRequestControlsLock;
	let toggleControlsDelegate = fallbackToggleControls;
	const locks = /* @__PURE__ */ new Set();
	const requestControlsLock = () => {
		const lock = { release: requestControlsLockDelegate() };
		let released = false;
		locks.add(lock);
		return () => {
			if (released) return;
			released = true;
			locks.delete(lock);
			lock.release();
		};
	};
	const toggleControls = (forceShow) => toggleControlsDelegate(forceShow);
	const actions = {
		requestControlsLock,
		toggleControls,
		setDelegates(nextRequestControlsLock, nextToggleControls) {
			if (nextRequestControlsLock !== requestControlsLockDelegate) {
				requestControlsLockDelegate = nextRequestControlsLock;
				for (const lock of locks) {
					lock.release();
					lock.release = nextRequestControlsLock();
				}
			}
			toggleControlsDelegate = nextToggleControls;
		},
		reset() {
			actions.setDelegates(fallbackRequestControlsLock, fallbackToggleControls);
		}
	};
	return actions;
}

//#endregion
//#region ../core/dist/dev/dom/presentation/fullscreen.js
function isFullscreenEnabled() {
	const doc = document;
	if (doc.fullscreenEnabled || doc.webkitFullscreenEnabled) return true;
	const video = document.createElement("video");
	return isFunction(video.webkitSetPresentationMode);
}
function getFullscreenElement() {
	const doc = document;
	return doc.fullscreenElement ?? doc.webkitFullscreenElement ?? null;
}
function matchesFullscreen(element) {
	if (!(element instanceof Element)) return false;
	try {
		return element.matches(":fullscreen");
	} catch {
		return false;
	}
}
function isFullscreen(container, media) {
	if (media.webkitPresentationMode === "fullscreen") return true;
	const fullscreenElement = getFullscreenElement();
	if (fullscreenElement && (fullscreenElement === container || fullscreenElement === getRegisteredMedia(media))) return true;
	if (matchesFullscreen(container) || matchesFullscreen(media)) return true;
	return media.isFullscreen ?? false;
}
async function requestFullscreen(container, media) {
	const doc = document;
	if (container && (doc.fullscreenEnabled || doc.webkitFullscreenEnabled)) {
		const el = container;
		hideOpenPopovers(container);
		if (isFunction(el.requestFullscreen)) return el.requestFullscreen();
		if (isFunction(el.webkitRequestFullscreen)) return el.webkitRequestFullscreen();
	}
	const webkitVideo = media;
	if (isFunction(webkitVideo.webkitSetPresentationMode)) {
		webkitVideo.webkitSetPresentationMode("fullscreen");
		return;
	}
	const video = media;
	if (isFunction(video.requestFullscreen)) return video.requestFullscreen();
}
async function exitFullscreen(media) {
	const doc = document;
	const webkitVideo = media;
	if (webkitVideo.webkitPresentationMode === "fullscreen" && isFunction(webkitVideo.webkitSetPresentationMode)) {
		webkitVideo.webkitSetPresentationMode("inline");
		return;
	}
	if (isFunction(doc.exitFullscreen)) return doc.exitFullscreen();
	if (isFunction(doc.webkitExitFullscreen)) return doc.webkitExitFullscreen();
	const video = media;
	if (isFunction(video.exitFullscreen)) return video.exitFullscreen();
}

//#endregion
//#region ../core/dist/dev/dom/presentation/pip.js
function isPictureInPictureEnabled() {
	if (document.pictureInPictureEnabled) {
		const isSafari = /.*Version\/.*Safari\/.*/.test(navigator.userAgent);
		const isPWA = typeof matchMedia === "function" && matchMedia("(display-mode: standalone)").matches;
		return !isSafari || !isPWA;
	}
	const video = document.createElement("video");
	return isFunction(video.webkitSetPresentationMode);
}
/**
* Whether this media can enter picture-in-picture at all, which is a separate question from whether the browser
* supports it. Mirrors the branches `requestPictureInPicture` takes below, so anything it would refuse to act on
* reports as incapable here — an iframe embed whose provider has no picture-in-picture can never enter it, however
* capable the browser is.
*/
function isPictureInPictureCapable(media) {
	if (isFunction(media.webkitSetPresentationMode)) return true;
	return isMediaPictureInPictureCapable(media);
}
function isPictureInPicture(media) {
	if (media.webkitPresentationMode === "picture-in-picture") return true;
	if (document.pictureInPictureElement === getRegisteredMedia(media)) return true;
	return media.isPictureInPicture ?? false;
}
async function requestPictureInPicture(media) {
	const webkitVideo = media;
	if (isFunction(webkitVideo.webkitSetPresentationMode)) {
		webkitVideo.webkitSetPresentationMode("picture-in-picture");
		return;
	}
	const video = media;
	if (isFunction(video.requestPictureInPicture)) return video.requestPictureInPicture();
}
async function exitPictureInPicture(media) {
	const webkitVideo = media;
	if (webkitVideo.webkitPresentationMode === "picture-in-picture" && isFunction(webkitVideo.webkitSetPresentationMode)) {
		webkitVideo.webkitSetPresentationMode("inline");
		return;
	}
	if (isFunction(document.exitPictureInPicture)) return document.exitPictureInPicture();
	const video = media;
	if (isFunction(video.exitPictureInPicture)) return video.exitPictureInPicture();
}

//#endregion
//#region ../core/dist/dev/dom/store/features/fullscreen.js
const fullscreenFeature = definePlayerFeature({
	name: "fullscreen",
	state: ({ target }) => ({
		isFullscreen: false,
		fullscreenAvailability: "unavailable",
		async requestFullscreen() {
			const { media, container } = target();
			if (isPictureInPicture(media)) await exitPictureInPicture(media);
			return requestFullscreen(container, media);
		},
		async exitFullscreen() {
			const { media } = target();
			return exitFullscreen(media);
		}
	}),
	attach({ target, signal, set }) {
		const { media, container } = target;
		set({ fullscreenAvailability: isFullscreenEnabled() ? "available" : "unsupported" });
		const sync = () => set({ isFullscreen: isFullscreen(container, media) });
		sync();
		listen$1(document, "fullscreenchange", sync, { signal });
		listen$1(document, "webkitfullscreenchange", sync, { signal });
		if ("webkitPresentationMode" in media) listen$1(media, "webkitpresentationmodechanged", sync, { signal });
	}
});

//#endregion
//#region ../core/dist/dev/dom/store/features/pip.js
const pipFeature = definePlayerFeature({
	name: "pip",
	state: ({ target }) => ({
		isPictureInPicture: false,
		pictureInPictureAvailability: "unavailable",
		async requestPictureInPicture() {
			const { media, container } = target();
			if (!isPictureInPictureCapable(media)) return;
			if (!isMediaSourceCapable(media) || !hasMetadata(media)) throw new DOMException("The media has no video data yet.", "InvalidStateError");
			if (isFullscreen(container, media)) await exitFullscreen(media);
			return requestPictureInPicture(media);
		},
		async exitPictureInPicture() {
			const { media } = target();
			return exitPictureInPicture(media);
		}
	}),
	attach({ target, signal, set }) {
		const { media } = target;
		const supported = isPictureInPictureEnabled() && isPictureInPictureCapable(media);
		const sync = () => set({
			isPictureInPicture: isPictureInPicture(media),
			pictureInPictureAvailability: supported ? isMediaSourceCapable(media) && hasMetadata(media) ? "available" : "unavailable" : "unsupported"
		});
		sync();
		listen$1(media, "enterpictureinpicture", sync, { signal });
		listen$1(media, "leavepictureinpicture", sync, { signal });
		listen$1(media, "loadstart", sync, { signal });
		listen$1(media, "loadedmetadata", sync, { signal });
		listen$1(media, "emptied", sync, { signal });
		if ("webkitPresentationMode" in media) listen$1(media, "webkitpresentationmodechanged", sync, { signal });
	}
});

//#endregion
//#region ../core/dist/dev/dom/store/features/remote-playback.js
const remotePlaybackFeature = definePlayerFeature({
	name: "remotePlayback",
	state: ({ target }) => ({
		remotePlaybackState: "disconnected",
		remotePlaybackAvailability: "unsupported",
		async promptRemotePlayback() {
			const { media, container } = target();
			if (isRemotePlaybackConnected(media)) return requestRemotePlayback(media);
			if (isFullscreen(container, media)) await exitFullscreen(media);
			return await requestRemotePlayback(media);
		}
	}),
	attach({ target, signal, set }) {
		const { media } = target;
		if (!isMediaRemotePlaybackCapable(media)) return;
		if (media.disableRemotePlayback === true) return;
		if (isWebKitAirPlayCapable(media)) {
			const syncConnection = () => {
				set({ remotePlaybackState: media.webkitCurrentPlaybackTargetIsWireless ? "connected" : "disconnected" });
			};
			const syncAvailability = (event) => {
				const { availability } = event;
				set({ remotePlaybackAvailability: availability === "available" ? "available" : "unavailable" });
			};
			listen$1(media, "webkitplaybacktargetavailabilitychanged", syncAvailability, { signal });
			listen$1(media, "webkitcurrentplaybacktargetiswirelesschanged", syncConnection, { signal });
			syncConnection();
			return;
		}
		const syncState = () => set({ remotePlaybackState: media.remote.state });
		syncState();
		listen$1(media.remote, "connect", syncState, { signal });
		listen$1(media.remote, "connecting", syncState, { signal });
		listen$1(media.remote, "disconnect", syncState, { signal });
		media.remote.watchAvailability((available) => {
			set({ remotePlaybackAvailability: available ? "available" : "unavailable" });
		}).catch(() => {
			set({ remotePlaybackAvailability: "unsupported" });
		});
		signal.addEventListener("abort", () => {
			media.remote?.cancelWatchAvailability?.()?.catch(noop);
		});
	}
});

//#endregion
//#region ../core/dist/dev/core/store/features/audio-track.js
function getTrackValue(track, index) {
	return track.id || String(index);
}
function toMediaTrack(track, index) {
	return {
		id: getTrackValue(track, index),
		...track.kind !== void 0 && { kind: track.kind },
		label: track.label,
		language: track.language,
		enabled: track.enabled
	};
}
/** @internal */
const audioTrackFeature$1 = definePlayerFeature$1({
	name: "audioTrack",
	state: ({ target }) => ({
		audioTrackList: [],
		selectAudioTrack(value) {
			const { media } = target();
			if (!isMediaAudioTrackCapable(media)) return;
			const tracks = [...media.audioTracks];
			const track = tracks.find((candidate, index) => getTrackValue(candidate, index) === value);
			if (!track) return;
			for (const candidate of tracks) candidate.enabled = candidate === track;
		}
	}),
	attach({ target, signal, set }) {
		const { media } = target;
		let audioTracks = null;
		let cleanup = null;
		const getAudioTracks = () => isMediaAudioTrackCapable(media) ? media.audioTracks : null;
		const sync = (list = getAudioTracks()) => {
			set({ audioTrackList: list ? [...list].map(toMediaTrack) : [] });
		};
		const bind = () => {
			const nextAudioTracks = getAudioTracks();
			if (nextAudioTracks === audioTracks) {
				sync(nextAudioTracks);
				return;
			}
			cleanup?.abort();
			cleanup = new AbortController();
			audioTracks = nextAudioTracks;
			if (audioTracks) {
				listen(audioTracks, "addtrack", () => sync(audioTracks), { signal: cleanup.signal });
				listen(audioTracks, "removetrack", () => sync(audioTracks), { signal: cleanup.signal });
				listen(audioTracks, "change", () => sync(audioTracks), { signal: cleanup.signal });
			}
			sync(audioTracks);
		};
		bind();
		listen(media, "loadstart", bind, { signal });
		signal.addEventListener("abort", () => cleanup?.abort(), { once: true });
	}
});

//#endregion
//#region ../core/dist/dev/core/store/features/buffer.js
/** @internal */
const bufferFeature$1 = definePlayerFeature$1({
	name: "buffer",
	state: () => ({
		buffered: [],
		seekable: []
	}),
	attach({ target, signal, set }) {
		const { media } = target;
		if (!isMediaBufferCapable(media)) return;
		const sync = () => set({
			buffered: serializeTimeRanges(media.buffered),
			seekable: serializeTimeRanges(media.seekable)
		});
		sync();
		listen(media, "loadedmetadata", sync, { signal });
		listen(media, "durationchange", sync, { signal });
		listen(media, "progress", sync, { signal });
		listen(media, "emptied", sync, { signal });
	}
});

//#endregion
//#region ../core/dist/dev/core/store/features/error.js
/** @internal */
const errorFeature$1 = definePlayerFeature$1({
	name: "error",
	state: ({ set }) => ({
		error: null,
		dismissError() {
			set({ error: null });
		}
	}),
	attach({ target, signal, set }) {
		const { media } = target;
		if (!isMediaErrorCapable(media)) return;
		const syncError = () => set({ error: media.error });
		syncError();
		listen(media, "error", syncError, { signal });
		listen(media, "emptied", () => set({ error: null }), { signal });
	}
});

//#endregion
//#region ../core/dist/dev/core/store/features/live.js
/**
* Player feature exposing `liveEdgeStart` and `targetLiveWindow` in store state for media that implements
* `MediaLiveCapability` (currently `HlsJsAdapter` and its delegates).
*
* - `liveEdgeStart` — presentation time marking the start of the Live Edge Window. Playing at the live edge when
*   `currentTime >= liveEdgeStart`. `NaN` when the stream isn't live or the value is unknown.
* - `targetLiveWindow` — `0` for standard latency live, `Infinity` for DVR, `NaN` for on-demand or unknown.
*
* Included by the {@link liveVideoFeatures} and {@link liveAudioFeatures} presets; apps can also compose it into a
* custom preset.
*
* @internal
* @see https://github.com/video-dev/media-ui-extensions/blob/main/proposals/0007-live-edge.md
*/
const liveFeature$1 = definePlayerFeature$1({
	name: "live",
	state: () => ({
		liveEdgeStart: NaN,
		targetLiveWindow: NaN
	}),
	attach({ target, signal, set }) {
		const { media } = target;
		if (!isMediaLiveCapable(media)) return;
		const sync = () => set({
			liveEdgeStart: media.liveEdgeStart,
			targetLiveWindow: media.targetLiveWindow
		});
		sync();
		listen(media, "targetlivewindowchange", sync, { signal });
		listen(media, "streamtypechange", sync, { signal });
		listen(media, "loadedmetadata", sync, { signal });
		listen(media, "canplay", sync, { signal });
		listen(media, "progress", sync, { signal });
		listen(media, "durationchange", sync, { signal });
		listen(media, "timeupdate", sync, { signal });
		listen(media, "emptied", sync, { signal });
	}
});

//#endregion
//#region ../core/dist/dev/core/store/features/metadata.js
const MEDIA_TITLE = Symbol("@videojs/media-title");
const DEFAULT_TITLE = "";
const MEDIA_POSTER = Symbol("@videojs/media-poster");
const DEFAULT_POSTER = "";
const USER_TITLE = Symbol("@videojs/user-title");
const SET_USER_TITLE = Symbol("@videojs/set-user-title");
const USER_POSTER = Symbol("@videojs/user-poster");
const SET_USER_POSTER = Symbol("@videojs/set-user-poster");
/**
* Resolves content metadata into player state, preferring what the author set over what the media carries. Included in
* the standard audio, video, and live presets.
*
* @internal
*/
const metadataFeature$1 = definePlayerFeature$1({
	name: "metadata",
	config: {
		/** The title to display. Takes precedence over the title the media carries. */
		title: {
			action: SET_USER_TITLE,
			state: USER_TITLE,
			html: { attribute: "content-title" }
		},
		/** The poster to display. Takes precedence over the poster the media carries. */
		poster: {
			action: SET_USER_POSTER,
			state: USER_POSTER
		}
	},
	state: ({ set }) => ({
		[MEDIA_TITLE]: void 0,
		[USER_TITLE]: void 0,
		[SET_USER_TITLE]: (value) => set({ [USER_TITLE]: value }),
		[MEDIA_POSTER]: void 0,
		[USER_POSTER]: void 0,
		[SET_USER_POSTER]: (value) => set({ [USER_POSTER]: value })
	}),
	derived: {
		/** The resolved content title. Set it through the player, not through the store. */
		title: ({ get }) => get()[USER_TITLE] ?? get()[MEDIA_TITLE] ?? DEFAULT_TITLE,
		/**
		* The resolved poster URL, independent of the media component's own `poster`. Set it through the player, not
		* through the store.
		*/
		poster: ({ get }) => get()[USER_POSTER] ?? get()[MEDIA_POSTER] ?? DEFAULT_POSTER
	},
	attach({ target, signal, set }) {
		const { media } = target;
		const sync = () => {
			const contentData = isMediaContentDataCapable(media) ? media.contentData : void 0;
			set({
				[MEDIA_TITLE]: contentData?.title,
				[MEDIA_POSTER]: contentData?.poster
			});
		};
		const bind = () => {
			sync();
			if (!isMediaContentDataCapable(media)) return;
			listen(media, "contentdatachange", sync, { signal });
		};
		bind();
		listen(media, "loadstart", bind, { signal });
	}
});

//#endregion
//#region ../core/dist/dev/core/ui/utils/playback-rates.js
const NATIVE_PLAYBACK_RATE_RANGE = {
	min: .0625,
	max: 16
};
/** The rate range a media reports, or the native range when it reports none or there is no media yet. */
function getPlaybackRateRange(media) {
	return media?.playbackRateRange ?? NATIVE_PLAYBACK_RATE_RANGE;
}
/**
* Normalizes author input into a list of playback rates. Accepts an array or a space-separated attribute value, keeps
* the author's order, and drops invalid and repeated rates. Returns `undefined` when no usable rate is left, so the
* caller can fall back to its own list.
*/
function parsePlaybackRates(value) {
	const candidates = isString(value) ? value.trim().split(/\s+/).map(Number) : value ?? [];
	const rates = [...new Set(candidates.filter((rate) => Number.isFinite(rate) && rate > 0))];
	return rates.length > 0 ? rates : void 0;
}
/**
* Slots `rate` into `rates` before the first faster rate, or at the end when none is faster. Returns `rates` unchanged
* when it already holds `rate`.
*/
function insertPlaybackRate(rates, rate) {
	if (rates.includes(rate)) return rates;
	const index = rates.findIndex((candidate) => candidate > rate);
	if (index === -1) return [...rates, rate];
	return [
		...rates.slice(0, index),
		rate,
		...rates.slice(index)
	];
}
/** Keeps the rates that fall within `range`, in their order. */
function filterPlaybackRates(rates, { min, max }) {
	return rates.filter((rate) => rate >= min && rate <= max);
}
/**
* Drops floating-point noise from a rate: browsers store `playbackRate` in single precision, so `1.1` reads back as
* `1.100000023841858`, and stepped rates such as `1 + 0.1` land just off the value. Five decimal places keeps every
* rate within the native range intact.
*/
function roundPlaybackRate(rate) {
	return Math.round(rate * 1e5) / 1e5;
}

//#endregion
//#region ../core/dist/dev/core/store/features/playback-rate.js
const DEFAULT_RATES = [
	.5,
	.75,
	1,
	1.25,
	1.5,
	1.75,
	2
];
const DEFAULT_PLAYBACK_RATES = Symbol("@videojs/default-playback-rates");
const SET_DEFAULT_PLAYBACK_RATES = Symbol("@videojs/set-default-playback-rates");
const PLAYBACK_RATE_RANGE = Symbol("@videojs/playback-rate-range");
/**
* Narrows the default rates to the media's range, keeping their order. When none fit, falls back to the built-in
* defaults that fit, then to the rate nearest `1`, so a control always has a rate to offer.
*/
function fitPlaybackRates(rates, range) {
	const fitted = filterPlaybackRates(rates, range);
	if (fitted.length > 0) return fitted;
	const defaults = filterPlaybackRates(DEFAULT_RATES, range);
	if (defaults.length > 0) return defaults;
	return [clamp(1, range.min, range.max)];
}
function warnUnfitPlaybackRates(rates, range) {
	if (filterPlaybackRates(rates, range).length > 0) return;
	console.warn(`[vjs] None of the default playback rates ${rates.join(", ")} is within the media's range of ${range.min} to ${range.max}; using ${fitPlaybackRates(rates, range).join(", ")}.`);
}
/** @internal */
const playbackRateFeature$1 = definePlayerFeature$1({
	name: "playbackRate",
	config: { 
	/**
	* Rates the player's controls offer, in the order they cycle through them, unless a control sets its own `rates`.
	* HTML takes a space-separated list, such as `default-playback-rates="0.5 1 1.5 2"`. Missing or empty input
	* restores the defaults. Rates outside the media's range are left out, so YouTube, which plays up to `2`, may offer
	* fewer.
	*/
defaultPlaybackRates: {
		action: SET_DEFAULT_PLAYBACK_RATES,
		state: DEFAULT_PLAYBACK_RATES
	} },
	state: ({ target, get, set }) => ({
		[DEFAULT_PLAYBACK_RATES]: DEFAULT_RATES,
		[SET_DEFAULT_PLAYBACK_RATES]: (value) => {
			const rates = parsePlaybackRates(value) ?? DEFAULT_RATES;
			set({ [DEFAULT_PLAYBACK_RATES]: rates });
			warnUnfitPlaybackRates(rates, get()[PLAYBACK_RATE_RANGE]);
		},
		[PLAYBACK_RATE_RANGE]: NATIVE_PLAYBACK_RATE_RANGE,
		playbackRate: 1,
		playbackRateAvailability: "unavailable",
		setPlaybackRate(rate) {
			const { media } = target();
			if (!isMediaPlaybackRateCapable(media)) return;
			const { min, max } = getPlaybackRateRange(media);
			const next = clamp(rate, min, max);
			if (next !== rate) console.warn(`[vjs] Playback rate ${rate} is outside the media's range of ${min} to ${max}; using ${next}.`);
			media.playbackRate = next;
		}
	}),
	derived: { 
	/**
	* The default rates that fit the media's range. Set them through the player, not through the store. They are listed
	* even when the media cannot change its rate, so check `playbackRateAvailability` before offering them.
	*/
playbackRates: ({ get }) => fitPlaybackRates(get()[DEFAULT_PLAYBACK_RATES], get()[PLAYBACK_RATE_RANGE]) },
	attach({ target, signal, get, set }) {
		const { media } = target;
		const syncRange = () => {
			const range = getPlaybackRateRange(media);
			const current = get()[PLAYBACK_RATE_RANGE];
			if (range.min === current.min && range.max === current.max) return;
			set({ [PLAYBACK_RATE_RANGE]: range });
			warnUnfitPlaybackRates(get()[DEFAULT_PLAYBACK_RATES], range);
		};
		syncRange();
		listen(media, "loadstart", syncRange, { signal });
		if (!isMediaPlaybackRateCapable(media)) {
			set({
				playbackRate: 1,
				playbackRateAvailability: "unsupported"
			});
			return;
		}
		const sync = () => set({ playbackRate: roundPlaybackRate(media.playbackRate) });
		set({ playbackRateAvailability: "available" });
		sync();
		listen(media, "ratechange", sync, { signal });
	}
});

//#endregion
//#region ../core/dist/dev/core/store/features/playback.js
/** @internal */
const playbackFeature$1 = definePlayerFeature$1({
	name: "playback",
	state: ({ target, set }) => ({
		paused: true,
		ended: false,
		started: false,
		waiting: false,
		autoplay: false,
		autoplaying: false,
		play() {
			const { media } = target();
			const playing = media.play();
			if (isMediaPauseCapable(media) && !media.paused) set({
				paused: false,
				ended: false,
				started: true
			});
			return playing;
		},
		pause() {
			const { media } = target();
			if (isMediaPauseCapable(media)) {
				media.pause();
				set({
					paused: media.paused,
					autoplaying: false
				});
			}
		}
	}),
	attach({ target, signal, get, set }) {
		const { media } = target;
		if (!isMediaPauseCapable(media) || !isMediaSeekCapable(media) || !isMediaSourceCapable(media)) return;
		const autoplayEnabled = () => isMediaAutoplayCapable(media) && media.autoplay;
		let autoplayCheck;
		const cancelAutoplayCheck = () => {
			if (autoplayCheck !== void 0) cancelAnimationFrame(autoplayCheck);
			autoplayCheck = void 0;
		};
		const startAutoplay = () => {
			cancelAutoplayCheck();
			const autoplaying = autoplayEnabled() && !!(media.currentSrc || media.src) && media.paused && !media.ended && media.currentTime === 0 && media.readyState < MediaReadyState.HAVE_ENOUGH_DATA;
			set({
				autoplay: autoplayEnabled(),
				autoplaying
			});
		};
		const refuseAutoplay = () => {
			if (get().autoplaying) set({ autoplaying: false });
		};
		const checkAutoplay = () => {
			if (!get().autoplaying) return;
			cancelAutoplayCheck();
			autoplayCheck = requestAnimationFrame(() => {
				autoplayCheck = requestAnimationFrame(() => {
					autoplayCheck = void 0;
					if (media.paused) refuseAutoplay();
				});
			});
		};
		signal.addEventListener("abort", cancelAutoplayCheck);
		let starvedAt = null;
		const sync = () => {
			const starved = media.readyState < MediaReadyState.HAVE_FUTURE_DATA && !media.paused;
			if (!starved) starvedAt = null;
			else starvedAt ??= media.currentTime;
			const started = !media.paused || media.currentTime > 0;
			set({
				paused: media.paused,
				ended: media.ended,
				started,
				waiting: starved && starvedAt === media.currentTime,
				autoplay: autoplayEnabled(),
				autoplaying: get().autoplaying && !started && !media.ended
			});
		};
		const starve = () => {
			starvedAt = media.currentTime;
			sync();
		};
		startAutoplay();
		sync();
		listen(media, "emptied", startAutoplay, { signal });
		listen(media, "loadstart", startAutoplay, { signal });
		listen(media, "canplaythrough", checkAutoplay, { signal });
		listen(media, "pause", refuseAutoplay, { signal });
		listen(media, "error", refuseAutoplay, { signal });
		listen(media, "emptied", starve, { signal });
		listen(media, "play", starve, { signal });
		listen(media, "pause", sync, { signal });
		listen(media, "ended", sync, { signal });
		listen(media, "playing", sync, { signal });
		listen(media, "waiting", starve, { signal });
		listen(media, "seeking", starve, { signal });
		listen(media, "seeked", sync, { signal });
		listen(media, "canplay", sync, { signal });
		listen(media, "timeupdate", sync, { signal });
	}
});

//#endregion
//#region ../core/dist/dev/core/store/features/quality.js
const QUALITY_AUTO_VALUE = "auto";
function getRenditionValue(rendition, index) {
	return rendition.id || String(index);
}
function toMediaRendition(rendition, index) {
	return {
		id: getRenditionValue(rendition, index),
		...rendition.width !== void 0 && { width: rendition.width },
		...rendition.height !== void 0 && { height: rendition.height },
		...rendition.bitrate !== void 0 && { bitrate: rendition.bitrate },
		...rendition.frameRate !== void 0 && { frameRate: rendition.frameRate },
		...rendition.codec !== void 0 && { codec: rendition.codec },
		selected: rendition.selected
	};
}
function getSize(rendition) {
	if (rendition.width && rendition.height) return Math.min(rendition.width, rendition.height);
	return rendition.height ?? rendition.width;
}
/** @internal */
const qualityFeature$1 = definePlayerFeature$1({
	name: "quality",
	state: ({ target }) => ({
		videoRenditionList: [],
		activeVideoRendition: null,
		selectVideoRendition(value) {
			const { media } = target();
			if (!isMediaVideoRenditionCapable(media)) return;
			if (value === QUALITY_AUTO_VALUE) {
				media.videoRenditions.selectedIndex = -1;
				return;
			}
			const index = [...media.videoRenditions].findIndex((rendition, renditionIndex) => getRenditionValue(rendition, renditionIndex) === value);
			if (index !== -1) media.videoRenditions.selectedIndex = index;
		}
	}),
	attach({ target, signal, set }) {
		const { media } = target;
		let videoRenditions = null;
		let cleanup = null;
		const getVideoRenditions = () => isMediaVideoRenditionCapable(media) ? media.videoRenditions : null;
		const getActiveRendition = (list) => {
			if (!list) return null;
			const renditions = [...list];
			const active = renditions.find((rendition) => rendition.active);
			if (active) return active;
			if (!isMediaVideoDimensionsCapable(media) || !media.videoWidth && !media.videoHeight) return null;
			const size = getSize({
				width: media.videoWidth || void 0,
				height: media.videoHeight || void 0
			});
			const matches = renditions.filter((rendition) => getSize(rendition) === size);
			return matches.length === 1 ? matches[0] : null;
		};
		const sync = (list = getVideoRenditions()) => {
			const renditions = list ? [...list] : [];
			const active = getActiveRendition(list);
			set({
				videoRenditionList: renditions.map(toMediaRendition),
				activeVideoRendition: active ? toMediaRendition(active, renditions.indexOf(active)) : null
			});
		};
		const bind = () => {
			const nextVideoRenditions = getVideoRenditions();
			if (nextVideoRenditions === videoRenditions) {
				sync(nextVideoRenditions);
				return;
			}
			cleanup?.abort();
			cleanup = new AbortController();
			videoRenditions = nextVideoRenditions;
			if (videoRenditions) {
				listen(videoRenditions, "addrendition", () => sync(videoRenditions), { signal: cleanup.signal });
				listen(videoRenditions, "removerendition", () => sync(videoRenditions), { signal: cleanup.signal });
				listen(videoRenditions, "change", () => sync(videoRenditions), { signal: cleanup.signal });
				listen(videoRenditions, "activechange", () => sync(videoRenditions), { signal: cleanup.signal });
			}
			sync(videoRenditions);
		};
		bind();
		listen(media, "loadstart", bind, { signal });
		listen(media, "resize", () => sync(videoRenditions), { signal });
		signal.addEventListener("abort", () => cleanup?.abort(), { once: true });
	}
});

//#endregion
//#region ../core/dist/dev/core/store/features/source.js
/** @internal */
const sourceFeature$1 = definePlayerFeature$1({
	name: "source",
	state: () => ({
		currentSrc: "",
		canPlay: false
	}),
	attach({ target, signal, set }) {
		const { media } = target;
		if (!isMediaSourceCapable(media)) return;
		const sync = () => set({
			currentSrc: media.currentSrc,
			canPlay: media.readyState >= MediaReadyState.HAVE_FUTURE_DATA
		});
		sync();
		listen(media, "canplay", sync, { signal });
		listen(media, "canplaythrough", sync, { signal });
		listen(media, "loadstart", sync, { signal });
		listen(media, "emptied", sync, { signal });
	}
});

//#endregion
//#region ../core/dist/dev/core/store/features/stream-type.js
/** @internal */
const streamTypeFeature$1 = definePlayerFeature$1({
	name: "streamType",
	state: () => ({ streamType: MediaStreamTypes.UNKNOWN }),
	attach({ target, signal, set }) {
		const { media } = target;
		if (isMediaStreamTypeCapable(media)) {
			const sync = () => set({ streamType: media.streamType });
			sync();
			listen(media, "streamtypechange", sync, { signal });
			return;
		}
		if (!isMediaSeekCapable(media)) return;
		const detect = () => {
			const { duration } = media;
			if (duration === Number.POSITIVE_INFINITY) return MediaStreamTypes.LIVE;
			if (Number.isFinite(duration) && duration > 0) return MediaStreamTypes.ON_DEMAND;
			return MediaStreamTypes.UNKNOWN;
		};
		const sync = () => set({ streamType: detect() });
		sync();
		listen(media, "durationchange", sync, { signal });
		listen(media, "loadedmetadata", sync, { signal });
		listen(media, "emptied", sync, { signal });
		if (isMediaBufferCapable(media)) listen(media, "progress", sync, { signal });
	}
});

//#endregion
//#region ../core/dist/dev/core/ui/utils/animation-frame.js
/** Schedule `callback` for the next animation frame and return its cancel function. */
function nextFrame(callback) {
	const id = requestAnimationFrame(callback);
	return () => cancelAnimationFrame(id);
}

//#endregion
//#region ../core/dist/dev/core/ui/utils/document-visibility.js
/** Whether a browser page is hidden. Platforms without a `document` never are. */
function isDocumentHidden() {
	return globalThis.document?.hidden === true;
}

//#endregion
//#region ../core/dist/dev/core/store/signal-keys.js
const signalKeys = { seek: Symbol.for("@videojs/seek") };

//#endregion
//#region ../core/dist/dev/core/store/features/time.js
/** @internal */
const timeFeature$1 = definePlayerFeature$1({
	name: "time",
	state: ({ target, signals, set }) => ({
		currentTime: 0,
		duration: 0,
		seeking: false,
		async seek(time) {
			const { media } = target(), signal = signals.supersede(signalKeys.seek);
			if (!isMediaSeekCapable(media) || !isMediaSourceCapable(media)) return 0;
			listen(media, "emptied", () => signals.supersede(signalKeys.seek), { signal });
			if (!hasMetadata(media)) {
				if (!await onEvent(media, "loadedmetadata", { signal }).catch(() => false)) return media.currentTime;
			}
			const clampedTime = Math.max(0, Math.min(time, media.duration || Infinity));
			set({
				currentTime: clampedTime,
				seeking: true
			});
			media.currentTime = clampedTime;
			await onEvent(media, "seeked", { signal }).catch(noop);
			return media.currentTime;
		}
	}),
	attach({ target, signal, set, get }) {
		const { media } = target;
		if (!isMediaSeekCapable(media)) return;
		const resolveDuration = () => {
			const { duration } = media;
			if (duration === Number.POSITIVE_INFINITY && isMediaBufferCapable(media)) {
				const { seekable } = media;
				return seekable.length > 0 ? seekable.end(seekable.length - 1) : 0;
			}
			return Number.isFinite(duration) ? duration : 0;
		};
		const sync = () => set({
			currentTime: media.currentTime,
			duration: resolveDuration(),
			seeking: media.seeking
		});
		const syncUnlessSeeking = () => {
			if (get().seeking) return;
			sync();
		};
		let cancel;
		const stop = () => {
			cancel?.();
			cancel = void 0;
		};
		const frame = () => {
			if (!isMediaPauseCapable(media) || media.paused || media.ended) {
				stop();
				return;
			}
			const state = get();
			const currentTime = media.currentTime;
			if (!state.seeking && currentTime !== state.currentTime) set({ currentTime });
			cancel = nextFrame(frame);
		};
		const start = () => {
			if (!cancel && isMediaPauseCapable(media) && !media.paused && !media.ended) cancel = nextFrame(frame);
		};
		const finish = () => {
			stop();
			syncUnlessSeeking();
		};
		sync();
		start();
		signal.addEventListener("abort", stop, { once: true });
		listen(media, "play", start, { signal });
		listen(media, "playing", start, { signal });
		listen(media, "pause", finish, { signal });
		listen(media, "ended", finish, { signal });
		listen(media, "timeupdate", () => {
			if (!cancel || isDocumentHidden()) syncUnlessSeeking();
		}, { signal });
		listen(media, "durationchange", sync, { signal });
		listen(media, "seeking", sync, { signal });
		listen(media, "seeked", sync, { signal });
		listen(media, "loadedmetadata", sync, { signal });
		listen(media, "emptied", () => {
			stop();
			sync();
		}, { signal });
		listen(media, "progress", syncUnlessSeeking, { signal });
	}
});

//#endregion
//#region ../core/dist/dev/dom/store/features/shared.js
/** Re-type a neutral feature for the web target. Identity at runtime. */
function asWebFeature(feature) {
	return feature;
}
const audioTrackFeature = asWebFeature(audioTrackFeature$1);
const bufferFeature = asWebFeature(bufferFeature$1);
const errorFeature = asWebFeature(errorFeature$1);
const liveFeature = asWebFeature(liveFeature$1);
const playbackFeature = asWebFeature(playbackFeature$1);
const qualityFeature = asWebFeature(qualityFeature$1);
const sourceFeature = asWebFeature(sourceFeature$1);
const streamTypeFeature = asWebFeature(streamTypeFeature$1);
const timeFeature = asWebFeature(timeFeature$1);
const metadataFeature = asWebFeature(metadataFeature$1);
const playbackRateFeature = asWebFeature(playbackRateFeature$1);

//#endregion
//#region ../core/dist/dev/core/text-track/filter.js
/**
* Whether a text track matches a filter. Each field the filter gives must match, and a kind list matches any kind in
* it.
*
* @internal
*/
function matchTextTrack(track, filter) {
	if (isFunction(filter)) return filter(track);
	const { kind, label, language } = filter;
	if (!isUndefined(kind) && !(isString(kind) ? kind === track.kind : kind.includes(track.kind))) return false;
	if (!isUndefined(label) && label !== track.label) return false;
	if (!isUndefined(language) && language !== track.language) return false;
	return true;
}

//#endregion
//#region ../core/dist/dev/core/text-track/active-cues.js
/**
* Whether a cue is active at a time. A range is active from its start up to its end; a point is active for
* {@link POINT_CUE_DURATION} from where it sits, so a seek far past it never activates it.
*
* @internal
*/
function isCueActive(cue, time) {
	const end = cue.endTime > cue.startTime ? cue.endTime : cue.startTime + 1;
	return time >= cue.startTime && time < end;
}
/**
* The cues active at a time, in list order.
*
* Time updates come a few times a second, so playback at a high rate, or after a stall, can pass a point or a short cue
* between two of them. Given `since`, the time of the last update, a cue that started after it counts as active even
* when it's already over. Leave it out after a seek, so a seek never activates the cues it jumps over.
*
* @internal
*/
function findActiveCues(cues, time, since = NaN) {
	if (!Number.isFinite(time)) return [];
	const played = Number.isFinite(since) && since < time;
	return cues.filter((cue) => isCueActive(cue, time) || played && cue.startTime > since && cue.startTime <= time);
}

//#endregion
//#region ../core/dist/dev/dom/store/app-text-tracks.js
/** Private store state holding the tracks apps added. Owned by the text-track feature; the text cues feature reads them. */
const APP_TEXT_TRACKS = Symbol("@videojs/app-text-tracks");
/** Private store state that notifies when the tracks apps added change, and keeps them across a detach. */
const APP_TEXT_TRACKS_CHANGE = Symbol("@videojs/app-text-tracks-change");
/** The tracks apps added, from store state that may not include the text-track feature. */
function getAppTextTracks(state) {
	return state[APP_TEXT_TRACKS] ?? [];
}
/** Listen for changes to the tracks apps added, when the store has the text-track feature. */
function listenForAppTextTracks(state, listener, signal) {
	const change = state[APP_TEXT_TRACKS_CHANGE];
	if (!change) return;
	const unsubscribe = change.subscribe(listener);
	signal.addEventListener("abort", unsubscribe, { once: true });
}
/** Each copy of an app track in the media, to the id of the app track it copies. */
const appTrackCopies = /* @__PURE__ */ new WeakMap();
/** Mark a media track as the copy of the app track with `id`, or unmark it given `null`. */
function setAppTrackCopy(track, id) {
	if (id) appTrackCopies.set(track, id);
	else appTrackCopies.delete(track);
}
/**
* The id of the app track a media track copies, if it's a copy. Copies are told apart by identity, so a media track
* that happens to share an app track's id is never taken for one.
*/
function getAppTrackCopyId(track) {
	return appTrackCopies.get(track);
}

//#endregion
//#region ../core/dist/dev/dom/store/text-cues.js
/**
* Plain cue data from a text track's cue list.
*
* Every cue gets an id: its own, or one derived from where it sits when it has none. `value` comes from the cue itself
* when it carries data, such as a `DataCue` or a `VTTCue` hls.js filled in, and otherwise from text on a `metadata`
* track that parses to a JSON object or array. Plain text, and text that isn't valid JSON, leave `value` unset.
*
* Cues delivered with a stream can be open-ended — a chapters document leaves its last chapter open, and the track
* carries that as a very large `endTime` because engines reject `Infinity` — so a consumer reading the list gets each
* cue ending no later than the media does. While the duration is unknown or infinite the ends pass through unchanged.
* The result is plain data, never the live cues, so a store can expose it without leaking DOM objects. A cue that
* hasn't changed since it was last read comes back as the same object, so reading a long list again only converts the
* cues that are new or changed.
*/
function toMediaTextCues(cues, options = {}) {
	if (!cues) return [];
	const { kind, duration } = options;
	const max = toMaxEnd(duration);
	const list = Array.from(cues);
	const getId = createCueIds(list);
	return list.map((cue) => toMediaTextCue(cue, kind, max, getId));
}
/**
* Whether reading `cues` again would give back the copies last read from `read`: the same cues, in the same order, none
* changed in place. It checks without allocating, so a long list can be checked on every tick and read again only when
* it changed.
*/
function isCueListUnchanged(cues, read, options = {}) {
	const max = toMaxEnd(options.duration);
	let index = 0;
	for (const cue of cues ?? []) if (cue !== read[index++] || !isCopyCurrent(plainCopies.get(cue), cue, options.kind, max)) return false;
	return index === read.length;
}
function toMaxEnd(duration = NaN) {
	return Number.isFinite(duration) && duration > 0 ? duration : Number.POSITIVE_INFINITY;
}
/** The last plain copy of each live cue. */
const plainCopies = /* @__PURE__ */ new WeakMap();
/** Engines can change a cue in place, so a copy is current only while everything it was made from still matches. */
function isCopyCurrent(copy, cue, kind, max) {
	if (!copy) return false;
	return copy.kind === kind && copy.id === cue.id && copy.data === ("value" in cue ? cue.value : void 0) && copy.cue.startTime === cue.startTime && copy.cue.endTime === Math.min(cue.endTime, max) && copy.cue.text === getCueText(cue) && copy.cue.type === getCueType(cue);
}
function getCueText(cue) {
	return "text" in cue && isString(cue.text) ? cue.text : "";
}
/**
* Types engines give values of the same shape as another type, by that type. ID3 from an `emsg` box is the same ID3
* frame as ID3 from a segment, so readers check one type for both.
*/
const CUE_TYPE_ALIASES = /* @__PURE__ */ new Map([["https://aomedia.org/emsg/ID3", "org.id3"]]);
/** What a cue's `value` holds, so only a cue with a `value` has one. */
function getCueType(cue) {
	if (!("value" in cue) || isUndefined(cue.value)) return void 0;
	const type = "type" in cue && isString(cue.type) && cue.type ? cue.type : void 0;
	return type && (CUE_TYPE_ALIASES.get(type) ?? type);
}
function toMediaTextCue(cue, kind, max, getId) {
	const id = getId(cue);
	const copy = plainCopies.get(cue);
	if (isCopyCurrent(copy, cue, kind, max) && copy.cue.id === id) return copy.cue;
	const text = getCueText(cue);
	const data = "value" in cue ? cue.value : void 0;
	const type = getCueType(cue);
	const result = {
		id,
		startTime: cue.startTime,
		endTime: Math.min(cue.endTime, max),
		text
	};
	if (!isUndefined(data)) {
		result.value = data;
		if (type) result.type = type;
	} else if (kind === "metadata") {
		const value = parseJsonValue(text);
		if (!isUndefined(value)) result.value = value;
	}
	plainCopies.set(cue, {
		kind,
		id: cue.id,
		data,
		cue: result
	});
	return result;
}
/**
* Ids for the cues in one list: a cue's own id, or where it starts and ends when it has none. A cue keeps its own id
* the first time it appears; a repeat, or a derived id another cue already owns, gets a suffix that no cue in the list
* has.
*/
function createCueIds(cues) {
	const own = /* @__PURE__ */ new Set();
	const used = /* @__PURE__ */ new Set();
	const counts = /* @__PURE__ */ new Map();
	for (const cue of cues) if (cue.id) own.add(cue.id);
	return (cue) => {
		const base = cue.id || `${cue.startTime}-${cue.endTime}`;
		let id = base;
		if (used.has(id) || !cue.id && own.has(id)) {
			let count = counts.get(base) ?? 0;
			do
				id = `${base}-${++count}`;
			while (own.has(id) || used.has(id));
			counts.set(base, count);
		}
		used.add(id);
		return id;
	};
}
/** A JSON object or array from cue text, or `undefined` for anything else. Never throws. */
function parseJsonValue(text) {
	const trimmed = text.trim();
	if (!trimmed.startsWith("{") && !trimmed.startsWith("[")) return void 0;
	try {
		const value = JSON.parse(trimmed);
		return isObject(value) ? value : void 0;
	} catch {
		return;
	}
}
/**
* `next`, keeping each cue from `previous` that hasn't changed, and `previous` itself when nothing has. Store cues are
* rebuilt on every sync, so this is what lets subscribers compare cues and cue lists by identity.
*/
function reuseCues(previous, next) {
	if (!previous) return next;
	const reused = isSameList(previous, next) ? next : reuseById(previous, next);
	return isSameList(previous, reused) ? previous : reused;
}
function reuseById(previous, next) {
	const byId = new Map(previous.map((cue) => [cue.id, cue]));
	return next.map((cue) => {
		const old = byId.get(cue.id);
		return old && isSameCue(old, cue) ? old : cue;
	});
}
function isSameList(a, b) {
	return a.length === b.length && a.every((cue, index) => cue === b[index]);
}
function isSameCue(a, b) {
	return a.startTime === b.startTime && a.endTime === b.endTime && a.text === b.text && a.type === b.type && deepEqual(a.value, b.value);
}
/** Cues ordered by start time, then end time, keeping the order of cues that tie. */
function sortCues(cues) {
	return cues.sort((a, b) => a.startTime - b.startTime || a.endTime - b.endTime);
}
/** Chapters an app gave no end run to the next chapter, and the last one to the end of the media. */
function fillChapterEnds(cues, duration) {
	if (cues.every((cue) => cue.endTime > cue.startTime)) return cues;
	const end = Number.isFinite(duration) && duration > 0 ? duration : void 0;
	return cues.map((cue, index) => {
		if (cue.endTime > cue.startTime) return cue;
		return {
			...cue,
			endTime: cues[index + 1]?.startTime ?? end ?? cue.endTime
		};
	});
}

//#endregion
//#region ../core/dist/dev/dom/store/text-tracks.js
/**
* A media track's id in `textTrackList`: its own, or one derived from where it sits and what it is. A copy of an app
* track goes by that track's id, and a track whose own id is one of `appIds` goes by the derived id, so a media track
* and an app track never share one.
*/
function getTextTrackId(track, index, appIds) {
	const copyOf = getAppTrackCopyId(track);
	if (copyOf) return copyOf;
	return track.id && !appIds?.has(track.id) ? track.id : `track:${index}:${track.kind}:${track.language}:${track.label}`;
}
/** A media track as plain data. */
function toMediaTextTrack(track, index, appIds) {
	return {
		id: getTextTrackId(track, index, appIds),
		kind: track.kind,
		label: track.label,
		language: track.language,
		mode: track.mode,
		source: "media"
	};
}
/**
* Call `listener` when a `<track>` element still missing cues finishes loading. `addtrack` fires before a track's cues
* are parsed, and only the element's `load` tells when they're ready.
*/
function listenForTrackLoads(media, listener, signal) {
	const tracks = isQuerySelectorAllCapable(media) && media.querySelectorAll("track") || [];
	const shadowTracks = media instanceof HTMLElement && media.shadowRoot?.querySelectorAll("track") || [];
	for (const element of [...tracks, ...shadowTracks]) if (!element.track?.cues?.length) listen$1(element, "load", listener, { signal });
}

//#endregion
//#region ../core/dist/dev/dom/store/features/text-cues.js
const CUE_FILTERS = Symbol("@videojs/text-cue-filters");
/**
* Tracks the cues, and the cues active now, of the text tracks being read, from the media or added by the app through
* the text-track feature. `metadata` and `chapters` tracks that aren't disabled are read by default, except the
* thumbnails track; `watchTextCues()` adds others while a reader needs them. Kept apart from the text-track feature so
* cue changes never reach UI that only lists tracks.
*
* Active cues follow `currentTime` with one rule for every track, from the media or the app, so a point added from code
* and a zero-length cue in a file behave alike.
*/
const textCuesFeature = definePlayerFeature({
	name: "textCues",
	state: () => {
		const entries = /* @__PURE__ */ new Set();
		const filterListeners = createListeners();
		return {
			textCues: [],
			watchTextCues(filter) {
				const entry = { filter };
				entries.add(entry);
				filterListeners.notify();
				return () => {
					if (!entries.delete(entry)) return;
					filterListeners.notify();
				};
			},
			[CUE_FILTERS]: {
				values: () => Array.from(entries, ({ filter }) => filter),
				subscribe: filterListeners.subscribe
			}
		};
	},
	attach({ target, signal, get, set }) {
		const { media } = target;
		const textTracks = isMediaTextTrackCapable(media) ? media : null;
		let tracked = /* @__PURE__ */ new Map();
		let trackCleanup = null;
		let lastTime = NaN;
		const getTime = () => isMediaSeekCapable(media) ? media.currentTime : NaN;
		const getDuration = () => isMediaSeekCapable(media) ? media.duration : NaN;
		const sync = () => {
			trackCleanup?.abort();
			trackCleanup = new AbortController();
			const filters = get()[CUE_FILTERS].values();
			const previous = new Map(get().textCues.map((entry) => [entry.track.id, entry]));
			const duration = getDuration();
			const time = getTime();
			const textCues = [];
			const appRecords = getAppTextTracks(get());
			const appTrackIds = new Set(appRecords.map(({ track }) => track.id));
			tracked = /* @__PURE__ */ new Map();
			for (let i = 0; i < (textTracks?.textTracks.length ?? 0); i++) {
				const track = textTracks.textTracks[i];
				if (getAppTrackCopyId(track)) continue;
				const plain = toMediaTextTrack(track, i, appTrackIds);
				if (!isTracked(plain, filters)) continue;
				const prior = previous.get(plain.id);
				const read = Array.from(track.cues ?? []);
				const cues = reuseCues(prior?.cues, sortCues(toMediaTextCues(read, {
					kind: plain.kind,
					duration
				})));
				const activeCues = reuseCues(prior?.activeCues, findActiveCues(cues, time, lastTime));
				tracked.set(plain.id, {
					track,
					read
				});
				textCues.push(reuseTrackCues(prior, {
					track: reuseTrack(prior?.track, plain),
					cues,
					activeCues
				}));
				if (track instanceof EventTarget) listen$1(track, "cuechange", syncActive, { signal: trackCleanup.signal });
			}
			for (const record of appRecords) {
				if (!isTracked(record.track, filters)) continue;
				const prior = previous.get(record.track.id);
				const cues = record.track.kind === "chapters" ? reuseCues(prior?.cues, fillChapterEnds(record.cues, duration)) : record.cues;
				const activeCues = reuseCues(prior?.activeCues, findActiveCues(cues, time, lastTime));
				textCues.push(reuseTrackCues(prior, {
					track: record.track,
					cues,
					activeCues
				}));
			}
			if (textTracks) listenForTrackLoads(textTracks, sync, trackCleanup.signal);
			lastTime = time;
			set({ textCues: reuseList(get().textCues, textCues) });
		};
		const syncActive = () => {
			const duration = getDuration();
			const time = getTime();
			const current = get().textCues;
			const next = current.map((entry) => {
				const source = tracked.get(entry.track.id);
				const cues = source ? readChangedCues(source, entry, duration) : entry.cues;
				const activeCues = reuseCues(entry.activeCues, findActiveCues(cues, time, lastTime));
				return reuseTrackCues(entry, {
					...entry,
					cues,
					activeCues
				});
			});
			lastTime = time;
			set({ textCues: reuseList(current, next) });
		};
		const forgetLastTime = () => {
			lastTime = NaN;
		};
		const syncSeeked = () => {
			forgetLastTime();
			syncActive();
		};
		sync();
		const trackList = textTracks?.textTracks;
		if (trackList instanceof EventTarget) {
			listen$1(trackList, "addtrack", sync, { signal });
			listen$1(trackList, "removetrack", sync, { signal });
			listen$1(trackList, "change", sync, { signal });
		}
		listen$1(media, "loadstart", sync, { signal });
		listen$1(media, "durationchange", sync, { signal });
		listen$1(media, "timeupdate", syncActive, { signal });
		listen$1(media, "seeking", forgetLastTime, { signal });
		listen$1(media, "seeked", syncSeeked, { signal });
		const unsubscribeFilters = get()[CUE_FILTERS].subscribe(sync);
		listenForAppTextTracks(get(), sync, signal);
		signal.addEventListener("abort", () => {
			unsubscribeFilters();
			trackCleanup?.abort();
		}, { once: true });
	}
});
/**
* The cues of a media track, read again only when they changed. Engines add, remove, and change cues in place without
* an event, so they're checked on every tick, and the check is cheap while they stay the same.
*/
function readChangedCues(source, entry, duration) {
	const options = {
		kind: entry.track.kind,
		duration
	};
	if (isCueListUnchanged(source.track.cues, source.read, options)) return entry.cues;
	source.read = Array.from(source.track.cues ?? []);
	return reuseCues(entry.cues, sortCues(toMediaTextCues(source.read, options)));
}
function isTracked(track, filters) {
	return isTrackedByDefault(track) || filters.some((filter) => matchTextTrack(track, filter));
}
function isTrackedByDefault(track) {
	if (track.mode === "disabled") return false;
	if (track.kind === "chapters") return true;
	return track.kind === "metadata" && track.label !== "thumbnails";
}
function reuseTrack(previous, next) {
	if (previous && previous.id === next.id && previous.kind === next.kind && previous.label === next.label && previous.language === next.language && previous.mode === next.mode && previous.source === next.source) return previous;
	return next;
}
function reuseTrackCues(previous, next) {
	if (previous && previous.track === next.track && previous.cues === next.cues && previous.activeCues === next.activeCues) return previous;
	return next;
}
/** `next`, or `previous` when every entry is the same, so an unchanged list never notifies subscribers. */
function reuseList(previous, next) {
	return next.length === previous.length && next.every((entry, index) => entry === previous[index]) ? previous : next;
}
function createListeners() {
	const listeners = /* @__PURE__ */ new Set();
	return {
		subscribe(listener) {
			listeners.add(listener);
			return () => listeners.delete(listener);
		},
		notify() {
			for (const listener of [...listeners]) listener();
		}
	};
}

//#endregion
//#region ../core/dist/dev/core/text-track/app.js
/**
* A cue an app added, as plain store data. A point's end is its start.
*
* @internal
*/
function toAppCue(cue, id) {
	const result = {
		id: cue.id || id,
		startTime: cue.startTime,
		endTime: cue.endTime ?? cue.startTime,
		text: cue.text ?? ""
	};
	if (!isUndefined(cue.value)) result.value = cue.value;
	if (cue.type) result.type = cue.type;
	return result;
}

//#endregion
//#region ../core/dist/dev/dom/store/load-text-track.js
/**
* Fetch and parse the cues of a track an app added with a `src`. The track's `format` reads the file when it has one;
* otherwise WebVTT goes through the browser's own parser, and anything else is JSON: an array of cues, or an object
* with a `cues` array.
*/
async function loadTextTrackCues(load, track, signal) {
	const response = await fetch(load.src, { signal });
	if (!response.ok) throw new Error(`[vjs-core] could not load text track ${load.src} (${response.status})`);
	const text = await response.text();
	if (!load.format && text.trimStart().startsWith("WEBVTT")) {
		const { parseVtt } = await import("./parse-vtt-ESZtNAch.js");
		return parseVtt(text, track.kind, signal);
	}
	return readCueList(load.format ? await load.format(text) : JSON.parse(text)).map((cue, index) => toAppCue(cue, `${track.id}-src-${index}`));
}
function readCueList(result) {
	if (Array.isArray(result)) return result;
	if (isObject(result) && "cues" in result && Array.isArray(result.cues)) return result.cues;
	throw new Error("[vjs-core] text track cues must be an array, or an object with a `cues` array");
}

//#endregion
//#region ../media/dist/dev/dom/text-track/text-track.js
/** `HTMLTrackElement.LOADED`: the element loaded its resource. `ERROR`, the only state above it, follows a failed load. */
const TRACK_LOADED = 2;
/**
* Add a text track to `media` that the caller can write cues to and remove again.
*
* DOM media back the track with a `<track>` element on the element they play through, which is what makes it removable.
* Chromium discards cues added to such a track before the element settles, which a `<track>` without `src` does by
* firing `error` once it leaves `disabled` mode. The handle holds cues until then, so callers can write them right
* away.
*
* @returns `null` when the media gives no track.
* @internal
*/
function createTextTrack(media, options) {
	const { id, kind, label, language, mode = "hidden" } = options;
	const mediaElement = findMediaElement(media);
	const track = mediaElement ? addRemovableTextTrack(mediaElement, kind, label, language) : media.addTextTrack?.(kind, label, language);
	if (!track) return null;
	const element = mediaElement ? findTrackElement(mediaElement, track) : null;
	if (element && id) element.id = id;
	const pending = /* @__PURE__ */ new Set();
	const written = /* @__PURE__ */ new Map();
	const listeners = new AbortController();
	let settled = !element || element.readyState >= TRACK_LOADED;
	let removed = false;
	const write = (cue) => {
		const nativeCue = createNativeCue(cue);
		if (!nativeCue) return;
		track.addCue?.(nativeCue);
		written.set(cue, nativeCue);
	};
	const settle = () => {
		listeners.abort();
		settled = true;
		for (const cue of pending) write(cue);
		pending.clear();
		if (mode === "disabled" && track.mode === "hidden") track.mode = "disabled";
	};
	if (element && !settled) {
		listen$1(element, "load", settle, { signal: listeners.signal });
		listen$1(element, "error", settle, { signal: listeners.signal });
	}
	track.mode = mode === "disabled" && !settled ? "hidden" : mode;
	return {
		track,
		addCue(cue) {
			if (removed || pending.has(cue) || written.has(cue)) return;
			if (settled) write(cue);
			else pending.add(cue);
		},
		removeCue(cue) {
			if (pending.delete(cue)) return;
			const nativeCue = written.get(cue);
			if (!nativeCue) return;
			written.delete(cue);
			try {
				track.removeCue?.(nativeCue);
			} catch {}
		},
		remove() {
			if (removed) return;
			removed = true;
			listeners.abort();
			pending.clear();
			written.clear();
			track.mode = "disabled";
			if (element) element.remove();
			else media.removeTextTrack?.(track);
		}
	};
}
/**
* The native element `media` plays through, also when it's a media component fronting another, such as an `<hls-video>`
* slotted into another media component.
*/
function findMediaElement(media) {
	const element = getMediaElement(media);
	if (element) return element;
	const target = isObject(media) ? getRegisteredMedia(media).target : null;
	return isObject(target) && target !== media ? findMediaElement(target) : null;
}
function isDataCue(cue) {
	return "value" in cue && !isUndefined(cue.value);
}
/** A native cue carrying `cue`, or `null` where the platform can't build one. */
function createNativeCue(cue) {
	try {
		const nativeCue = isDataCue(cue) ? createDataCue(cue) : createVTTCue(cue.startTime, cue.endTime, cue.text);
		if (nativeCue && cue.id) nativeCue.id = cue.id;
		return nativeCue;
	} catch {
		return null;
	}
}
/**
* A `DataCue` where the platform has one. Elsewhere, what hls.js writes: a `VTTCue` with `value` and `type` set on it,
* here with the value as JSON for its text so readers that only see text still get the data.
*/
function createDataCue({ startTime, endTime, value, type }) {
	const { DataCue } = globalThis;
	if (DataCue) return new DataCue(startTime, endTime, value, type);
	const cue = createVTTCue(startTime, endTime, toJson(value));
	return cue && Object.assign(cue, type ? {
		value,
		type
	} : { value });
}
function createVTTCue(startTime, endTime, text) {
	const { VTTCue } = globalThis;
	return VTTCue ? new VTTCue(startTime, endTime, text) : null;
}
function toJson(value) {
	try {
		return JSON.stringify(value) ?? "";
	} catch {
		return "";
	}
}

//#endregion
//#region ../core/dist/dev/dom/store/features/text-track.js
/** The modes of app caption copies when the store last detached, so a re-attach can restore them. */
const savedCopyModes = /* @__PURE__ */ new WeakMap();
/** Caption/subtitle tracks paired with the ids exposed through `textTrackList`, in captions menu order. */
function getSubtitlesTracks(media, records) {
	const appIds = new Set(records.map(({ track }) => track.id));
	return getCaptionOrSubtitleTracks(Array.from(media.textTracks, (track, index) => ({
		id: getTextTrackId(track, index, appIds),
		kind: track.kind,
		track
	})));
}
/** Show at most one caption/subtitle track; passing `null` disables them all. */
function showOnly(tracks, active) {
	for (const { track } of tracks) {
		const mode = track === active ? "showing" : "disabled";
		if (track.mode !== mode) track.mode = mode;
	}
}
/**
* The cues of the first chapters track an app added, which stand in when the media has no chapters of its own. A
* chapter without an end runs to the next one, and the last to the end of the media.
*/
function findAppChaptersCues(records, duration) {
	return fillChapterEnds(records.find(({ track }) => track.kind === "chapters")?.cues ?? [], duration);
}
/**
* The first thumbnails track an app added, which stands in when the media has none of its own, such as a storyboard on
* an embed. Image URLs in its cues resolve against the file the cues came from.
*/
function findAppThumbnailsTrack(records, crossOrigin) {
	const record = records.find(({ track }) => track.kind === "metadata" && track.label === "thumbnails");
	if (!record) return null;
	return {
		cues: record.cues,
		src: record.src ? resolveUrl(record.src) : null,
		crossOrigin
	};
}
/** `url` made absolute against the page, as a `<track>` element's `src` reads. */
function resolveUrl(url) {
	try {
		return new URL(url, document.baseURI).href;
	} catch {
		return url;
	}
}
function findLocaleTrack(tracks, locale) {
	const localeKey = getLocaleKey(locale);
	const keys = findLocaleKeys(locale);
	if (localeKey !== "en" && !localeKey.startsWith(`${"en"}-`)) keys.pop();
	for (const key of keys) {
		const exact = tracks.find(({ track }) => getLocaleKey(track.language) === key);
		if (exact) return exact;
		const regional = tracks.find(({ track }) => getLocaleKey(track.language).startsWith(`${key}-`));
		if (regional) return regional;
	}
}
/**
* Lists the media's text tracks and the tracks apps add, selects captions, and holds the chapters and thumbnails
* tracks. App tracks live in the store, so they work on any media; app captions and subtitles are also copied into the
* media so the browser can draw them. The text cues feature reads the cues of both.
*/
const textTrackFeature = definePlayerFeature({
	name: "textTrack",
	state: ({ target, get, set }) => {
		let lastShownId = null;
		let trackCount = 0;
		const appListeners = /* @__PURE__ */ new Set();
		const appTracks = {
			records: [],
			subscribe(listener) {
				appListeners.add(listener);
				return () => appListeners.delete(listener);
			},
			notify() {
				appTracks.records = getAppTextTracks(get());
				for (const listener of [...appListeners]) listener();
			}
		};
		const setRecords = (records) => {
			set({ [APP_TEXT_TRACKS]: records });
			appTracks.notify();
		};
		const updateRecord = (id, update) => {
			const records = appTracks.records;
			if (!records.some((record) => record.track.id === id)) return;
			setRecords(records.map((record) => record.track.id === id ? update(record) : record));
		};
		const removeTextTrack = (id) => {
			const records = appTracks.records;
			const next = records.filter((record) => record.track.id !== id);
			if (next.length !== records.length) setRecords(next);
		};
		const addTextTrack = (init, content) => {
			const id = `app-text-track-${++trackCount}`;
			let cueCount = 0;
			const toCue = (cue) => toAppCue(cue, `${id}-cue-${cueCount++}`);
			const record = {
				track: {
					id,
					kind: init.kind,
					label: init.label ?? "",
					language: init.language ?? "",
					mode: "hidden",
					source: "app"
				},
				cues: sortCues((content?.cues ?? init.cues ?? []).map(toCue))
			};
			const src = content?.src ?? init.src;
			if (src) {
				record.src = src;
				record.load = {
					src,
					format: content?.format ?? init.format
				};
			}
			setRecords([...appTracks.records, record]);
			return {
				id,
				get removed() {
					return !appTracks.records.some((entry) => entry.track.id === id);
				},
				addCue(cue) {
					updateRecord(id, (current) => ({
						...current,
						cues: sortCues([...current.cues, toCue(cue)])
					}));
				},
				removeCue(cueId) {
					updateRecord(id, (current) => ({
						...current,
						cues: current.cues.filter((cue) => cue.id !== cueId)
					}));
				},
				remove() {
					removeTextTrack(id);
				}
			};
		};
		return {
			textTrackList: [],
			subtitlesShowing: false,
			toggleSubtitles(forceShow) {
				const { media } = target();
				if (!isMediaTextTrackCapable(media)) return false;
				const subtitlesTracks = getSubtitlesTracks(media, appTracks.records);
				if (!subtitlesTracks.length) return false;
				const showing = subtitlesTracks.find(({ track }) => track.mode === "showing");
				const nextShowing = forceShow ?? !showing;
				if (showing) lastShownId = showing.id;
				if (!nextShowing) {
					showOnly(subtitlesTracks, null);
					return false;
				}
				const next = showing ?? subtitlesTracks.find(({ id }) => id === lastShownId) ?? findLocaleTrack(subtitlesTracks, globalThis.navigator?.language ?? "") ?? subtitlesTracks[0];
				lastShownId = next.id;
				showOnly(subtitlesTracks, next.track);
				return true;
			},
			selectSubtitlesTrack(id) {
				const { media } = target();
				if (!isMediaTextTrackCapable(media)) return;
				const subtitlesTracks = getSubtitlesTracks(media, appTracks.records);
				if (!subtitlesTracks.length) return;
				if (isNull(id)) {
					const showing = subtitlesTracks.find(({ track }) => track.mode === "showing");
					if (showing) lastShownId = showing.id;
					showOnly(subtitlesTracks, null);
					return;
				}
				const active = subtitlesTracks.find((entry) => entry.id === id);
				if (!active) return;
				lastShownId = active.id;
				showOnly(subtitlesTracks, active.track);
			},
			chaptersCues: [],
			thumbnailsTrack: null,
			addTextTrack,
			removeTextTrack,
			[APP_TEXT_TRACKS]: [],
			[APP_TEXT_TRACKS_CHANGE]: appTracks
		};
	},
	attach({ target, signal, get, set }) {
		const { media } = target;
		const textTracks = isMediaTextTrackCapable(media) ? media : null;
		const getCrossOrigin = () => isMediaSourceCapable(media) ? toMediaCrossOrigin(media.crossOrigin) : null;
		const appTracks = get()[APP_TEXT_TRACKS_CHANGE];
		set({ [APP_TEXT_TRACKS]: appTracks.records });
		appTracks.notify();
		const copies = /* @__PURE__ */ new Map();
		const syncCopies = () => {
			if (!textTracks) return;
			const records = get()[APP_TEXT_TRACKS].filter(({ track }) => isCaptionOrSubtitleTrack(track));
			const ids = new Set(records.map(({ track }) => track.id));
			for (const [id, copy] of copies) {
				if (ids.has(id)) continue;
				copy.handle.remove();
				setAppTrackCopy(copy.handle.track, null);
				copies.delete(id);
			}
			for (const { track, cues } of records) {
				let copy = copies.get(track.id);
				if (!copy) {
					const { id, kind, label, language } = track;
					const mode = savedCopyModes.get(appTracks)?.get(id) ?? "disabled";
					const handle = createTextTrack(textTracks, {
						id,
						kind,
						label,
						language,
						mode
					});
					if (!handle) continue;
					setAppTrackCopy(handle.track, id);
					copy = {
						handle,
						written: /* @__PURE__ */ new Map()
					};
					copies.set(id, copy);
				}
				const current = new Set(cues);
				for (const [cue, written] of copy.written) {
					if (current.has(cue)) continue;
					copy.handle.removeCue(written);
					copy.written.delete(cue);
				}
				for (const cue of cues) {
					if (copy.written.has(cue)) continue;
					const written = {
						id: cue.id,
						startTime: cue.startTime,
						endTime: cue.endTime,
						text: cue.text
					};
					copy.handle.addCue(written);
					copy.written.set(cue, written);
				}
			}
		};
		const loads = /* @__PURE__ */ new Map();
		const loadAppCues = () => {
			const records = get()[APP_TEXT_TRACKS];
			for (const [id, controller] of loads) {
				if (records.some(({ track }) => track.id === id)) continue;
				controller.abort();
				loads.delete(id);
			}
			for (const { track, load } of records) {
				if (!load || loads.has(track.id)) continue;
				const controller = new AbortController();
				loads.set(track.id, controller);
				loadTextTrackCues(load, track, controller.signal).then((cues) => updateAppTrack(track.id, (record) => ({
					...record,
					cues: sortCues([...record.cues, ...cues])
				}))).catch((error) => {
					if (isAbortError(error)) return;
					console.warn(error);
					updateAppTrack(track.id, (record) => record);
				}).finally(() => {
					if (loads.get(track.id) === controller) loads.delete(track.id);
				});
			}
		};
		const updateAppTrack = (id, update) => {
			if (signal.aborted) return;
			const records = get()[APP_TEXT_TRACKS];
			if (!records.some((record) => record.track.id === id)) return;
			set({ [APP_TEXT_TRACKS]: records.map((record) => record.track.id === id ? {
				...update(record),
				load: void 0
			} : record) });
			appTracks.notify();
		};
		let trackCleanup = null;
		const syncCopyModes = () => {
			const records = get()[APP_TEXT_TRACKS];
			let changed = false;
			const next = records.map((record) => {
				const mode = copies.get(record.track.id)?.handle.track.mode;
				if (!mode || mode === record.track.mode) return record;
				changed = true;
				return {
					...record,
					track: {
						...record.track,
						mode
					}
				};
			});
			if (!changed) return false;
			set({ [APP_TEXT_TRACKS]: next });
			appTracks.notify();
			return true;
		};
		const sync = () => {
			if (syncCopyModes()) return;
			const appRecords = get()[APP_TEXT_TRACKS];
			if (!textTracks) {
				set({
					textTrackList: appRecords.map(({ track }) => track),
					chaptersCues: findAppChaptersCues(appRecords, isMediaSeekCapable(media) ? media.duration : NaN),
					thumbnailsTrack: findAppThumbnailsTrack(appRecords, getCrossOrigin())
				});
				return;
			}
			trackCleanup?.abort();
			trackCleanup = new AbortController();
			let chaptersTrack = null;
			let thumbnailsTextTrack = null;
			const textTrackList = [];
			let subtitlesShowing = false;
			const appTrackIds = new Set(appRecords.map(({ track }) => track.id));
			for (let i = 0; i < textTracks.textTracks.length; i++) {
				const track = textTracks.textTracks[i];
				if (getAppTrackCopyId(track)) {
					if (isCaptionOrSubtitleTrack(track) && track.mode === "showing") subtitlesShowing = true;
					continue;
				}
				if (!chaptersTrack && track.kind === "chapters") chaptersTrack = track;
				if (!thumbnailsTextTrack && track.kind === "metadata" && track.label === "thumbnails") thumbnailsTextTrack = track;
				textTrackList.push(toMediaTextTrack(track, i, appTrackIds));
				if (isCaptionOrSubtitleTrack(track) && track.mode === "showing") subtitlesShowing = true;
			}
			for (const { track } of appRecords) textTrackList.push(track);
			const duration = isMediaSeekCapable(media) ? media.duration : NaN;
			const chaptersCues = chaptersTrack ? toMediaTextCues(chaptersTrack.cues, {
				kind: "chapters",
				duration
			}) : findAppChaptersCues(appRecords, duration);
			let thumbnailsTrack = findAppThumbnailsTrack(appRecords, getCrossOrigin());
			if (thumbnailsTextTrack) thumbnailsTrack = {
				cues: thumbnailsTextTrack.cues ? Array.from(thumbnailsTextTrack.cues) : [],
				src: findTrackElement(textTracks, thumbnailsTextTrack)?.src ?? null,
				crossOrigin: getCrossOrigin()
			};
			listenForTrackLoads(textTracks, sync, trackCleanup.signal);
			set({
				textTrackList,
				subtitlesShowing,
				chaptersCues,
				thumbnailsTrack
			});
		};
		syncCopies();
		sync();
		loadAppCues();
		const unsubscribeAppTracks = appTracks.subscribe(() => {
			syncCopies();
			sync();
			loadAppCues();
		});
		const trackList = textTracks?.textTracks;
		if (trackList instanceof EventTarget) {
			listen$1(trackList, "addtrack", sync, { signal });
			listen$1(trackList, "removetrack", sync, { signal });
			listen$1(trackList, "change", sync, { signal });
		}
		listen$1(media, "loadstart", sync, { signal });
		listen$1(media, "durationchange", sync, { signal });
		signal.addEventListener("abort", () => {
			unsubscribeAppTracks();
			trackCleanup?.abort();
			for (const controller of loads.values()) controller.abort();
			loads.clear();
			savedCopyModes.set(appTracks, new Map(Array.from(copies, ([id, { handle }]) => [id, handle.track.mode])));
			for (const { handle } of copies.values()) handle.remove();
			copies.clear();
		}, { once: true });
	}
});

//#endregion
//#region ../core/dist/dev/dom/store/features/volume.js
/** Volume to restore when unmuting at zero. */
const UNMUTE_VOLUME = .25;
const volumeFeature = definePlayerFeature({
	name: "volume",
	state: ({ target, get, set }) => {
		const isVolumeLocked = () => get().volumeAvailability === "unsupported";
		return {
			volume: 1,
			muted: false,
			volumeAvailability: "unavailable",
			mutedAvailability: "unavailable",
			setVolume(volume) {
				const { media } = target();
				if (!isMediaVolumeCapable(media)) return 0;
				const clamped = Math.max(0, Math.min(1, volume));
				if (clamped > 0 && media.muted) media.muted = false;
				if (!isVolumeLocked()) media.volume = clamped;
				set({
					volume: media.volume,
					muted: media.muted
				});
				return media.volume;
			},
			setMuted(muted) {
				const { media } = target();
				if (!isMediaMutedCapable(media)) return false;
				media.muted = muted;
				const volumeCapable = isMediaVolumeCapable(media);
				if (!muted && volumeCapable && !isVolumeLocked() && media.volume === 0) media.volume = UNMUTE_VOLUME;
				set({
					volume: volumeCapable ? media.volume : 1,
					muted: media.muted
				});
				return media.muted;
			}
		};
	},
	attach({ target, signal, set }) {
		const { media } = target;
		const volumeCapable = isMediaVolumeCapable(media);
		const mutedCapable = isMediaMutedCapable(media);
		if (!volumeCapable && !mutedCapable) return;
		set({
			volumeAvailability: volumeCapable ? canSetVolume() : "unavailable",
			mutedAvailability: mutedCapable ? "available" : "unavailable"
		});
		const sync = () => set({
			volume: volumeCapable ? media.volume : 1,
			muted: mutedCapable ? media.muted : false
		});
		sync();
		listen$1(media, "volumechange", sync, { signal });
	}
});
/**
* Check if the browser lets media volume be set (fails on iOS Safari). Probes a detached element rather than the
* attached Media: an embed's setter accepts any level, but its player plays through a media element in its iframe, so
* the browser's answer holds for every Media.
*/
function canSetVolume() {
	const video = document.createElement("video");
	try {
		video.volume = .5;
		return video.volume === .5 ? "available" : "unsupported";
	} catch {
		return "unsupported";
	}
}

//#endregion
export { throwNoTargetError as A, controlsFeature as C, combinePlayerFeatureConfigs as D, getMediaInputActionValue as E, isEditableTarget as M, isInteractiveActivation as N, setPlayerConfigValue as O, fullscreenFeature as S, getGestureActionValue as T, insertPlaybackRate as _, matchTextTrack as a, remotePlaybackFeature as b, errorFeature as c, playbackFeature as d, playbackRateFeature as f, timeFeature as g, streamTypeFeature as h, toMediaTextCues as i, AbortControllerRegistry as j, throwDestroyedError as k, liveFeature as l, sourceFeature as m, textTrackFeature as n, audioTrackFeature as o, qualityFeature as p, textCuesFeature as r, bufferFeature as s, volumeFeature as t, metadataFeature as u, parsePlaybackRates as v, getGestureCoordinator as w, pipFeature as x, roundPlaybackRate as y };
//# sourceMappingURL=volume-4z08ktsN.js.map