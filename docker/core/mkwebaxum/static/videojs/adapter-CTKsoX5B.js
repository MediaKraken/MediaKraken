import { n as onEvent } from "./abort-DsWgOZ8b.js";
import { t as HTMLMediaAdapter } from "./html-media-adapter-BttXG6mg.js";
import { At as defineCompositionFactory, Ft as SVTA_UNSUPPORTED_PLAYBACK_FEATURE, Kt as SVTA_PRIMARY_MANIFEST_LOAD_ERROR, S as calculateDurationFeature, Wt as SVTA_NO_SUPPORTED_AUDIO_TRACK, Xt as findTrackById, a as LOW_LATENCY_UNSUPPORTED_MESSAGE, an as effect, c as resolveSourceUrl, g as hlsLoadingFeature, i as DVR_EXPERIMENTAL_MESSAGE, in as isResolvedTrack, kt as flattenFeatures, l as shiftTimestampsFeature, n as hasUnsupportedFeatureCause, p as mediaSourceFeature, r as withAlternativeMediaSuggestion, rn as isResolvedPresentation, s as UNSUPPORTED_PLAYBACK_FEATURE_MESSAGE, t as firstFatal, tn as getMediaPlaylistMetadata, x as currentTimeFeature, y as errorFeature } from "./error-surface-BHz7DxGx.js";
import { n as MediaStreamTypes, r as toMediaCrossOrigin } from "./types-D8a85nGw.js";
import { _ as liveTrackId, a as initialLoadFeature, c as audioFeature, g as getLiveEdge, i as liveFeature, m as airPlayFeature, n as startPositionFeature, o as endStallRecoveryFeature, r as multiCdnFeature, s as chaptersFeature, t as crossOriginToRequestCredentials, v as deriveTargetLiveWindow, y as resolveLiveLatency } from "./request-credentials-DILzL07q.js";

//#region ../media/dist/dev/dom/html-audio-adapter/html-audio-adapter.js
/** @internal */
var HTMLAudioAdapter = class extends HTMLMediaAdapter {};

//#endregion
//#region ../spf/dist/dev/engines/hls-audio.js
/**
* The features the audio-only HLS playback engine composes, in order. A feature's behaviors compose in its position,
* and a later feature's `defaultConfig` and `initialState` values replace an earlier one's.
*/
const features = [
	initialLoadFeature,
	hlsLoadingFeature,
	multiCdnFeature,
	errorFeature,
	calculateDurationFeature,
	mediaSourceFeature,
	audioFeature,
	shiftTimestampsFeature,
	airPlayFeature,
	currentTimeFeature,
	startPositionFeature,
	liveFeature,
	endStallRecoveryFeature,
	chaptersFeature
];
const composed = flattenFeatures(features);
/**
* The behaviors the engine composes, in setup order: its features' behaviors, each composed once. The engine's state
* and context types are derived from this list, so adding or removing a feature changes them with no separate type to
* update.
*/
const behaviors = composed.behaviors;
/**
* The defaults `createEngine` fills in for every config key the caller leaves `undefined`: its features' merged
* `defaultConfig`. Each is optional in `EngineConfig`, so a caller may override it.
*/
const defaultConfig = composed.defaultConfig;
/** The state the engine starts with: its features' merged `initialState`. */
const initialState = composed.initialState;
/**
* Create an audio-only HLS playback engine: the video engine without its video and subtitle behaviors, chapters kept.
* Subtractive composition variant of the HLS video engine (`./hls-video`): omits video-side behaviors
* (`resolveVideoTrack`, `switchVideoTrack`, `setupVideoBufferActors`, `loadVideoSegments`) and subtitle behaviors
* (`switchTextTrack`, `resolveTextTrack`, `syncTextTracks`, `setupTextTrackActors`, `loadTextTrackSegments`). Chapters
* (`loadChapters`) stay: they are session data, not a subtitle rendition. The remaining audio pipeline composes
* unchanged.
*
* Handles both truly audio-only HLS sources (no video stream-inf) and mixed-AV HLS sources where the audio rendition is
* selected and video / subtitle renditions are ignored at composition time. The variant decision is encoded by adapter
* choice; this engine does not branch on source shape.
*
* @example
*   ```ts
*   const engine = createEngine();
*
*   engine.context.mediaElement.set(audioEl);
*   engine.state.presentation.set({ url: 'https://example.com/stream.m3u8' });
*   ```;
*/
const createEngine = defineCompositionFactory(behaviors, {
	defaultConfig,
	initialState
});

