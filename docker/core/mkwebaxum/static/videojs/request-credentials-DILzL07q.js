import { f as isUndefined } from "./predicate-3rF1m2uv.js";
import { t as listen } from "./listen-CO63BggB.js";
import { t as isWebKitAirPlayCapable } from "./webkit-DXmoZ2Wz.js";
import { C as reportUnsupportedTrackConditions, Dt as attachMediaSourceAsSourceElement, E as resolveAudioTrack, Et as defineBehavior, G as getCdnId, Gt as SVTA_NO_SUPPORTED_VIDEO_TRACK, J as applyRules, K as getOrderedCdnIds, Mt as peek, Ot as defineFeature, Pt as update, Q as sameCandidateSet, Qt as getTracksByType, R as resolveRequestCredentials, U as emitError, Wt as SVTA_NO_SUPPORTED_AUDIO_TRACK, X as excludeUnplayableTracks, Xt as findTrackById, Z as preferCodecFamilies, Zt as getCodecFamilies, _ as isStandardPreload, an as effect, b as getMinBufferedEnd, bt as TEXT_TYPE_CONFIG, dt as matchesPartialTrack, en as deriveStreamType, gt as loadAudioSegments, h as gateFirstParseOnAnchor, ht as tracksUnderPixelArea, in as isResolvedTrack, jt as computed, k as setupAudioBufferActors, m as establishStartMediaTime, mt as smallestCoveringPixelArea, nn as getSessionData, pt as pickTextTrackFromTracks, q as applyConstraints, rn as isResolvedPresentation, tn as getMediaPlaylistMetadata, wt as createMachineReactor, xt as VIDEO_TYPE_CONFIG, yt as AUDIO_TYPE_CONFIG, z as canPlayTrack } from "./error-surface-BHz7DxGx.js";
import { h as getBandwidthEstimate, m as DEFAULT_BANDWIDTH_CONFIG } from "./session-data-6y7aXrmJ.js";
import { n as APPLE_HLS_CHAPTERS_DATA_ID, t as loadChaptersTracks } from "./load-chapters-tracks-ca_JiVd8.js";

//#region ../utils/dist/time/sleep.js
/**
* Resolve after `ms` milliseconds. Pass a `signal` to make it cancellable: the timer is cleared and the promise rejects
* with the signal's reason as soon as the signal aborts (including if it's already aborted).
*
* @internal
*/
function sleep(ms, signal) {
	return new Promise((resolve, reject) => {
		if (signal?.aborted) {
			reject(signal.reason);
			return;
		}
		const onAbort = () => {
			clearTimeout(timer);
			reject(signal?.reason);
		};
		const timer = setTimeout(() => {
			signal?.removeEventListener("abort", onAbort);
			resolve();
		}, ms);
		signal?.addEventListener("abort", onAbort, { once: true });
	});
}

//#endregion
//#region ../spf/dist/dev/media/hls/reload-policy.js
/** Reload cadence when a playlist carries no usable target duration. */
const FALLBACK_TARGET_DURATION = 6;
/**
* Default `HOLD-BACK` as a multiple of the target duration when the playlist declares none — the HLS spec default (RFC
* 8216bis `EXT-X-SERVER-CONTROL`).
*/
const HOLD_BACK_TARGET_MULTIPLIER = 3;
/** Identity of a reload snapshot — window position + length. Changes when the window slid or grew. */
function snapshotSignature(track) {
	return `${getMediaPlaylistMetadata(track)?.mediaSequence ?? 0}:${track.segments.length}`;
}
function targetDurationOf(track) {
	return getMediaPlaylistMetadata(track)?.targetDuration || FALLBACK_TARGET_DURATION;
}
/**
* Live media-playlist reload cadence, per RFC 8216bis §6.3.4 — a {@link RecurrencePolicy} for a `RecurringRunner`
* re-resolving the selected track. Structurally matches `RecurrencePolicy<ResolvedTrack>` without importing it (media
* stays core-free): `current` is the freshly resolved track, `previous` the prior resolved snapshot.
*
* - Complete playlist (VoD, or live that hit `#EXT-X-ENDLIST`) → `null`: stop. Keys off `Track.duration` (finite once
*   complete), the single completeness source of truth.
* - Unchanged window (same media sequence + segment count as `previous`) → poll at half the target duration; a
*   moved/grown window (or the first reload) → full target duration.
*
* A failed reload doesn't reach here — the rejection propagates through the `RecurringRunner`; transient-failure
* recovery belongs at the fetch layer.
*
* Returned delays are milliseconds.
*/
function mediaPlaylistReloadDelay(current, previous) {
	if (Number.isFinite(current.duration)) return null;
	const target = targetDurationOf(current);
	return (!previous || snapshotSignature(current) !== snapshotSignature(previous) ? target : target / 2) * 1e3;
}
/**
* Target live latency (seconds) for a resolved track — how far behind the live edge the playhead should sit. Prefers
* the server's declared `EXT-X-SERVER-CONTROL` `HOLD-BACK`, falling back to {@link HOLD_BACK_TARGET_MULTIPLIER}× the
* target duration when absent (the spec default). This is the HLS side of the format-neutral `resolveLiveLatency` seam
* consumed by `seek-to-live-edge`; a DASH engine supplies its own (`suggestedPresentationDelay`).
*
* Only `HOLD-BACK` — never `PART-HOLD-BACK`, which assumes partial-segment playback. See
* {@link MediaPlaylistMetadata.holdBack}.
*/
function liveLatencyFor(track) {
	return getMediaPlaylistMetadata(track)?.holdBack ?? HOLD_BACK_TARGET_MULTIPLIER * targetDurationOf(track);
}
/**
* Resolve the target live latency for a presentation's timeline-bearing track — the HLS engine injects this as
* `seekToLiveEdge`'s format-neutral `resolveLiveLatency` seam. `0` when there is no resolved track to read (the
* behavior then seeks straight to the edge).
*/
function resolveLiveLatency(presentation, trackId) {
	if (!isResolvedPresentation(presentation) || !trackId) return 0;
	const track = findTrackById(presentation, trackId);
	return track && isResolvedTrack(track) ? liveLatencyFor(track) : 0;
}

//#endregion
//#region ../spf/dist/dev/media/live-window.js
/**
* Derive the live window of the track with the given id — the single source of truth for "where is live," consumed (via
* `liveWindowFromState`) by the seek-to-live-edge and live-seekable-range behaviors so neither re-derives (or
* re-presumes) the window shape. Type-agnostic: the caller decides which track bears the timeline (video when present,
* else audio).
*
* The window is a **live read** over the current `segments` array — never stored on the track. The anchor triple is
* frozen per source; per-segment `startTime` values are re-derived against it on every reload, so the edge here slides
* while the origin stays fixed (see `internal/design/spf/live-presentation-timeline-model.md`).
*
* Returns `null` when there is no live edge to track: an unresolved presentation or track, a track with no segments, or
* a **complete** playlist (VoD, or live that has ended — a finite `Track.duration`).
*/
function liveWindowFor(presentation, trackId) {
	if (!isResolvedPresentation(presentation) || !trackId) return null;
	const track = findTrackById(presentation, trackId);
	if (!track || !isResolvedTrack(track) || track.segments.length === 0) return null;
	if (Number.isFinite(track.duration)) return null;
	const { segments } = track;
	const last = segments[segments.length - 1];
	return {
		start: segments[0].startTime,
		end: last.startTime + last.duration
	};
}
/**
* `targetLiveWindow` per the media-ui-extensions live-edge proposal: `NaN` for on-demand (or nothing resolved yet), `0`
* for standard sliding-window live, `Infinity` for DVR (`#EXT-X-PLAYLIST-TYPE:EVENT` — the window grows from the
* start). Read from the timeline-bearing track's playlist metadata.
*/
function deriveTargetLiveWindow(presentation, trackId) {
	if (!isResolvedPresentation(presentation) || !trackId) return NaN;
	const track = findTrackById(presentation, trackId);
	if (!track || !isResolvedTrack(track)) return NaN;
	const metadata = getMediaPlaylistMetadata(track);
	if (!metadata) return NaN;
	if (metadata.playlistType === "EVENT") return Number.POSITIVE_INFINITY;
	return deriveStreamType(metadata) === "live" ? 0 : NaN;
}

//#endregion
//#region ../spf/dist/dev/playback/primitives/live-window.js
/**
* The id of the timeline-bearing track: the selected video track when present, else the selected audio track. The
* single pick both the window derivation and the live-latency resolution (`seek-to-live-edge`) share, so they can't
* drift.
*/
function liveTrackId(state) {
	return state.selectedVideoTrackId?.get() ?? state.selectedAudioTrackId?.get();
}
/**
* One type's window: the selected track's when it's resolved, else any resolved track of the type. The fallback keeps
* the window from blinking to `null` mid ABR / user switch — all renditions of a type are time-aligned and share the
* anchor, so a deselected track's window may trail live by up to one reload during the switch gap (acceptable; the
* selected track's fresh window resumes the moment it resolves). A null blink would flip `seekToLiveEdge` out of `live`
* and stall the seekable-range writer.
*/
function liveWindowForType(presentation, selectedId, type) {
	if (!presentation || selectedId === void 0) return null;
	const selected = liveWindowFor(presentation, selectedId);
	if (selected) return selected;
	for (const track of getTracksByType(presentation, type)) {
		const window = liveWindowFor(presentation, track.id);
		if (window) return window;
	}
	return null;
}
function liveWindowFromState(state) {
	const presentation = state.presentation.get();
	const video = liveWindowForType(presentation, state.selectedVideoTrackId?.get(), "video");
	const audio = liveWindowForType(presentation, state.selectedAudioTrackId?.get(), "audio");
	if (video && audio) {
		const start = Math.max(video.start, audio.start);
		const end = Math.min(video.end, audio.end);
		return start < end ? {
			start,
			end
		} : null;
	}
	return video ?? audio;
}
/**
* Resolve the live edge — the window bounds plus the target playhead position — from a behavior's setup arguments.
* Bundles the window geometry and the format-specific latency policy (`config.resolveLiveLatency`) so the consuming
* behavior never has to compose them; it just forwards its `{ state, config }`. `null` when there is no live edge (VOD
* / ended / unresolved).
*
* Reads signals lazily — call it inside a reactive context (an effect).
*/
function getLiveEdge({ state, config }) {
	const window = liveWindowFromState(state);
	if (!window) return null;
	const latency = config?.resolveLiveLatency?.(state.presentation.get(), liveTrackId(state)) ?? 0;
	return {
		...window,
		liveEdgeStart: Math.max(window.start, window.end - latency)
	};
}

//#endregion
//#region ../spf/dist/dev/core/composition/define-external-signals.js
/**
* Behavior factory that defines a composition's **external signals**: state and context keys set and updated from
* outside the composition (typically by an adapter) that no composed behavior declares, because the behaviors that
* consult them only read them, and read them optionally.
*
* It creates no signals itself. The behavior it returns declares the keys, and `createComposition` creates their
* signals as it does for every other behavior's keys, so they also reach the composition's inferred state and context
* types. The setup does nothing.
*
* Curried so the two shapes are named explicitly while the key lists are inferred and checked against them: leaving out
* a key of `ExternalState` or `ExternalContext` is a compile error. Both shapes must name their keys and make every key
* optional, and both default to none.
*
* @example
*   ```ts
*   const externalSignals = defineExternalSignals<UserTrackSelectionState<'audio'>>()({ state: ['userAudioTrackSelection'] });
*   const behaviors = [resolvePresentation, switchAudioTrack, externalSignals] as const;
*   type State = ResolveBehaviorState<typeof behaviors>; // includes `userAudioTrackSelection`
*   ```;
*/
function defineExternalSignals() {
	return (keys) => ({
		stateKeys: keys.state ?? [],
		contextKeys: keys.context ?? [],
		setup: () => {}
	});
}

//#endregion
//#region ../spf/dist/dev/behaviors/dom/airplay.js
/**
* How long WebKit's wireless flag must read _inactive_ before the session-driven `loadingSuspended` clears.
*
* Measured on Safari 26.4 (macOS): when an AirPlay session engages, Safari closes the ManagedMediaSource and — while
* its pipeline switches to the native-HLS fallback source — transiently reports `webkitCurrentPlaybackTargetIsWireless
* === false`, firing the changed event. Trusting an instantaneous inactive reading would release `setupMediaSource`'s
* rebuild hold mid-handoff; its recovery `load()` then destroys the very session being established. Rising edges apply
* immediately; only the falling edge waits out this settle window.
*/
const REMOTE_INACTIVE_SETTLE_MS = 1e3;
function deriveState$4(mediaElement, authorDisabledRemotePlayback) {
	if (!mediaElement || !isWebKitAirPlayCapable(mediaElement)) return "preconditions-unmet";
	if (authorDisabledRemotePlayback) return "preconditions-unmet";
	return "airplay-capable";
}
function setupAirPlaySetup({ state, context }) {
	const derivedStateSignal = computed(() => deriveState$4(context.mediaElement.get(), state.disableRemotePlayback.get()));
	return createMachineReactor({
		initial: "preconditions-unmet",
		monitor: () => derivedStateSignal.get(),
		states: {
			"preconditions-unmet": {},
			"airplay-capable": { entry: () => {
				const mediaElement = context.mediaElement.get();
				const isSessionActive = () => !!mediaElement.webkitCurrentPlaybackTargetIsWireless;
				let settleTimer;
				let sessionPresentationUrl;
				let pendingRestore;
				/** Presentation an already-written `state.startPosition` belongs to. */
				let restoreOwnerUrl;
				let resumeWhenRestored = false;
				const sync = () => {
					if (isSessionActive()) {
						clearTimeout(settleTimer);
						settleTimer = void 0;
						if (!peek(state.loadingSuspended)) sessionPresentationUrl = peek(state.presentation)?.url;
						state.loadingSuspended.set(true);
					} else if (peek(state.loadingSuspended)) settleTimer ??= setTimeout(() => {
						settleTimer = void 0;
						const stillActive = isSessionActive();
						if (!stillActive) {
							const ownerUrl = sessionPresentationUrl;
							sessionPresentationUrl = void 0;
							if (peek(state.presentation)?.url === ownerUrl) pendingRestore = {
								position: mediaElement.currentTime,
								wasPlaying: !mediaElement.paused,
								presentationUrl: ownerUrl
							};
						}
						state.loadingSuspended.set(stillActive);
					}, REMOTE_INACTIVE_SETTLE_MS);
					else state.loadingSuspended.set(false);
				};
				const listenerCleanup = new AbortController();
				listen(mediaElement, "webkitcurrentplaybacktargetiswirelesschanged", sync, { signal: listenerCleanup.signal });
				listen(mediaElement, "emptied", () => {
					if (!pendingRestore) return;
					const { position, wasPlaying, presentationUrl } = pendingRestore;
					pendingRestore = void 0;
					if (peek(state.presentation)?.url !== presentationUrl) return;
					state.startPosition.set(position);
					restoreOwnerUrl = presentationUrl;
					resumeWhenRestored = wasPlaying;
				}, { signal: listenerCleanup.signal });
				const disposeSourceChangeEnd = effect(() => {
					const url = state.presentation.get()?.url;
					if (!peek(state.loadingSuspended) || url === sessionPresentationUrl) return;
					sessionPresentationUrl = void 0;
					clearTimeout(settleTimer);
					settleTimer = void 0;
					state.loadingSuspended.set(false);
				});
				const disposeRestoreWatch = effect(() => {
					const url = state.presentation.get()?.url;
					const position = state.startPosition.get();
					if (!restoreOwnerUrl) return;
					if (url !== restoreOwnerUrl) {
						restoreOwnerUrl = void 0;
						resumeWhenRestored = false;
						if (position !== void 0) state.startPosition.set(void 0);
						return;
					}
					if (position !== void 0) return;
					restoreOwnerUrl = void 0;
					if (!resumeWhenRestored) return;
					resumeWhenRestored = false;
					mediaElement.play().catch((err) => {
						console.warn("[setupAirPlay] session-end resume play() rejected — staying paused:", err);
					});
				});
				let sourceEl = null;
				const disposeSource = effect(() => {
					const hasMediaSource = !!context.mediaSource.get();
					const sessionActive = !!state.loadingSuspended.get();
					const url = state.presentation.get()?.url ?? "";
					if (!hasMediaSource && !sessionActive) {
						sourceEl?.remove();
						sourceEl = null;
					} else if (hasMediaSource && (!sourceEl || sourceEl.parentNode !== mediaElement)) {
						sourceEl = document.createElement("source");
						sourceEl.type = "application/x-mpegURL";
						mediaElement.append(sourceEl);
						mediaElement.disableRemotePlayback = false;
					}
					if (sourceEl) sourceEl.src = url;
				});
				sync();
				return () => {
					disposeSource();
					disposeSourceChangeEnd();
					disposeRestoreWatch();
					listenerCleanup.abort();
					clearTimeout(settleTimer);
					sourceEl?.remove();
					sourceEl = null;
					mediaElement.disableRemotePlayback = true;
					state.loadingSuspended.set(false);
					if (restoreOwnerUrl) {
						restoreOwnerUrl = void 0;
						resumeWhenRestored = false;
						if (peek(state.startPosition) !== void 0) state.startPosition.set(void 0);
					}
				};
			} }
		}
	});
}
/**
* **Lets WebKit hand an MSE stream to an AirPlay receiver, by giving the element a native-HLS fallback source and
* suspending loading while a session holds.** MSE streams can't be handed to an AirPlay receiver directly. The
* WebKit-recommended workaround is to append a fallback `<source type="application/x-mpegURL">` carrying the original
* manifest URL: Safari exposes the AirPlay picker and, when a wireless target is selected, plays that native-HLS source
* on the receiver. The session state (WebKit's wireless flag, falling edge debounced — see `REMOTE_INACTIVE_SETTLE_MS`)
* is written straight to its policy consequences, declared here so the cause→policy mapping stays with the feature:
*
* - `state.loadingSuspended` — held while the session is live. Observed by the `loadXSegments` dispatchers (no fetching
*   alongside the receiver) and by `setupMediaSource` (its post-close rebuild waits — attaching runs `element.load()`
*   under the live receiver, which destroys a session still being established). The suspension this behavior holds
*   doubles as its own session fact — same writer, same edges.
* - `state.startPosition` — one-shot command: the position is captured from the element at the session's settled end
*   (still receiver-mirrored) and written once the rebuild's `load()` resets the element (its `'emptied'`), so
*   `applyStartPosition` applies it to the rebuilt source — never to the pre-rebuild element — and starts it where the
*   receiver left off. The playing state rides the same snapshot but stays behavior-local: this behavior itself calls
*   `play()` once the command has been _consumed_ — i.e. after the seek — when the receiver was playing at session end.
*   The whole restore is bound to the presentation the session owned and retracted if that changes.
*
* A source change during a live session releases the hold rather than deferring until the session ends, so the rebuild
* runs and WebKit switches the receiver to the newly-built AirPlay alternate. Measured, not contracted — see the
* source-change effect in `setupAirPlaySetup`.
* https://webkit.org/blog/15036/how-to-use-media-source-extensions-with-airplay/
*
* Single-positive-state reactor (`'preconditions-unmet'` ↔ `'airplay-capable'`): gated on a WebKit-AirPlay-capable
* media element being in scope. The entry — gated on `context.mediaSource` — appends the fallback `<source>` (kept
* current from `state.presentation`) and enables the AirPlay picker once the MediaSource is open, removing the source
* the moment the MediaSource detaches so it never survives an MSE teardown. State-exit cleanup (author opt-out, detach,
* source reset, behavior destroy) removes the source and restores the element's `disableRemotePlayback` default. No-op
* on non-WebKit platforms (Chromium, Firefox) — `deriveState` never leaves `'preconditions-unmet'`.
*
* MMS and AirPlay want _opposite_ values of `disableRemotePlayback` on the same element, so it is **sequenced**:
*
* - **MMS needs `true` to open.** `setupMediaSource` sets `disableRemotePlayback = true` when it attaches a
*   ManagedMediaSource — Safari won't fire `sourceopen` (and MSE playback never starts) otherwise.
* - **AirPlay needs `false` to offer the picker.** Flipping to `false` _before_ the source opens would prevent
*   `sourceopen`, so the flip is gated on `context.mediaSource` — which `setupMediaSource` publishes exactly once the
*   MS is open. Re-fires per source (the signal clears + republishes on reset).
* - **Author opt-out wins.** `state.disableRemotePlayback` is the author's intent, written only by the media adapter's
*   IDL property; MMS/programmatic code touch the element's own `disableRemotePlayback` instead. A `true` there is
*   unambiguously the author's choice to disable remote playback, so it holds the machine in `'preconditions-unmet'`
*   and nothing is set up.
*/
const setupAirPlay = defineBehavior({
	stateKeys: [
		"presentation",
		"disableRemotePlayback",
		"loadingSuspended",
		"startPosition"
	],
	contextKeys: ["mediaElement", "mediaSource"],
	setup: setupAirPlaySetup
});