//#endregion
//#region ../spf/dist/dev/playback/adapters/hls-audio/mixin.js
/**
* Which reported conditions this composition treats as fatal: a manifest that fails to load, and the audio verdict. An
* audio-only engine composes no video selection, so `SVTA_NO_SUPPORTED_VIDEO_TRACK` is never reported and surfacing it
* would describe a track type this media doesn't have.
*/
const FATAL_SVTA_CODES = /* @__PURE__ */ new Set([SVTA_PRIMARY_MANIFEST_LOAD_ERROR, SVTA_NO_SUPPORTED_AUDIO_TRACK]);
function HlsAudioMixin(BaseClass) {
	class HlsAudioImpl extends BaseClass {
		static defaultProps = {
			src: "",
			source: null,
			preload: "",
			crossOrigin: null,
			disableRemotePlayback: false,
			streamType: MediaStreamTypes.UNKNOWN
		};
		/**
		* Engine this media plays through, reported to analytics such as Mux Data. Names the SPF engine entry point it
		* composes (`@videojs/spf/engines/hls-audio`), since each SPF engine is its own composition.
		*/
		static engineName = "spf-hls-audio";
		/**
		* A complete sentence naming the Media to reach for when this one can't play a source. Appended to the copy this
		* adapter logs.
		*
		* Empty here, and overridden the same way as on the video adapter — see its note. `hls-audio` has no
		* better-equipped sibling of its own; the Mux audio Media built on this engine does, and points at the
		* hls.js-backed one.
		*
		* @internal
		*/
		static get alternativeMediaSuggestion() {}
		/**
		* The engine factory the adapter builds its engine with. A Media over a differently composed engine — one sharing
		* this engine's state and context, with more features laid in, as the Mux Medias lay the MPEG-TS transmux —
		* overrides this static and nothing else.
		*/
		static createEngine = createEngine;
		#engine;
		#config;
		#preload = HlsAudioImpl.defaultProps.preload;
		#crossOrigin = toMediaCrossOrigin(HlsAudioImpl.defaultProps.crossOrigin);
		#disableRemotePlayback = HlsAudioImpl.defaultProps.disableRemotePlayback;
		#streamType = HlsAudioImpl.defaultProps.streamType;
		#isUserStreamType = false;
		#targetLiveWindow = NaN;
		/** Delivery notices already logged for the current source — see the video adapter. */
		#noticed = /* @__PURE__ */ new Set();
		#stopLiveSync;
		#error = null;
		/** Reported condition currently surfaced — see the video adapter's note. */
		#reportedCode = null;
		#stopErrorSync;
		/** Aborting a generation cancels all retries, including ones not yet registered. */
		#playGeneration = new AbortController();
		#source = HlsAudioImpl.defaultProps.source;
		constructor(...args) {
			super(...args);
			const { config } = args[0] ?? {};
			this.#config = config ?? {};
			this.#engine = this.#createEngine();
			this.#stopLiveSync = effect(() => {
				const presentation = this.#engine.state.presentation.get();
				this.#setDetectedStreamType(presentation?.streamType ?? MediaStreamTypes.UNKNOWN);
				this.#setTargetLiveWindow(deriveTargetLiveWindow(presentation, liveTrackId(this.#engine.state)));
				this.#reportDeliveryNotices(presentation);
			});
			this.#stopErrorSync = effect(() => {
				const errors = this.#engine.state.errors.get();
				this.#setError(firstFatal(errors, FATAL_SVTA_CODES), errors);
			});
		}
		/**
		* The current fatal error, or `null`. Only _fatal_ conditions appear here — the engine reports non-fatal ones too,
		* which stay in `engine.state.errors`. Resets per source. Fires `'error'` when set.
		*/
		get error() {
			return this.#error;
		}
		/**
		* The source's stream type — `'live'`, `'on-demand'`, or `'unknown'` until a media playlist has been parsed.
		* Setting a non-`'unknown'` value pins a user override (detection stops updating it); setting `'unknown'` reverts
		* to the engine's detected value.
		*/
		get streamType() {
			return this.#streamType;
		}
		set streamType(value) {
			if (value === MediaStreamTypes.UNKNOWN) {
				this.#isUserStreamType = false;
				this.#updateStreamType(this.#engine.state.presentation.get()?.streamType ?? MediaStreamTypes.UNKNOWN);
				return;
			}
			this.#isUserStreamType = true;
			this.#updateStreamType(value);
		}
		/**
		* Presentation time marking the start of the live-edge window. Playback at `currentTime >= liveEdgeStart` counts as
		* "at the live edge", and seeking to live lands here. `NaN` when the stream isn't live or nothing has loaded yet.
		* No change event fires; re-read it on `timeupdate` or `progress`.
		*/
		get liveEdgeStart() {
			return getLiveEdge({
				state: this.#engine.state,
				config: { resolveLiveLatency }
			})?.liveEdgeStart ?? NaN;
		}
		/**
		* The target live window: `NaN` for on-demand (or unknown), `0` for standard sliding-window live, `Infinity` for
		* DVR (`#EXT-X-PLAYLIST-TYPE:EVENT`). Fires `targetlivewindowchange` on change.
		*/
		get targetLiveWindow() {
			return this.#targetLiveWindow;
		}
		#setDetectedStreamType(value) {
			if (this.#isUserStreamType) return;
			this.#updateStreamType(value);
		}
		#updateStreamType(value) {
			if (this.#streamType === value) return;
			this.#streamType = value;
			this.dispatchEvent?.(new Event("streamtypechange"));
		}
		#setTargetLiveWindow(value) {
			if (Object.is(this.#targetLiveWindow, value)) return;
			this.#targetLiveWindow = value;
			this.dispatchEvent?.(new Event("targetlivewindowchange"));
		}
		#reportDeliveryNotices(presentation) {
			if (!isResolvedPresentation(presentation)) {
				this.#noticed.clear();
				return;
			}
			const trackId = liveTrackId(this.#engine.state);
			const track = trackId ? findTrackById(presentation, trackId) : void 0;
			if (!track || !isResolvedTrack(track)) return;
			const metadata = getMediaPlaylistMetadata(track);
			if (!metadata) return;
			if (metadata.lowLatency && !this.#noticed.has("lowLatency")) {
				this.#noticed.add("lowLatency");
				console.warn(this.#withSuggestion(LOW_LATENCY_UNSUPPORTED_MESSAGE));
			}
			if (metadata.playlistType === "EVENT" && !this.#noticed.has("dvr")) {
				this.#noticed.add("dvr");
				console.warn(this.#withSuggestion(DVR_EXPERIMENTAL_MESSAGE));
			}
		}
		#setError(reported, errors) {
			if (!reported) {
				this.#error = null;
				this.#reportedCode = null;
				return;
			}
			if (this.#reportedCode === reported.code) return;
			this.#reportedCode = reported.code;
			const unsupported = hasUnsupportedFeatureCause(errors);
			if (unsupported) console.error(this.#withSuggestion(UNSUPPORTED_PLAYBACK_FEATURE_MESSAGE), { conditions: errors });
			this.#error = {
				code: unsupported ? SVTA_UNSUPPORTED_PLAYBACK_FEATURE : reported.code,
				message: reported.message ?? "",
				...reported.data === void 0 ? {} : { data: reported.data }
			};
			this.dispatchEvent?.(new Event("error"));
		}
		/**
		* Underlying playback engine — the low-level SPF reactive composition that drives playback. An advanced escape
		* hatch for direct engine access; normal playback is driven through this element's own properties and methods.
		*
		* @internal
		*/
		get engine() {
			return this.#engine;
		}
		attach(mediaElement) {
			if (mediaElement !== this.#engine.context.mediaElement.get()) this.#cancelPendingPlay();
			super.attach?.(mediaElement);
			this.#engine.context.mediaElement.set(mediaElement);
			this.#crossOrigin = toMediaCrossOrigin(mediaElement.crossOrigin) ?? this.#crossOrigin;
			mediaElement.crossOrigin = this.#crossOrigin;
		}
		detach() {
			this.#cancelPendingPlay();
			this.#engine.context.mediaElement.set(void 0);
			super.detach?.();
		}
		destroy() {
			this.#cancelPendingPlay();
			this.#stopLiveSync();
			this.#stopErrorSync();
			this.#engine.destroy();
		}
		get crossOrigin() {
			return this.#crossOrigin;
		}
		set crossOrigin(value) {
			this.#crossOrigin = toMediaCrossOrigin(value);
			const mediaElement = this.#engine.context.mediaElement.get();
			if (mediaElement) mediaElement.crossOrigin = value;
		}
		/** Preload type (`'none'` / `'metadata'` / `'auto'`). */
		get preload() {
			return this.#preload;
		}
		set preload(value) {
			this.#preload = value;
			if (value) this.#engine.state.preload.set(value);
		}
		get disableRemotePlayback() {
			return this.#disableRemotePlayback;
		}
		set disableRemotePlayback(value) {
			this.#disableRemotePlayback = value;
			this.#engine.state.disableRemotePlayback.set(value);
		}
		get src() {
			return this.#engine.state.presentation.get()?.url ?? "";
		}
		set src(value) {
			const url = resolveSourceUrl(value);
			if (url === this.src) return;
			this.#source = url ? { src: url } : null;
			this.#applySrc(url);
			this.dispatchEvent?.(new Event("sourcechange"));
		}
		/**
		* Structured source, the same shape the video flavor takes so one object serves either.
		*
		* `drm` is accepted and inert: this engine composes no EME. It is kept in the shape rather than removed so a source
		* can be handed to both flavors — and because Mux encrypts video renditions and leaves audio clear, so a protected
		* playback ID plays here regardless.
		*
		* @fires sourcechange - Fired when `source` changes. Read `source` for the new value.
		*/
		get source() {
			return this.#source;
		}
		set source(value) {
			const source = value ?? null;
			if (source === this.#source) return;
			this.#source = source;
			this.#applySrc(source?.src ?? "");
			this.dispatchEvent?.(new Event("sourcechange"));
		}
		/** Point the engine at a URL, resolved; an unchanged one is not a reload request. */
		#applySrc(value) {
			const url = resolveSourceUrl(value);
			if (url === this.src) return;
			this.#cancelPendingPlay();
			this.#engine.state.presentation.set(url ? { url } : void 0);
		}
		play() {
			const mediaElement = this.#engine.context.mediaElement.get();
			if (!mediaElement) return Promise.reject(/* @__PURE__ */ new Error("HlsAudioAdapterCore: no media element attached"));
			const { signal } = this.#playGeneration;
			this.#engine.state.loadActivated.set(true);
			return mediaElement.play().catch((err) => {
				signal.throwIfAborted();
				if (this.src) return onEvent(mediaElement, "loadstart", { signal }).then(() => {
					signal.throwIfAborted();
					return mediaElement.play();
				});
				throw err;
			});
		}
		/** `message`, plus the alternative-Media sentence when this class names one. */
		#withSuggestion(message) {
			return withAlternativeMediaSuggestion(message, this);
		}
		#createEngine() {
			return this.constructor.createEngine({
				...this.#config,
				requestCredentials: this.#config?.requestCredentials ?? (() => crossOriginToRequestCredentials(this.#crossOrigin))
			});
		}
		#cancelPendingPlay() {
			this.#playGeneration.abort();
			this.#playGeneration = new AbortController();
		}
	}
	return HlsAudioImpl;
}
/** Standalone SPF audio-only media adapter with no base class. */
var HlsAudioAdapterCore = class extends HlsAudioMixin(class {}) {};

//#endregion
//#region ../spf/dist/dev/playback/adapters/hls-audio/adapter.js
var HlsAudioAdapter = class extends HlsAudioMixin(HTMLAudioAdapter) {};

//#endregion
export { HlsAudioAdapter as t };
//# sourceMappingURL=adapter-CTKsoX5B.js.map