//#endregion
//#region ../spf/dist/dev/features/airplay.js
/** The behaviors the feature composes, in setup order. */
const behaviors$7 = [setupAirPlay, defineExternalSignals()({ state: ["disableRemotePlayback"] })];
/** The config defaults the feature contributes. */
const defaultConfig$7 = { attachMediaSource: attachMediaSourceAsSourceElement };
/** The state values the feature seeds. */
const initialState$7 = {};
/**
* Plays to AirPlay receivers (WebKit only; inert elsewhere), unless the user sets `disableRemotePlayback`. Attaches the
* MediaSource through a `<source>` element, which the native fallback the receiver plays requires.
*
* Requires a writer of `context.mediaSource`, such as `mediaSourceFeature`; without one, AirPlay is never offered.
*/
const airPlayFeature = defineFeature({
	behaviors: behaviors$7,
	defaultConfig: defaultConfig$7,
	initialState: initialState$7
});

//#endregion
//#region ../spf/dist/dev/media/abr/quality-selection.js
/** Default quality selection configuration. Values match Shaka Player upgrade threshold (0.85 = 15% headroom). */
const DEFAULT_QUALITY_CONFIG = {
	safetyMargin: .85,
	upgradeMargin: 1.15
};
/**
* Resolution as a total pixel count (`width × height`), the basis for comparing two tracks at the same bitrate. Missing
* dimensions count as 0, so tracks without resolution metadata (e.g. audio) area-compare equal.
*/
function resolutionArea(track) {
	return (track.width ?? 0) * (track.height ?? 0);
}

//#endregion
//#region ../spf/dist/dev/behaviors/track-switching.js
/**
* User intent — a soft filter. Narrows to tracks matching the partial-track selection in `user*TrackSelection`; an
* empty match falls through (the composer skips it) to the unfiltered set — e.g. a stale id from a previous source.
*/
function filterByUserSelection(tracks, { state, config }) {
	const key = config.userSelectionKey;
	if (!key) return tracks;
	const filter = state[key]?.get();
	return filter ? tracks.filter((track) => matchesPartialTrack(track, filter)) : tracks;
}
/**
* Audio preference — a soft filter. Prefer the configured language, then manifest-default tracks; an empty match falls
* through to bandwidth ranking. Keep every matching rendition so preference doesn't pin a bitrate.
*/
function preferAudioPolicy(tracks, { config }) {
	const preferredLanguage = config?.preferredAudioLanguage;
	if (preferredLanguage) {
		const matches = tracks.filter((track) => track.language === preferredLanguage);
		if (matches.length) return matches;
	}
	return tracks.filter((track) => track.default === true);
}
/**
* Player-resolution cap — a soft filter, video only. Narrows to the renditions worth delivering at the player element's
* rendered size, so a small embed doesn't pull segments nobody can perceive. The tighter sibling of
* `screenResolutionCap`: the element's box, not the screen behind it.
*
* The cap is the _smallest tier that still covers the player_, and everything at or below it survives — not "everything
* at or below the player's area," which under-serves a player falling between two tiers. Take an 800×450 player against
* a 360p/720p/1080p ladder: only 360p is below it, so capping at the player's area would hold an 800-px-wide box to a
* 640-px-wide picture. The honest answer is the tier above, 720p, with 360p left in for the ranker.
* `smallestCoveringPixelArea` picks that cap; `tracksUnderPixelArea` — the same filter `screenResolutionCap` narrows
* with — applies it.
*
* Renditions declaring no width or height compare as area `0` and are never capped out — they can't be judged against
* the player, and dropping them could strand a source whose renditions all omit it.
*
* Runs _after_ `preferActiveCdn`, so it narrows within the host already chosen. Ahead of it, a cap that pruned every
* rendition of the preferred CDN would leave the scope to fall to the next one with survivors — a size preference
* silently moving playback to another host. Redundant streams normally mirror the same ladder, which makes that a
* nonstandard-but-legal mismatch across CDNs rather than an everyday case; the ordering costs nothing either way.
*
* Reading `state.playerResolution` through its signal is what subscribes the chain to resizes; `undefined` — no signal
* composed, or nothing to measure — means "don't cap" rather than a cap of zero, so the chain proceeds unnarrowed.
*/
function playerResolutionCap(tracks, { state }) {
	const playerResolution = state.playerResolution?.get();
	if (!playerResolution) return tracks;
	const cap = smallestCoveringPixelArea(tracks, playerResolution.width * playerResolution.height);
	return tracksUnderPixelArea(tracks, cap);
}
/**
* Failed-CDN constraint — a _hard_ filter (constraints pre-pass), shared by video and audio. Removes tracks served from
* a CDN currently in failover cooldown (`failedCdns`, written by the failover monitor). Removed tracks are never
* attempted; the scope then narrows to the next surviving CDN in `cdnPriority`, and snaps back to the primary once it
* leaves cooldown.
*
* Passes everything through when there's no `failedCdns` signal/value. When it prunes _every_ track (all CDNs cooled
* down), the empty result is preserved (per `applyConstraints`) — "nothing playable," which clears the selection (no
* pick); a later CDN recovery refills the candidate set and re-picks.
*/
function excludeFailedCdns(tracks, { state, config }) {
	const failed = state.failedCdns?.get();
	if (!failed?.length) return tracks;
	const getCdnId$1 = config.getCdnId ?? getCdnId;
	const failedSet = new Set(failed);
	return tracks.filter((track) => !failedSet.has(getCdnId$1(track.url)));
}
/**
* Active-CDN scope — a soft filter, shared by video and audio. Narrows to the highest-priority CDN in `cdnPriority`
* (owned by `deriveCdnPriority`) that still has tracks, so every track type stays on one CDN. A redundant-streams
* source lists the same renditions on multiple hosts; this keeps the pick on one host rather than letting the ranker
* drift across them.
*
* "Active" is derived, not stored: constraints run before the rule chain, so a failed CDN's tracks are already pruned
* by the time this runs — "first CDN with survivors" _is_ the active CDN, and it falls through to the next on failover
* (and snaps back to the primary when it recovers). Content steering reorders `cdnPriority`; this rule just honors the
* order.
*
* Soft-filter semantics: passes through when there's no `cdnPriority` signal/value (no preference) or when nothing
* matches (`applyRules` skips an empty result). Non-redundant sources have one CDN, so the narrow is a no-op.
*
* The CDN-id derivation defaults to origin-based `getCdnId`, overridable via the `getCdnId` config — it must match the
* one `deriveCdnPriority` used to build `cdnPriority`, or no track's CDN would ever equal an entry.
*/
function preferActiveCdn(tracks, { state, config }) {
	const cdnPriority = state.cdnPriority?.get();
	if (!cdnPriority?.length) return tracks;
	const getCdnId$2 = config.getCdnId ?? getCdnId;
	for (const cdn of cdnPriority) {
		const tracksUsingCdn = tracks.filter((track) => getCdnId$2(track.url) === cdn);
		if (tracksUsingCdn.length) return tracksUsingCdn;
	}
	return tracks;
}
/**
* Codec-family sticky constraint — a _hard_ filter for the constraints pre-pass, shared by video and audio, and the
* reason the re-evaluating variants can run ABR over a mixed-codec source at all: SPF implements no
* `SourceBuffer.changeType()`, so once segments of the selected track's codec families are what a buffer was created
* for, a pick outside them could never play — constraint semantics ("can't play here"), not preference. Removes the
* candidates whose codec-family set doesn't _equal_ the selected track's (set-equality so a muxed `hvc1,mp4a` rendition
* can't pass as a match for `avc1,mp4a` on its audio half). Purely relational: the lock _is_ the current selection's
* families, no state of its own. Pruned pre-pass, a cross-family user selection mid-stream never reaches the user
* filter, which falls through unhonored instead of killing playback. Before any selection exists (the initial pick) it
* removes nothing, which is what leaves the initial family choice to the rule chain (user intent, then
* `preferCodecFamilies`).
*
* The selected track's families come from the _presentation's_ track list, not the already-pruned candidates: a CDN
* entering cooldown may prune the current track itself while same-family renditions survive on another host, and the
* lock must carry over to them. Removes nothing when: no selection yet, a selection the presentation no longer carries,
* or a selected track without codecs (then no candidate's compatibility is decidable — same inertness as
* `preferCodecFamilies` on a codec-less ladder).
*
* When the selected family genuinely vanishes from the playable set, the pre-pass empties: the behavior reports the
* type's no-supported-track code and clears the selection — an explicable stop instead of a cross-family append
* surfacing as an opaque decode error. Clearing also releases the lock (it keys on the live selection), so a following
* pick may land in the surviving family on freshly-created buffer actors; whether the append layer survives that
* rebuild is the changeType gap, out of this constraint's hands. If `changeType` support ever lands, this constraint is
* what relaxes.
*
* Reading the selection here — a signal the picking effect itself writes, from inside the candidate-set computed — is
* safe only because the effect scheduler revalidates an effect's sources after each run; see `core/signals/effect.ts`
* (`revalidateSources`) for the lost-wakeup it prevents.
*
* Composed only into the re-evaluating `switch*` variants: the pinned `selectVideoTrack` (background compositions)
* evaluates once and never re-picks, so it's immune by construction.
*/
function stickToSelectedCodecs(tracks, { state, config }) {
	const currentId = state[config.selectionKey].get();
	if (!currentId) return tracks;
	const presentation = state.presentation.get();
	if (!isResolvedPresentation(presentation)) return tracks;
	const current = config.getTracks(presentation).find((track) => track.id === currentId);
	const families = current && getCodecFamilies(current);
	if (!families) return tracks;
	return tracks.filter((track) => {
		const candidateFamilies = getCodecFamilies(track);
		return !!candidateFamilies && candidateFamilies.length === families.length && candidateFamilies.every((family) => families.includes(family));
	});
}
/**
* Bandwidth ranking — the terminal sort, shared by video and audio. Orders by the throughput estimate: tracks within
* the bandwidth threshold first (fitting), highest bitrate first; then over-threshold tracks, least-over first. The
* head is the best-quality track that fits, falling back to the smallest over-throughput track when nothing fits.
*
* Hysteresis without temporal state: the current track's effective bitrate is boosted by `upgradeMargin` in the fitting
* sort, so a higher track only outranks it once it clears `current.bitrate * upgradeMargin` (no flapping on marginal
* bandwidth gains). Downgrades fall out for free — a current track over the threshold isn't in the fitting set to be
* boosted, so the best fit (a downgrade) wins immediately. Equal-bitrate tracks break by resolution (higher `width ×
* height` first), so an equal-bitrate ladder never picks a lower- quality rendition by manifest order; audio tracks
* carry no dimensions, so they area-compare equal and a stable sort keeps their candidate order (e.g. same-bitrate
* language variants). Early-bail skips this rule when a prior one narrowed to a single track, so the estimate is
* neither read nor subscribed while that holds.
*/
function rankByBandwidth(tracks, { state, config }) {
	const safetyMargin = config.quality?.safetyMargin ?? DEFAULT_QUALITY_CONFIG.safetyMargin;
	const upgradeMargin = config.quality?.upgradeMargin ?? DEFAULT_QUALITY_CONFIG.upgradeMargin;
	const initialBandwidth = config.initialBandwidth ?? 5e6;
	const bandwidthConfig = {
		...DEFAULT_BANDWIDTH_CONFIG,
		...config.bandwidth
	};
	if (!state.bandwidthState) console.debug("[track-switching] rankByBandwidth: no bandwidthState signal in composition; ranking on initialBandwidth");
	const threshold = getBandwidthEstimate(state.bandwidthState?.get(), initialBandwidth, bandwidthConfig) * safetyMargin;
	const currentId = state[config.selectionKey].get();
	const bitrate = (track) => track.bandwidth ?? 0;
	const rank = (track) => track.id === currentId ? bitrate(track) * upgradeMargin : bitrate(track);
	const fitting = tracks.filter((track) => bitrate(track) <= threshold).sort((a, b) => rank(b) - rank(a) || resolutionArea(b) - resolutionArea(a));
	const over = tracks.filter((track) => bitrate(track) > threshold).sort((a, b) => bitrate(a) - bitrate(b) || resolutionArea(b) - resolutionArea(a));
	return [...fitting, ...over];
}
/**
* Default final pick: the chain head. `applyRules` never narrows to nothing and early-bails to a single survivor, so
* video and audio always converge to a track and the head is the pick.
*/
function selectChainHead(candidates) {
	return candidates[0].id;
}
/**
* Terminal pick for text — the `resolveSelection` the text variant supplies. Resolves the standing
* `userTextTrackSelection` intent against the chain's survivors (already CDN-failover-pruned and active-CDN-scoped):
*
* - `'off'` → no selection (clear the signal). Sticky through re-evaluation, so a live refresh or failover re-run can't
*   re-assert a default.
* - Explicit `Partial<TextTrack>` → narrow to the match (language-based). A stale pick whose match is gone (e.g. the
*   language dropped on a source change) falls through to the default policy.
* - Auto (`undefined`) → the opt-in default policy (`preferredSubtitleLanguage` → `DEFAULT=YES + AUTOSELECT=YES` → none),
*   via `pickTextTrackFromTracks`.
*
* Returning `undefined` is a real outcome (captions are opt-in), which is why the text variant relies on
* `setupTrackSwitching`'s no-selection seam.
*/
function pickResolvedTextTrack(candidates, { state, config }) {
	const intent = state.userTextTrackSelection?.get();
	if (intent === "off") return void 0;
	if (intent) {
		const matched = candidates.filter((track) => matchesPartialTrack(track, intent));
		if (matched.length) return matched[0].id;
	}
	return pickTextTrackFromTracks(candidates, config);
}
function setupTrackSwitching(deps) {
	const { state, config } = deps;
	const { selectionKey, getTracks, rules, resolveSelection = selectChainHead, noSupportedTrackCode } = config;
	const derivedStateSignal = computed(() => isResolvedPresentation(state.presentation.get()) ? "presentation-resolved" : "presentation-unresolved");
	const candidateSet = computed(() => {
		const presentation = state.presentation.get();
		if (!isResolvedPresentation(presentation)) return [];
		return applyConstraints(config.constraints ?? [], getTracks(presentation), deps);
	}, { equals: sameCandidateSet });
	return createMachineReactor({
		initial: "presentation-unresolved",
		monitor: () => derivedStateSignal.get(),
		states: {
			"presentation-unresolved": {},
			"presentation-resolved": {
				entry: () => () => state[selectionKey].set(void 0),
				effects: [() => {
					const tracks = candidateSet.get();
					if (!tracks.length) {
						const presentation = peek(state.presentation);
						if (isResolvedPresentation(presentation) && getTracks(presentation).length > 0) {
							if (noSupportedTrackCode !== void 0) emitError(state, {
								code: noSupportedTrackCode,
								data: { selectionKey }
							});
							state[selectionKey].set(void 0);
						}
						return;
					}
					const candidates = applyRules(rules, tracks, deps);
					if (!candidates.length) {
						console.error("[track-switching] applyRules returned no candidates");
						return;
					}
					state[selectionKey].set(resolveSelection(candidates, deps));
				}]
			}
		}
	});
}
/** Default video pre-pass: failed CDNs, capability, codec-family stickiness. */
const DEFAULT_VIDEO_CONSTRAINTS = [
	excludeFailedCdns,
	excludeUnplayableTracks,
	stickToSelectedCodecs
];
/** Default video chain: user intent, codec family, active CDN, player resolution, then the bandwidth ranker. */
const DEFAULT_VIDEO_RULES = [
	filterByUserSelection,
	preferCodecFamilies,
	preferActiveCdn,
	playerResolutionCap,
	rankByBandwidth
];
/** Default audio pre-pass: the video one over audio candidates. */
const DEFAULT_AUDIO_CONSTRAINTS = [
	excludeFailedCdns,
	excludeUnplayableTracks,
	stickToSelectedCodecs
];
/** Default audio chain: user intent, codec family, active CDN, language/default preference, then bandwidth ranking. */
const DEFAULT_AUDIO_RULES = [
	filterByUserSelection,
	preferCodecFamilies,
	preferActiveCdn,
	preferAudioPolicy,
	rankByBandwidth
];
/** Default text pre-pass: failed CDNs only — an MSE codec probe is the wrong question for WebVTT. */
const DEFAULT_TEXT_CONSTRAINTS = [excludeFailedCdns];
/** Default text chain: the active-CDN scope; the pick itself is the text terminal's (`pickResolvedTextTrack`). */
const DEFAULT_TEXT_RULES = [preferActiveCdn];
/**
* Manage `selectedVideoTrackId`: pick a default on src load, dynamically adjust based on bandwidth, clear on src
* unload. Honors `userVideoTrackSelection` as a partial-track constraint on candidates; short-circuits ABR when the
* constraint narrows to a single track.
*
* @example
*   const reactor = switchVideoTrack.setup({ state });
*/
const switchVideoTrack = defineBehavior({
	stateKeys: ["presentation", "selectedVideoTrackId"],
	contextKeys: [],
	setup: ({ state, config, ...otherProps }) => setupTrackSwitching({
		...otherProps,
		state,
		config: {
			...config,
			selectionKey: VIDEO_TYPE_CONFIG.selectedKey,
			userSelectionKey: VIDEO_TYPE_CONFIG.userSelectionKey,
			getTracks: (presentation) => getTracksByType(presentation, "video"),
			constraints: config?.[VIDEO_TYPE_CONFIG.constraintsKey] ?? DEFAULT_VIDEO_CONSTRAINTS,
			rules: config?.[VIDEO_TYPE_CONFIG.rulesKey] ?? DEFAULT_VIDEO_RULES,
			noSupportedTrackCode: SVTA_NO_SUPPORTED_VIDEO_TRACK
		}
	})
});
/**
* Manage `selectedAudioTrackId`: pick a default on src load, narrow by `userAudioTrackSelection` filter, re-pick on
* filter change, clear on src unload.
*
* Mid-stream flush on language switch is handled by the segment-loader's `planTasks` (see
* `playback/actors/dom/segment-loader.ts`) — not this behavior. Same split as the video pipeline: key owner writes;
* loader orchestrates segment + flush plans.
*
* @example
*   const reactor = switchAudioTrack.setup({ state });
*/
const switchAudioTrack = defineBehavior({
	stateKeys: ["presentation", "selectedAudioTrackId"],
	contextKeys: [],
	setup: ({ state, config, ...otherProps }) => setupTrackSwitching({
		...otherProps,
		state,
		config: {
			...config,
			selectionKey: AUDIO_TYPE_CONFIG.selectedKey,
			userSelectionKey: AUDIO_TYPE_CONFIG.userSelectionKey,
			getTracks: (presentation) => getTracksByType(presentation, "audio"),
			constraints: config?.[AUDIO_TYPE_CONFIG.constraintsKey] ?? DEFAULT_AUDIO_CONSTRAINTS,
			rules: config?.[AUDIO_TYPE_CONFIG.rulesKey] ?? DEFAULT_AUDIO_RULES,
			noSupportedTrackCode: SVTA_NO_SUPPORTED_AUDIO_TRACK
		}
	})
});
/**
* Manage `selectedTextTrackId` as the single-writer **output** of standing user intent (`userTextTrackSelection`)
* resolved against the playable, CDN-scoped text renditions: clear on src unload; re-resolve when a CDN fails or
* recovers.
*
* Unlike video/audio, the selection is _optional_ — captions are opt-in and the user can turn them off — so the chain
* skips the bandwidth ranker and the shared user-selection filter, and supplies a text-specific terminal
* (`pickResolvedTextTrack`) that may resolve to no-selection via `setupTrackSwitching`'s `resolveSelection` seam.
* Constraints are failed-CDN only (`excludeUnplayableTracks`/`canPlayTrack` is MSE-based — the wrong probe for text,
* whose playability is SPF-parser support); the active-CDN scope co-locates captions with the surviving CDN on
* failover.
*
* @example
*   const reactor = switchTextTrack.setup({ state, config: { preferredSubtitleLanguage: 'en' } });
*/
const switchTextTrack = defineBehavior({
	stateKeys: ["presentation", "selectedTextTrackId"],
	contextKeys: [],
	setup: ({ state, config, ...otherProps }) => setupTrackSwitching({
		...otherProps,
		state,
		config: {
			...config,
			selectionKey: TEXT_TYPE_CONFIG.selectedKey,
			getTracks: (presentation) => getTracksByType(presentation, "text"),
			constraints: config?.[TEXT_TYPE_CONFIG.constraintsKey] ?? DEFAULT_TEXT_CONSTRAINTS,
			rules: config?.[TEXT_TYPE_CONFIG.rulesKey] ?? DEFAULT_TEXT_RULES,
			resolveSelection: pickResolvedTextTrack
		}
	})
});

//#endregion
//#region ../spf/dist/dev/media/buffer/covering-segment-start.js
/**
* The start of the segment whose media covers `currentTime`, when that's not the segment the playlist places there: the
* playlist's segment turned out to start late, so its predecessor's end holds the media. Reads the evidence straight
* from what is buffered — a range that starts inside the playhead's segment, after the playhead, is that segment's
* media starting late. `undefined` when the playlist's segment is the right one to load from (the playhead is buffered,
* nothing near it is, or it's in the first segment, which has no predecessor).
*/
function coveringSegmentStart(segments, bufferedRanges, currentTime) {
	if (bufferedRanges.some(({ start, end }) => start <= currentTime && currentTime < end)) return void 0;
	const index = segments.findIndex(({ startTime, duration }) => startTime <= currentTime && currentTime < startTime + duration);
	if (index <= 0) return void 0;
	const segment = segments[index];
	return bufferedRanges.some(({ start }) => start > currentTime && start < segment.startTime + segment.duration) ? segments[index - 1].startTime : void 0;
}

//#endregion
//#region ../spf/dist/dev/playback/primitives/load-start.js
/**
* Load-start seam for the `loadXSegments` dispatchers: a signal that moves a full-range load's start earlier than the
* playhead. The dispatcher re-sends its load whenever the value changes and starts the range there (`undefined`: at the
* playhead), so the loader's ordinary planning fetches whatever the earlier start reaches.
*/
/**
* Starts the load at the previous segment while the segment at the playhead turns out to begin after the playhead (its
* media starts a little after the time the playlist gives for it; see `media/buffer/covering-segment-start`). The end
* of the previous segment is the media the playhead needs.
*/
const coveringSegmentLoadStart = ({ state, context, config }) => computed(() => {
	const track = config.findResolvedTrack(state.presentation.get(), state[config.selectedKey]?.get());
	const bufferActor = config.actorKey && context[config.actorKey]?.get();
	if (!track || !bufferActor) return void 0;
	const { bufferedRanges } = bufferActor.snapshot.get().context;
	return coveringSegmentStart(track.segments, bufferedRanges, state.currentTime.get() ?? 0);
});

//#endregion
//#region ../spf/dist/dev/features/audio.js
/** The behaviors the feature composes, in setup order. */
const behaviors$6 = [
	resolveAudioTrack,
	switchAudioTrack,
	setupAudioBufferActors,
	loadAudioSegments,
	defineExternalSignals()({ state: ["userAudioTrackSelection"] })
];
/** The config defaults the feature contributes. */
const defaultConfig$6 = {
	canPlayTrack,
	reportUnsupportedTrackConditions,
	createLoadStart: coveringSegmentLoadStart
};
/** The state values the feature seeds. */
const initialState$6 = {};
/**
* Plays audio: resolves the selected rendition's playlist, switches renditions, and buffers and loads its segments.
* Honors the user's `userAudioTrackSelection`, such as a language.
*
* Requires, from this or other features:
*
* - A writer of `context.mediaSource`, such as `mediaSourceFeature`; nothing loads without one.
* - A writer of `state.currentTime`, such as `currentTimeFeature`; without one, loading stops after the first window.
* - A writer of `state.loadActivated`, such as `initialLoadFeature`, or a seeded `loadActivated: true`; without one, only
*   init segments load.
*/
const audioFeature = defineFeature({
	behaviors: behaviors$6,
	defaultConfig: defaultConfig$6,
	initialState: initialState$6
});

//#endregion
//#region ../spf/dist/dev/behaviors/dom/load-chapters.js
function deriveState$3(presentation, mediaElement) {
	if (!mediaElement || !isResolvedPresentation(presentation)) return "preconditions-unmet";
	return findChaptersDocument(presentation) === void 0 ? "preconditions-unmet" : "loading";
}
/** The URI of the chapters document — the first `com.apple.hls.chapters` entry that points at one. */
function findChaptersDocument(presentation) {
	return getSessionData(presentation, APPLE_HLS_CHAPTERS_DATA_ID).find((entry) => entry.uri !== void 0)?.uri;
}
function loadChaptersSetup({ state, context, config }) {
	const derivedStateSignal = computed(() => deriveState$3(state.presentation.get(), context.mediaElement.get()));
	const { requestCredentials } = config;
	return createMachineReactor({
		initial: "preconditions-unmet",
		monitor: () => derivedStateSignal.get(),
		states: {
			"preconditions-unmet": {},
			loading: { effects: () => {
				const mediaElement = context.mediaElement.get();
				const presentation = peek(state.presentation);
				const uri = presentation && findChaptersDocument(presentation);
				if (!mediaElement || !uri) return;
				const controller = new AbortController();
				loadChaptersTracks(mediaElement, uri, controller.signal, {
					preferredLanguage: config.preferredSubtitleLanguage,
					credentials: resolveRequestCredentials(requestCredentials, { url: uri })
				});
				return controller;
			} }
		}
	});
}
/**
* **Project Apple JSON chapters onto the host media element.** When the resolved presentation carries an
* `#EXT-X-SESSION-DATA` entry for `com.apple.hls.chapters`, fetch the document it points at, parse it, and add one
* hidden `<track kind="chapters">` per title language — cues and all — to `mediaElement`. The element holds the
* chapters: there is no chapters state signal, exactly as subtitle cues live on their DOM tracks and nowhere else.
* Consumers (video.js's textTrack feature, a host page) read the first chapters track's `cues` the way they would an
* authored `<track>`.
*
* Single-positive-state reactor (`'preconditions-unmet'` ↔ `'loading'`), gated on a media element, a resolved
* presentation, and a chapters entry with a URI. An element-bound effect fetches and projects; the returned cleanup
* aborts an in-flight fetch and removes the tracks on element replacement or state exit (source unload, destroy). The
* document leaves the last chapter open; it is written with `OPEN_CHAPTER_END` and never amended — a consumer that
* wants it to end where the media ends clamps to the media duration on read (video.js's textTrack feature does), so
* nothing here waits for, or chases, a duration.
*
* Failures are never fatal: a document that won't load or won't parse is warned about and projects nothing. An entry
* carrying its data inline as `VALUE` is skipped — chapters are a document, not a string. Exactly one document is
* assumed: Apple carries every language inside it, so the first entry with a URI is the one read.
*
* The tracks carry `data-src-chapters-track`, distinct from the subtitle tracks' tag, so neither cleanup removes the
* other's. A `<track kind="chapters">` the host page authored precedes these in `textTracks` (tree order), so a page
* that supplies its own chapters keeps them. On a Mux source this document is the same one the Mux adapter reads for
* the asset title; the request is cacheable, and each side stays ignorant of the other.
*/
const loadChapters = defineBehavior({
	stateKeys: ["presentation"],
	contextKeys: ["mediaElement"],
	setup: loadChaptersSetup
});

//#endregion
//#region ../spf/dist/dev/features/chapters.js
/** The behaviors the feature composes, in setup order. */
const behaviors$5 = [loadChapters];
/** The config defaults the feature contributes. */
const defaultConfig$5 = {};
/** The state values the feature seeds. */
const initialState$5 = {};
/**
* Adds the source's Apple JSON chapters (`EXT-X-SESSION-DATA`, `com.apple.hls.chapters`) to the media element as a
* hidden `chapters` text track per language.
*/
const chaptersFeature = defineFeature({
	behaviors: behaviors$5,
	defaultConfig: defaultConfig$5,
	initialState: initialState$5
});

//#endregion
//#region ../spf/dist/dev/behaviors/dom/recover-end-stall.js
/**
* Whether a `waiting` should be forced to `ended`: the MediaSource is `ended`, the stream is finite (not live),
* playback is active (not paused/seeking/already-ended), and the playhead sits within `nudgeWindow` of the reachable
* buffered end (so it's the true end, not a mid-stream buffer hole). Pure — the behavior supplies the live values.
*/
function shouldForceEnded(input, nudgeWindow) {
	const { msEnded, durationFinite, paused, seeking, ended, currentTime, bufferedEnd } = input;
	if (!msEnded || !durationFinite || paused || seeking || ended || bufferedEnd === void 0) return false;
	const gap = bufferedEnd - currentTime;
	return bufferedEnd > 0 && gap >= 0 && gap < nudgeWindow;
}
function recoverEndStallSetup({ context, config }) {
	const nudgeWindow = config?.endStallNudgeWindow ?? .2;
	return effect(() => {
		const mediaElement = context.mediaElement.get();
		if (!mediaElement) return;
		const onWaiting = () => {
			const mediaSource = context.mediaSource.get();
			if (shouldForceEnded({
				msEnded: mediaSource?.readyState === "ended",
				durationFinite: Number.isFinite(mediaElement.duration),
				paused: mediaElement.paused,
				seeking: mediaElement.seeking,
				ended: mediaElement.ended,
				currentTime: mediaElement.currentTime,
				bufferedEnd: mediaSource ? getMinBufferedEnd(mediaSource.sourceBuffers) : void 0
			}, nudgeWindow)) mediaElement.currentTime = mediaElement.duration;
		};
		return listen(mediaElement, "waiting", onWaiting);
	});
}
/**
* Recover the end-of-stream stall that Chrome exhibits on skewed A/V. After `endOfStream`, when the audio and video
* tracks end a few ms apart (e.g. a source with an A/V PTS skew), Chrome's audio-clock-paced playback freezes the
* playhead ~50–70ms short of the reachable buffered end and never fires `ended` — so playback hangs at the very end and
* loop never re-triggers. This behavior watches for the `waiting` event that fires at that freeze and, when the
* MediaSource is `ended` and the playhead sits at the reachable buffered end, nudges `currentTime` to `duration` to
* force the native `ended`.
*
* **Event-driven, no poll.** `waiting` fires at the instant the playhead stalls (measured ~0ms latency), so there's
* nothing to gain from polling — and polling would add its interval + a stall threshold before reacting.
*
* **Proximity to the _reachable_ buffered end** is the discriminator. That end is
* `getMinBufferedEnd(mediaSource.sourceBuffers)` — the `min` of the per-track (video/audio) SourceBuffer ends, i.e. the
* furthest point playback can reach — read from the SourceBuffers directly rather than the `mediaElement.buffered`
* aggregate. Once `endOfStream` is signalled that's the true content end. Requiring the playhead within
* `endStallNudgeWindow` of it distinguishes the real end-of-stream freeze from a mid-stream buffer-hole stall (which
* sits far from the buffered end), so we never skip content. The window must exceed the freeze gap; too small would
* miss the stall (a permanent hang), so the default is generous relative to the measured gap and is config-tunable for
* empirical tuning.
*
* Inert where it shouldn't act: live (the MediaSource never reaches `ended` while the window grows; `duration` is
* `Infinity`) and streams that end cleanly (no `waiting`).
*
* See `internal/decisions/spf/end-of-stream-av-skew-recovery.md`.
*/
const recoverEndStall = defineBehavior({
	stateKeys: [],
	contextKeys: ["mediaElement", "mediaSource"],
	setup: recoverEndStallSetup
});

//#endregion
//#region ../spf/dist/dev/features/end-stall-recovery.js
/** The behaviors the feature composes, in setup order. */
const behaviors$4 = [recoverEndStall];
/** The config defaults the feature contributes. */
const defaultConfig$4 = {};
/** The state values the feature seeds. */
const initialState$4 = {};
/**
* Fires the media element's `ended` when Chrome freezes the playhead a few frames short of the end of a stream whose
* audio and video end at slightly different times. Inert otherwise.
*/
const endStallRecoveryFeature = defineFeature({
	behaviors: behaviors$4,
	defaultConfig: defaultConfig$4,
	initialState: initialState$4
});

//#endregion
//#region ../spf/dist/dev/behaviors/dom/track-load-triggers.js
/**
* Signal-driven FSM. The signal `loadActivated` is the canonical externally- observable state; `deriveState` reads it
* (and the preconditions) to derive the local FSM state. The signal is _both_ the state and the data — no separate
* internal bookkeeping.
*
*     'preconditions-unmet' ⟷ 'monitoring' ⟷ 'load-active'
*
*     preconditions-unmet → monitoring        element + URL appear
*     monitoring          → load-active       signal flips true (listener fires
*                                             or external write)
*     load-active         → monitoring        within-state cleanup resets signal
*                                             (URL or element identity change)
*     any                 → preconditions-unmet  element or URL → undefined
*
*     any state → destroying → destroyed       on destroy()
*/
function deriveState$2(presentation, mediaElement, loadActivated) {
	if (!mediaElement || !presentation?.url) return "preconditions-unmet";
	if (loadActivated) return "load-active";
	return "monitoring";
}
/**
* Flips `loadActivated` to true the first time the user plays or seeks the current source, which lifts the preload gate
* on loading.
*
* Writes `state.loadActivated = true` the first time a `play` or `seeking` event fires on the attached media element
* for the current source — or immediately on entry if the element is already committed to loading (`el.autoplay`,
* `!el.paused`, or `el.seeking`), covering autoplay, native-controls, and direct-DOM-`play()` scenarios.
*
* Sticky-true _within a source identity_: subsequent play/pause/seek cycles don't flip back. Source identity =
* (mediaElement, presentation URL). Either changing — including direct in-place swap with no `undefined` intermediate —
* resets the signal to `false`.
*
* Multi-writer with `hls/adapter.ts:play()` (which writes `true` directly on programmatic play) is intentional —
* different domains. The adapter records programmatic intent; this behavior is the DOM-side observer. Pre-existing
* `true` writes are honored because `deriveState` reads the signal — a `true` value routes directly to `'load-active'`
* without entering `'monitoring'`.
*
* @example
*   const reactor = trackLoadTriggers.setup({ state, context });
*   // later:
*   reactor.destroy();
*/
function trackLoadTriggersSetup({ state, context }) {
	const derivedStateSignal = computed(() => deriveState$2(state.presentation.get(), context.mediaElement.get(), state.loadActivated.get()));
	const urlSignal = computed(() => state.presentation.get()?.url);
	return createMachineReactor({
		initial: "preconditions-unmet",
		monitor: () => derivedStateSignal.get(),
		states: {
			"preconditions-unmet": {},
			monitoring: { effects: () => {
				const el = context.mediaElement.get();
				const setLoadActivated = () => state.loadActivated.set(true);
				if (el.autoplay || !el.paused || el.seeking) {
					setLoadActivated();
					return;
				}
				const cleanupPlay = listen(el, "play", setLoadActivated);
				const cleanupSeeking = listen(el, "seeking", setLoadActivated);
				return () => {
					cleanupPlay();
					cleanupSeeking();
				};
			} },
			"load-active": { effects: () => {
				context.mediaElement.get();
				urlSignal.get();
				return () => state.loadActivated.set(false);
			} }
		}
	});
}
const trackLoadTriggers = defineBehavior({
	stateKeys: ["loadActivated", "presentation"],
	contextKeys: ["mediaElement"],
	setup: trackLoadTriggersSetup
});

//#endregion
//#region ../spf/dist/dev/behaviors/sync-preload.js
function syncPreloadSetup({ state, context, config }) {
	const defaultPreload = config?.defaultPreload ?? "metadata";
	const presentationUrl = computed(() => state.presentation.get()?.url);
	const cleanupRead = effect(() => {
		presentationUrl.get();
		const mediaElement = context.mediaElement.get();
		const current = peek(state.preload);
		if (current !== void 0 && !isStandardPreload(current)) return;
		const authoredPreload = mediaElement && (!mediaElement.hasAttribute || mediaElement.hasAttribute("preload")) ? mediaElement.preload : void 0;
		const target = isStandardPreload(authoredPreload) ? authoredPreload : current === void 0 ? defaultPreload : void 0;
		if (target === void 0 || target === current) return;
		state.preload.set(target);
	});
	const cleanupWrite = effect(() => {
		const next = state.preload.get();
		const mediaElement = context.mediaElement.get();
		if (!mediaElement) return;
		if (!isStandardPreload(next)) return;
		if (mediaElement.preload === next) return;
		mediaElement.preload = next;
	});
	return () => {
		cleanupRead();
		cleanupWrite();
	};
}
/**
* **Bidirectional sync between `state.preload` and `mediaElement.preload`.**
*
* Two effects:
*
* - **Read (DOM → state)** — on `context.mediaElement` swap or `state.presentation.url` change, copies the media
*   element's _authored_ preload into `state.preload` if it's a W3C value and `state.preload` isn't holding an extended
*   (non-W3C) value. Author intent only: on a DOM element the `preload` IDL property reports a browser-dependent UA
*   default even when no attribute was set (`'metadata'` Chromium, `'auto'` WebKit), so the property is consulted only
*   when the element carries a `preload` content attribute — a UA default is never a source. When the element has no
*   authored W3C opinion and `state.preload` is undefined, backfills from `config.defaultPreload` (default-default
*   `'metadata'`) so `state.preload` is never undefined in steady state.
* - **Write (state → DOM)** — on `state.preload` change or `context.mediaElement` swap, writes `state.preload` back to
*   `mediaElement.preload` if the value is W3C.
*
* Extended values (e.g. `'canplay'`) written externally to `state.preload` are sticky: read won't overwrite them, write
* won't push them to the DOM. All writes are deduped to break echo loops and avoid spurious re-triggers downstream
* (notably `resolvePresentation`, which reads `state.preload`).
*/
const syncPreload = defineBehavior({
	stateKeys: ["preload", "presentation"],
	contextKeys: ["mediaElement"],
	setup: syncPreloadSetup
});

//#endregion
//#region ../spf/dist/dev/features/initial-load.js
/** The behaviors the feature composes, in setup order. */
const behaviors$3 = [syncPreload, trackLoadTriggers];
/** The config defaults the feature contributes. */
const defaultConfig$3 = {};
/** The state values the feature seeds. */
const initialState$3 = {};
/**
* Defers loading until the media element's `preload` allows it or the user plays or seeks, mirroring native
* `HTMLMediaElement` preload behavior. Syncs `preload` between the element and state, and activates loading on the
* first `play` or `seeking` for each source.
*
* Without it, a composition loads the moment a source is set; seed `loadActivated: true` in its initial state, as the
* background-video engine does. The two behaviors compose together: syncing `preload` without the load triggers would
* leave `preload="none"` and `"metadata"` sources unable to load on play.
*/
const initialLoadFeature = defineFeature({
	behaviors: behaviors$3,
	defaultConfig: defaultConfig$3,
	initialState: initialState$3
});

//#endregion
//#region ../spf/dist/dev/core/tasks/delayed-reschedule.js
/**
* Build a {@link Reschedule} from a pure cadence function — the common timer-based, _start-anchored_ implementation.
*
* Invoked concurrently with the run, it observes the result, then waits `cadence(current, previous)` milliseconds
* **measured from when it was invoked** (≈ the run's start): it subtracts the run's own elapsed time, so consecutive
* runs begin one cadence apart regardless of how long each run takes (per RFC 8216 §6.3.4's "measured from the last
* time the client began loading"). If the run takes longer than the cadence, the next run starts immediately.
*
* A `null` cadence stops the recurrence. A rejected run rejects this reschedule, which the `RecurringRunner` propagates
* as the recurrence's failure — error recovery (e.g. retrying transient fetch failures) belongs below, at the fetch
* layer, not in the cadence.
*/
function delayedReschedule(cadence) {
	return async (task) => {
		const startedAt = Date.now();
		const ms = cadence(await task.run(), task.previous);
		if (ms === null) return false;
		await sleep(Math.max(0, ms - (Date.now() - startedAt)), task.signal);
		return true;
	};
}

//#endregion
//#region ../spf/dist/dev/behaviors/dom/seek-to-live-edge.js
/**
* Tolerance (seconds) around the window edges before the guard repositions, so boundary / floating-point noise doesn't
* trigger a spurious seek.
*/
const REPOSITION_TOLERANCE = .1;
/**
* `'live'` once the preconditions hold: a media element and a derivable live edge (whose placement is final by
* construction — see `seekToLiveEdge`'s JSDoc). `'inactive'` otherwise.
*
* Deliberately narrow: every signal here can flip the reactor out of and back into `live`, re-firing `entry`. Neither
* blinks mid-source — the edge can't, because `liveWindowForType` falls back to any resolved track of the type — so
* `entry` fires once per source without a latch, and a live reload (same source, slid window) correctly doesn't
* re-command.
*
* The flip side: the presentation _url_ is not part of this state, so replacing one already-resolved live presentation
* with another would stay in `live` and never command a start position for the new source. Unreachable today — every
* writer sets `{ url }` (unresolved) or `undefined` first, so a source change always transits `inactive`. Supporting a
* seeded pre-resolved presentation (via `initialState`) that later changes would mean folding the url in here.
*/
function deriveState$1(mediaElement, edge) {
	return mediaElement && edge ? "live" : "inactive";
}
function seekToLiveEdgeSetup({ state, context, config }) {
	const derivedStateSignal = computed(() => deriveState$1(context.mediaElement.get(), getLiveEdge({
		state,
		config
	})));
	return createMachineReactor({
		initial: "inactive",
		monitor: () => derivedStateSignal.get(),
		states: {
			inactive: {},
			live: {
				entry: () => {
					const mediaElement = context.mediaElement.get();
					const { liveEdgeStart } = getLiveEdge({
						state,
						config
					});
					if (mediaElement.currentTime >= liveEdgeStart) return;
					state.startPosition.set(liveEdgeStart);
				},
				effects: () => {
					const { start: windowStart, liveEdgeStart } = getLiveEdge({
						state,
						config
					});
					const mediaElement = peek(context.mediaElement);
					const reposition = () => {
						if (mediaElement.paused) return;
						if (mediaElement.currentTime < windowStart - REPOSITION_TOLERANCE) mediaElement.currentTime = liveEdgeStart;
					};
					reposition();
					return listen(mediaElement, "play", reposition);
				}
			}
		}
	});
}
/**
* Keeps the playhead inside the live window: starts near the live edge and snaps back to it when playback falls behind
* the window.
*
* A two-state reactor gated on the preconditions for "we know where live is":
*
* - **`inactive`** — no media element, or no live edge (`getLiveEdge` is `null`: VOD, ended, or unresolved). Idle.
* - **`live`** — preconditions met. `entry` commands `state.startPosition` once to the target live latency behind the
*   edge (clamped to the window start) so playback begins near the edge and the loader dispatches an in-window range;
*   `effects` runs the window-exit guard.
*
* A derivable live edge is itself the establishment gate: segment placement is settled at parse time — the reference
* track's local placement _is_ the presentation timeline, and every other track's first parse is held until the anchor
* is stamped (`resolve-track`'s gate + `establishStartMediaTime`) — so any window derived from resolved segments is
* already final, with no separate anchor signal to wait on (see
* `internal/design/spf/live-presentation-timeline-model.md`).
*
* The two pieces split along the axis a future DVR / EVENT mode will care about: the **one-time start position**
* (`entry`) is the _live-specific_ behavior — start near the edge on load; a DVR mode makes it conditional (start in
* place). The **window-exit guard** (`effects`) is the _general windowed-live_ behavior — applies to sliding-window
* live, DVR, and EVENT alike. Because the command is an `entry`, it fires once per entry into `live`; a source change
* exits to `inactive`, so the next source re-commands (no closure latch to reset). The guard stays a direct seek — it
* is recurring, while `startPosition` is a self-clearing one-shot.
*
* Window-exit guard: while playing (not paused), reposition to the live edge when the playhead has fallen behind the
* window start — including when a seek to a now-evicted position has stranded the playhead (such a seek can never
* settle, so we rescue rather than wait on it). Two triggers: the **window-update re-fire** (the guard reads the live
* edge, so each reload / slide re-runs it — this catches a stall, where `timeupdate` stops but the playlist keeps
* reloading) and a **`play` listener** for immediate reactivity on resume, since the reload interval can be seconds.
* `play`, not `playing`: after a long pause the playhead sits behind the window at an unseekable position, where the
* browser stalls and `playing` never fires; `play` fires on the paused→false transition regardless, so we snap before
* the stall. In-window pause / DVR scrub-back are left untouched.
*
* The latency comes from the injected `resolveLiveLatency` seam (HLS: `HOLD-BACK`), so this behavior carries no
* delivery-format specifics. `applyStartPosition` performs the seek, gated on `loadedmetadata` — which implies an open
* MediaSource and hence a declared seekable range (a seek outside `seekable` is clamped) — so this behavior needs no
* MediaSource precondition of its own.
*
* Manual `Behavior<>` literal (like `calculatePresentationDuration`): declares only `presentation` + `startPosition` in
* stateKeys while reading `selectedVideoTrackId` / `selectedAudioTrackId` defensively (contributed by the switch*
* behaviors), so it composes without a stateKeys/type conflict.
*/
const seekToLiveEdge = {
	stateKeys: ["presentation", "startPosition"],
	contextKeys: ["mediaElement"],
	setup: seekToLiveEdgeSetup
};

//#endregion
//#region ../spf/dist/dev/behaviors/dom/sync-live-seekable-range.js
function syncLiveSeekableRangeSetup({ state, context }) {
	return effect(() => {
		const mediaSource = context.mediaSource.get();
		const liveWindow = liveWindowFromState(state);
		if (!mediaSource || !liveWindow) return;
		const start = Math.max(0, liveWindow.start);
		if (start >= liveWindow.end) return;
		mediaSource.setLiveSeekableRange(start, liveWindow.end);
	});
}
/**
* Mirror the live window into the MediaSource's seekable range. On every window update — **including while paused**
* (the seekable range must stay current as the window slides, regardless of play state) — declare
* `setLiveSeekableRange(start, end)` so the browser's `HTMLMediaElement.seekable` reflects the live window (without it,
* `seekable` is empty under `duration === Infinity`).
*
* The live window comes from `liveWindowFromState` (the shared derivation — the intersection over the selected A/V
* tracks' windows); inert when it returns `null` (VoD / ended live). Clamp the start to presentation-0 and skip windows
* ending at or before it. Composed _before_ `seekToLiveEdge` so the range is declared before that behavior seeks the
* playhead into it (a seek outside `seekable` is clamped).
*
* Duration is owned solely by `updateMediaSourceDuration`; this behavior only declares the seekable range
* (`setLiveSeekableRange` requires only `readyState === 'open'` per the W3C MSE spec, not a set `duration`).
*
* No `clearLiveSeekableRange()` on termination, by design: the MSE spec consults the live seekable range _only_ while
* `duration === Infinity`. When a live stream ends, `endOfStream()` sets a finite duration and the UA derives
* `seekable` from buffered + duration, ignoring the live range — so clearing is unnecessary. Clearing on the
* `ENDLIST`→finite-`Track.duration` transition (when the window goes `null`) would also be premature: `duration` is
* still `Infinity` until `endOfStream`, so clearing would shrink `seekable` to buffered-only while the stream is still
* effectively live.
*
* Manual `Behavior<>` literal (like `seekToLiveEdge`): declares only `presentation` in stateKeys while reading
* `selectedVideoTrackId` / `selectedAudioTrackId` defensively (contributed by the switch* behaviors), so it composes
* without a stateKeys/type conflict.
*/
const syncLiveSeekableRange = {
	stateKeys: ["presentation"],
	contextKeys: ["mediaSource"],
	setup: syncLiveSeekableRangeSetup
};

//#endregion
//#region ../spf/dist/dev/features/live.js
/** The behaviors the feature composes, in setup order. */
const behaviors$2 = [
	syncLiveSeekableRange,
	seekToLiveEdge,
	establishStartMediaTime
];
/** The config defaults the feature contributes. */
const defaultConfig$2 = {
	resolveLiveLatency,
	reschedule: delayedReschedule(mediaPlaylistReloadDelay),
	gateFirstParse: gateFirstParseOnAnchor
};
/** The state values the feature seeds. */
const initialState$2 = {};
/**
* Plays live streams: reloads their playlists, keeps the seekable window current, starts at the live edge, and keeps
* the playhead inside the window. Aligns separately delivered tracks on the stream's program date-times when joining
* mid-stream. Inert for on-demand content.
*
* Requires:
*
* - A consumer of `state.startPosition`, such as `startPositionFeature`; without one, playback starts at the window's
*   start instead of the live edge.
* - A writer of `presentation.duration` and of the MediaSource's duration, such as `calculateDurationFeature` with
*   `mediaSourceFeature`; without them, the stream never gets its infinite duration and stalls.
*/
const liveFeature = defineFeature({
	behaviors: behaviors$2,
	defaultConfig: defaultConfig$2,
	initialState: initialState$2
});

//#endregion
//#region ../spf/dist/dev/behaviors/derive-cdn-priority.js
const samePriority = (a, b) => !!a && a.length === b.length && a.every((cdn, i) => cdn === b[i]);
/**
* **Publishes which CDNs a source is served from, most-preferred first.** While a presentation is resolved, owns the
* `cdnPriority` signal: the distinct CDNs the source is served from (origin of each track's URL), in manifest priority
* order — most-preferred first. Cleared on src unload. The name mirrors HLS content steering's `PATHWAY-PRIORITY`.
*
* Redundant-stream sources (e.g. Mux's `?redundant_streams=true`) list the same content on multiple hosts, so the
* candidate tracks already include one variant per CDN. This behavior publishes _which_ CDNs exist and their priority;
* the `preferActiveCdn` scope rule in `track-switching` reads `cdnPriority` and narrows each type's candidates to the
* first CDN with surviving tracks — so video / audio / text all resolve from one host (the shared list is the
* per-presentation coherence guarantee).
*
* The "active" CDN is not stored — it's derived by the scope as the highest-priority entry in `cdnPriority` that still
* has tracks after the constraints pre-pass. That makes failover a pure consequence of the (future) failed-CDN
* constraint: when the primary's tracks are pruned during cooldown, the scope falls to the next CDN; when the primary
* recovers, it snaps back. Content steering, when it lands, reorders `cdnPriority` (pathway priority as a sort key).
*
* Lifecycle: `'presentation-unresolved'` ↔ `'presentation-resolved'`, mirroring `setupTrackSwitching`. The resolved
* state owns the signal; its entry-returned cleanup clears it on exit (canonical cleanup-binds-to-setup per
* `reactors.md`).
*
* @example
*   const reactor = deriveCdnPriority.setup({ state });
*/
const deriveCdnPriority = defineBehavior({
	stateKeys: ["presentation", "cdnPriority"],
	contextKeys: [],
	setup: ({ state, config = {} }) => {
		const getCdnId$1 = config.getCdnId ?? getCdnId;
		const derivedStateSignal = computed(() => isResolvedPresentation(state.presentation.get()) ? "presentation-resolved" : "presentation-unresolved");
		return createMachineReactor({
			initial: "presentation-unresolved",
			monitor: () => derivedStateSignal.get(),
			states: {
				"presentation-unresolved": {},
				"presentation-resolved": {
					entry: () => () => state.cdnPriority.set(void 0),
					effects: [() => {
						const presentation = state.presentation.get();
						if (!isResolvedPresentation(presentation)) return;
						const next = getOrderedCdnIds(presentation, getCdnId$1);
						if (!samePriority(peek(state.cdnPriority), next)) state.cdnPriority.set(next);
					}]
				}
			}
		});
	}
});

//#endregion
//#region ../spf/dist/dev/behaviors/setup-failover-monitor.js
const DEFAULT_FAILOVER_MONITOR_CONFIG = { cooldownMs: 3e5 };
/**
* **Returns a failed CDN to the candidates once its cooldown lapses.** The expiry half of multi-CDN failover. Fetch
* sites own the _trip_: on a failed fetch they add the failing CDN (origin) to the `failedCdns` state signal directly.
* This behavior owns the _expiry_: while a presentation is resolved, it watches `failedCdns` and, for each CDN that
* appears, schedules a timer to remove it once its cooldown lapses. `track-switching`'s `excludeFailedCdns` constraint
* prunes a failed CDN's tracks and the active-CDN scope falls to the next one — and back, once the cooldown removes it
* here.
*
* Lifecycle is per-source: timers + `failedCdns` are cleared on exit (a new source starts with a clean slate). Policy
* (cooldown) is engine config. This is the minimal `network-resilience` slice — a single failure trips a CDN, since
* transient blips are the retry layer's job (it sits below the fetch sites, so anything that reaches `failedCdns` is
* already terminal).
*
* @example
*   const reactor = setupFailoverMonitor.setup({ state });
*/
const setupFailoverMonitor = defineBehavior({
	stateKeys: ["presentation", "failedCdns"],
	contextKeys: [],
	setup: ({ state, config = {} }) => {
		const cooldownMs = config.failover?.cooldownMs ?? DEFAULT_FAILOVER_MONITOR_CONFIG.cooldownMs;
		const timers = /* @__PURE__ */ new Map();
		const derivedStateSignal = computed(() => isResolvedPresentation(state.presentation.get()) ? "presentation-resolved" : "presentation-unresolved");
		return createMachineReactor({
			initial: "presentation-unresolved",
			monitor: () => derivedStateSignal.get(),
			states: {
				"presentation-unresolved": {},
				"presentation-resolved": {
					entry: () => () => {
						timers.forEach((timer) => clearTimeout(timer));
						timers.clear();
						state.failedCdns.set(void 0);
					},
					effects: [() => {
						(state.failedCdns.get() ?? []).forEach((cdn) => {
							if (timers.has(cdn)) return;
							const timer = setTimeout(() => {
								timers.delete(cdn);
								update(state.failedCdns, (current) => current?.filter((c) => c !== cdn));
							}, cooldownMs);
							timers.set(cdn, timer);
						});
					}]
				}
			}
		});
	}
});

//#endregion
//#region ../spf/dist/dev/features/multi-cdn.js
/** The behaviors the feature composes, in setup order. */
const behaviors$1 = [deriveCdnPriority, setupFailoverMonitor];
/** The config defaults the feature contributes. */
const defaultConfig$1 = {};
/** The state values the feature seeds. */
const initialState$1 = {};
/**
* Plays sources served from several CDNs (redundant streams): keeps every track type on one CDN, and fails over to the
* next when a CDN fails, returning to it once its cooldown lapses. Single-CDN sources pass through unchanged.
*/
const multiCdnFeature = defineFeature({
	behaviors: behaviors$1,
	defaultConfig: defaultConfig$1,
	initialState: initialState$1
});

//#endregion
//#region ../spf/dist/dev/behaviors/dom/apply-start-position.js
function deriveState(presentation, mediaElement, startPosition) {
	if (!mediaElement || !isResolvedPresentation(presentation)) return "preconditions-unmet";
	if (isUndefined(startPosition)) return "preconditions-unmet";
	return "position-pending";
}
function applyStartPositionSetup({ state, context }) {
	const derivedStateSignal = computed(() => deriveState(state.presentation.get(), context.mediaElement.get(), state.startPosition.get()));
	return createMachineReactor({
		initial: "preconditions-unmet",
		monitor: () => derivedStateSignal.get(),
		states: {
			"preconditions-unmet": {},
			"position-pending": { entry: () => {
				const mediaElement = context.mediaElement.get();
				const position = state.startPosition.get();
				state.currentTime.set(position);
				const apply = () => {
					mediaElement.currentTime = position;
					state.startPosition.set(void 0);
				};
				if (mediaElement.readyState >= HTMLMediaElement.HAVE_METADATA) {
					apply();
					return;
				}
				return listen(mediaElement, "loadedmetadata", apply, { once: true });
			} }
		}
	});
}
/**
* **Start playback of a source at a requested position.** `state.startPosition` is a one-shot command — "when the
* current source can seek, start there" — the SPF analogue of hls.js's `startPosition` (and the primitive
* `EXT-X-START`, resume-where-you-left-off, and MediaSource-recovery restore build on). Consumers (adapters,
* `setupAirPlay`'s session-end snapshot) write it; this behavior is its sole consumer and clears it after applying, so
* a stale position can never replay against a later source or rebuild.
*
* Single-positive-state reactor (`'preconditions-unmet'` ↔ `'position-pending'`): gated on `mediaElement + resolved
* presentation + startPosition` defined. The entry applies the command in two steps:
*
* 1. **Seed `state.currentTime` immediately.** The segment loaders anchor their load window on `state.currentTime`;
*    seeding points the _first_ fetches at the requested position instead of 0. Multi-writer with `trackCurrentTime`
*    (ongoing DOM mirror) — legitimate: different decision domains (element-derived mirror vs one-shot command), and
*    before HAVE_METADATA no `timeupdate`/`seeking` fires to overwrite the seed.
* 2. **Seek the element at metadata.** `element.currentTime = position` once `readyState >= HAVE_METADATA` (immediately if
*    already there, else on `loadedmetadata`), then clear `startPosition` (consume). The element clamps the seek to its
*    seekable range per spec, and the resulting `seeking` event flows back through `trackCurrentTime` — from here the
*    ordinary seek path owns the position.
*
* Position only — playing/paused is deliberately out of scope. The media element load algorithm forces `paused = true`,
* so a source that was playing before a rebuild comes back paused at the restored position; resume intent belongs to
* whoever commands the start (e.g. `setupAirPlay` restores its session-end playing state itself).
*
* Deliberately NOT relying on the pre-metadata "default playback start position" write (setting `currentTime` at
* HAVE_NOTHING): cross-browser MSE behavior there is inconsistent; the explicit `loadedmetadata` sequencing is
* deterministic everywhere.
*
* State-exit cleanup (source reset, element detach, destroy) drops the pending `loadedmetadata` listener. An
* _unapplied_ command survives a source reset — "start the source I'm loading at P" holds while the presentation routes
* through unresolved — but is consumed the moment it applies.
*/
const applyStartPosition = defineBehavior({
	stateKeys: [
		"presentation",
		"startPosition",
		"currentTime"
	],
	contextKeys: ["mediaElement"],
	setup: applyStartPositionSetup
});

//#endregion
//#region ../spf/dist/dev/features/start-position.js
/** The behaviors the feature composes, in setup order. */
const behaviors = [applyStartPosition];
/** The config defaults the feature contributes. */
const defaultConfig = {};
/** The state values the feature seeds. */
const initialState = {};
/**
* Starts playback at `state.startPosition` instead of zero. Also performs the seeks other features request through
* `state.startPosition`, such as the live edge and the AirPlay session-end restore.
*/
const startPositionFeature = defineFeature({
	behaviors,
	defaultConfig,
	initialState
});

//#endregion
//#region ../spf/dist/dev/network/request-credentials.js
/**
* Map a media element's CORS mode (its `crossOrigin` as reflected — see `toMediaCrossOrigin`) to the `credentials` mode
* the engine's own requests should carry.
*
* Only `use-credentials` asks for anything: it maps to `'include'`, the mode the browser itself fetches the element's
* resources with, so a cookie-gated cross-origin stream works the same whether the browser or the engine loads it.
* `anonymous` and no attribute yield `undefined`, leaving the platform default (`same-origin`).
*
* The attribute-absent case deliberately does not map to `'include'` even though the browser's own no-CORS media
* fetches send cookies: `fetch` cannot use no-CORS here (bodies must be readable), and a credentialed CORS request is
* refused by any server answering `Access-Control-Allow-Origin: *`, which would break every stream that works today.
*/
function crossOriginToRequestCredentials(crossOrigin) {
	return crossOrigin === "use-credentials" ? "include" : void 0;
}

//#endregion
export { liveTrackId as _, initialLoadFeature as a, audioFeature as c, DEFAULT_VIDEO_CONSTRAINTS as d, switchTextTrack as f, getLiveEdge as g, defineExternalSignals as h, liveFeature as i, coveringSegmentLoadStart as l, airPlayFeature as m, startPositionFeature as n, endStallRecoveryFeature as o, switchVideoTrack as p, multiCdnFeature as r, chaptersFeature as s, crossOriginToRequestCredentials as t, DEFAULT_AUDIO_CONSTRAINTS as u, deriveTargetLiveWindow as v, resolveLiveLatency as y };
//# sourceMappingURL=request-credentials-DILzL07q.js.map