import { H as transmuxedSourceBufferMimeCodec, Ot as defineFeature, T as reportUnsupportedTransmuxedTrackConditions, V as canPlayTransmuxedTrack, v as parseMultivariantPlaylist } from "./error-surface-BHz7DxGx.js";
import { a as u16, i as box, l as resolveCarriedAudioRenditions, n as aacCodec, o as u32, r as concatPes, s as zeros, t as transmuxPipelinesFor } from "./transmux-pipelines-DcYjJ_gT.js";

//#region ../spf/dist/dev/media/mpeg-ts/h264.js
/**
* H.264 Annex B elementary-stream parsing for the TS transmux path: splits each PES payload into NAL units, groups them
* into access units (one `VideoSample` per picture, keyed on the access-unit delimiter the packager emits), and reads
* the SPS for the `avcC` / `avc1` sample entry (profile, level, coded size).
*
* Targets spec-valid HLS content: AUD-delimited access units, in-band SPS/PPS, IDR at segment start.
*/
const NAL_IDR = 5;
const NAL_SPS = 7;
const NAL_PPS = 8;
const NAL_AUD = 9;
/** `[start, end)` byte ranges of the NAL units in an Annex B stream (start codes excluded). */
function findNalUnits(data) {
	const ranges = [];
	let unitStart = -1;
	const close = (end) => {
		while (end > unitStart && data[end - 1] === 0) end--;
		ranges.push([unitStart, end]);
	};
	for (let i = 0; i + 2 < data.length;) {
		if (data[i] !== 0 || data[i + 1] !== 0 || data[i + 2] !== 1) {
			i++;
			continue;
		}
		if (unitStart >= 0) close(i);
		i += 3;
		unitStart = i;
	}
	if (unitStart >= 0 && unitStart < data.length) close(data.length);
	return ranges;
}
/**
* Group a stream's PES payloads into access units. The payloads are walked as one continuous Annex B stream — PES
* boundaries need not fall on picture boundaries — and a new sample opens on each access-unit delimiter, timed by the
* PES in which that delimiter begins. Parameter sets are copied out for the config and also kept in-band in the sample,
* as decoders accept.
*/
function parseAvcAccessUnits(packets) {
	const { data, boundaries } = concatPes(packets);
	const samples = [];
	let current;
	let sps;
	let pps;
	let pesIndex = 0;
	/** The PES whose payload holds `position` — boundaries ascend, as do NAL positions, so this only moves forward. */
	const pesAt = (position) => {
		while (pesIndex + 1 < boundaries.length && boundaries[pesIndex + 1].offset <= position) pesIndex++;
		return boundaries[pesIndex];
	};
	const close = () => {
		if (current?.units.length) samples.push(current);
		current = void 0;
	};
	for (const [start, end] of findNalUnits(data)) {
		const unit = data.subarray(start, end);
		const type = unit[0] & 31;
		if (type === NAL_AUD) close();
		if (type === NAL_SPS) sps = unit;
		if (type === NAL_PPS) pps = unit;
		if (current === void 0) {
			const { pts, dts } = pesAt(start);
			const previous = samples[samples.length - 1];
			current = {
				pts: pts ?? previous?.pts ?? 0,
				dts: dts ?? previous?.dts ?? 0,
				key: false,
				units: [],
				size: 0
			};
		}
		if (type === NAL_IDR) current.key = true;
		current.units.push(unit);
		current.size += 4 + unit.length;
	}
	close();
	if (!sps || !pps) return { samples };
	const { width, height } = parseSps(sps);
	const codec = `avc1.${[
		sps[1],
		sps[2],
		sps[3]
	].map((b) => b.toString(16).padStart(2, "0")).join("")}`;
	return {
		samples,
		config: {
			sps,
			pps,
			width,
			height,
			codec
		}
	};
}
/** Strip emulation-prevention bytes (`00 00 03` → `00 00`) to recover the RBSP. */
function toRbsp(nal) {
	const out = [];
	for (let i = 0, zeros = 0; i < nal.length; i++) {
		const byte = nal[i];
		if (zeros === 2 && byte === 3) {
			zeros = 0;
			continue;
		}
		out.push(byte);
		zeros = byte === 0 ? zeros + 1 : 0;
	}
	return Uint8Array.from(out);
}
/** MSB-first bit reader with the Exp-Golomb codes SPS parsing needs. */
var BitReader = class {
	data;
	position = 0;
	constructor(data) {
		this.data = data;
	}
	readBits(count) {
		let value = 0;
		for (let i = 0; i < count; i++, this.position++) value = value * 2 + (this.data[this.position >> 3] >> 7 - (this.position & 7) & 1);
		return value;
	}
	readUe() {
		let zeros = 0;
		while (this.readBits(1) === 0 && zeros < 32) zeros++;
		return zeros === 0 ? 0 : 2 ** zeros - 1 + this.readBits(zeros);
	}
	readSe() {
		const value = this.readUe();
		return value & 1 ? (value + 1) / 2 : -value / 2;
	}
};
const HIGH_PROFILES = /* @__PURE__ */ new Set([
	100,
	110,
	122,
	244,
	44,
	83,
	86,
	118,
	128,
	138,
	139,
	134,
	135
]);
function skipScalingList(reader, size) {
	let lastScale = 8;
	let nextScale = 8;
	for (let i = 0; i < size; i++) {
		if (nextScale !== 0) nextScale = (lastScale + reader.readSe() + 256) % 256;
		lastScale = nextScale === 0 ? lastScale : nextScale;
	}
}
/** Coded picture size from a sequence parameter set (ITU-T H.264 §7.3.2.1.1), 4:2:0 crop units. */
function parseSps(nal) {
	const reader = new BitReader(toRbsp(nal.subarray(1)));
	const profileIdc = reader.readBits(8);
	reader.readBits(16);
	reader.readUe();
	let chromaFormatIdc = 1;
	if (HIGH_PROFILES.has(profileIdc)) {
		chromaFormatIdc = reader.readUe();
		if (chromaFormatIdc === 3) reader.readBits(1);
		reader.readUe();
		reader.readUe();
		reader.readBits(1);
		if (reader.readBits(1)) {
			for (let i = 0; i < (chromaFormatIdc !== 3 ? 8 : 12); i++) if (reader.readBits(1)) skipScalingList(reader, i < 6 ? 16 : 64);
		}
	}
	reader.readUe();
	const picOrderCntType = reader.readUe();
	if (picOrderCntType === 0) reader.readUe();
	else if (picOrderCntType === 1) {
		reader.readBits(1);
		reader.readSe();
		reader.readSe();
		const cycleLength = reader.readUe();
		for (let i = 0; i < cycleLength; i++) reader.readSe();
	}
	reader.readUe();
	reader.readBits(1);
	const picWidthInMbs = reader.readUe() + 1;
	const picHeightInMapUnits = reader.readUe() + 1;
	const frameMbsOnly = reader.readBits(1);
	if (!frameMbsOnly) reader.readBits(1);
	reader.readBits(1);
	let cropLeft = 0;
	let cropRight = 0;
	let cropTop = 0;
	let cropBottom = 0;
	if (reader.readBits(1)) {
		cropLeft = reader.readUe();
		cropRight = reader.readUe();
		cropTop = reader.readUe();
		cropBottom = reader.readUe();
	}
	return {
		width: picWidthInMbs * 16 - (cropLeft + cropRight) * 2,
		height: (2 - frameMbsOnly) * picHeightInMapUnits * 16 - (frameMbsOnly ? 2 : 4) * (cropTop + cropBottom)
	};
}

//#endregion
//#region ../spf/dist/dev/media/mpeg-ts/codecs/avc.js
const MPEG_TS_TIMESCALE = 9e4;
function avc1(sps, pps, width, height) {
	const avcC = box("avcC", Uint8Array.from([
		1,
		sps[1],
		sps[2],
		sps[3],
		255,
		225
	]), u16(sps.length), sps, Uint8Array.from([1]), u16(pps.length), pps);
	return box("avc1", zeros(6), u16(1), zeros(16), u16(width), u16(height), u32(4718592), u32(4718592), zeros(4), u16(1), zeros(32), u16(24), u16(65535), avcC);
}
/**
* A fragment's timing, carried as the stream recorded it: `tfdt` is the first sample's decode time and each composition
* offset is that sample's PTS − DTS. Relocation reads `tfdt` as a decode time (`media/mp4/timestamp-origin`), so the
* fragment lands on the playlist's timeline exactly as a CMAF segment of the same stream would. With B-frames the first
* frame presents a frame or two after the segment's start. That delay is real media timing, covered by the previous
* segment's end rather than hidden by moving the timeline. Durations are DTS deltas; the last sample reuses the one
* before it.
*/
function toFragmentTiming(samples) {
	return {
		baseMediaDecodeTime: samples[0].dts,
		samples: samples.map((sample, i) => {
			const next = samples[i + 1];
			const previous = samples[i - 1];
			return {
				duration: next ? next.dts - sample.dts : previous ? sample.dts - previous.dts : MPEG_TS_TIMESCALE / 30,
				size: sample.size,
				key: sample.key,
				cts: sample.pts - sample.dts
			};
		})
	};
}
/** Length-prefixed NAL units, as `avc1` samples carry them. */
function toPayload(samples) {
	const payload = new Uint8Array(samples.reduce((n, s) => n + s.size, 0));
	const view = new DataView(payload.buffer);
	let offset = 0;
	for (const sample of samples) for (const unit of sample.units) {
		view.setUint32(offset, unit.length);
		payload.set(unit, offset + 4);
		offset += 4 + unit.length;
	}
	return payload;
}
const avcCodec = {
	streamType: 27,
	family: "avc1",
	frame(packets, { resolution }) {
		const { samples, config } = parseAvcAccessUnits(packets);
		if (!config || samples.length === 0) return void 0;
		const { width, height } = resolution ?? parseSps(config.sps);
		return {
			track: {
				id: 1,
				handler: "vide",
				timescale: MPEG_TS_TIMESCALE,
				width,
				height,
				sampleEntry: avc1(config.sps, config.pps, width, height)
			},
			...toFragmentTiming(samples),
			payload: toPayload(samples)
		};
	}
};

//#endregion
//#region ../spf/dist/dev/playback/primitives/segment-share.js
/** `keep` bounds the untaken entries held after their fetch completes (each is a whole segment). */
function createSegmentShare({ keep = 4 } = {}) {
	const entries = /* @__PURE__ */ new Map();
	const history = /* @__PURE__ */ new Set();
	const waiters = /* @__PURE__ */ new Map();
	const remember = (key) => {
		history.add(key);
		if (history.size > 1e3) history.delete(history.keys().next().value);
	};
	const withdraw = (key, bytes) => {
		if (entries.get(key) === bytes) entries.delete(key);
	};
	/** Resolves `true` when the key is published within `waitMs`, `false` when the wait runs out. */
	const awaitPublish = (key, waitMs) => new Promise((resolve) => {
		const set = waiters.get(key) ?? /* @__PURE__ */ new Set();
		const timer = setTimeout(() => {
			set.delete(settle);
			resolve(false);
		}, waitMs);
		const settle = () => {
			clearTimeout(timer);
			resolve(true);
		};
		set.add(settle);
		waiters.set(key, set);
	});
	return {
		publish(key, bytes) {
			entries.set(key, bytes);
			remember(key);
			bytes.catch(() => withdraw(key, bytes));
			if (entries.size > keep) entries.delete(entries.keys().next().value);
			const pending = waiters.get(key);
			waiters.delete(key);
			pending?.forEach((settle) => settle());
		},
		async take(key, waitMs) {
			let entry = entries.get(key);
			if (!entry && !history.has(key) && await awaitPublish(key, waitMs)) entry = entries.get(key);
			if (!entry) return void 0;
			try {
				return await entry;
			} catch {
				return;
			} finally {
				withdraw(key, entry);
			}
		},
		published: (key) => history.has(key)
	};
}

//#endregion
//#region ../spf/dist/dev/features/mpeg-ts-transmux.js
/** The feature composes no behaviors of its own: it reconfigures the track features' loaders and probes. */
const behaviors = [];
/** The HLS parser, with audio renditions carried in the video segments resolved to their carrier variants. */
const parsePresentation = (text, presentation) => resolveCarriedAudioRenditions(parseMultivariantPlaylist(text, presentation));
const share = createSegmentShare();
/** The config defaults the feature contributes. */
const defaultConfig = {
	parsePresentation,
	videoMessagePipelines: transmuxPipelinesFor("video", [avcCodec, aacCodec], { share }),
	audioMessagePipelines: transmuxPipelinesFor("audio", [aacCodec], { share }),
	canPlayTrack: canPlayTransmuxedTrack,
	reportUnsupportedTrackConditions: reportUnsupportedTransmuxedTrackConditions,
	sourceBufferMimeCodec: transmuxedSourceBufferMimeCodec
};
/** The state values the feature seeds. */
const initialState = {};
/**
* Plays HLS content packaged as MPEG-TS (H.264 + AAC) by transmuxing each segment to fMP4 on the client before it
* reaches the SourceBuffer. fMP4 and CMAF content plays through it unchanged. The `hls-video-mpeg-ts` and
* `hls-background-video-mpeg-ts` engines compose it; the container-neutral engines don't, because its codec and edge
* coverage is what Mux Video packages, so a general HLS source keeps the unsupported-format verdict there. An
* audio-only composition takes `mpegTsAudioTransmuxFeature` instead, which carries no H.264 framing. Compose either
* after `hlsLoadingFeature`, `shiftTimestampsFeature` and `drmFeature`: it wraps the parser the first sets, replaces
* the loader pipelines the second sets and the playability probe and reporter the third sets, and it needs
* `deriveStartMediaTime` from the second to place TS content, whose timestamps rarely start at zero, onto the playlist
* timeline.
*
* Scope is a deliberately narrow subset of MPEG-TS: what Mux Video packages on demand and live, which is one program,
* H.264 + AAC-LC, unencrypted, with access-unit delimiters. Not covered: MP3, AC-3, E-AC-3 or HEVC; SAMPLE-AES; ID3
* timed metadata; CEA-608 captions; a PES split across segments; 33-bit PTS rollover; and timestamps that jump at a
* discontinuity. Mux marks an encoder reconnect with a discontinuity but keeps its timestamps continuous across it. For
* general MPEG-TS, use `@videojs/hlsjs-video`.
*
* A muxed rendition's in-band audio is carried in the video SourceBuffer when the track declares an audio codec. When
* the audio is demuxed into renditions of its own, the video SourceBuffer drops it and the audio SourceBuffer takes
* each rendition's AAC — including the default rendition that HLS declares URI-less because it is carried in the video
* segments: the audio pipeline extracts it from the segments the video pipeline fetches, one request serving both.
*/
const mpegTsTransmuxFeature = defineFeature({
	behaviors,
	defaultConfig,
	initialState
});

//#endregion
export { mpegTsTransmuxFeature as t };
//# sourceMappingURL=mpeg-ts-transmux-DNsGmkV3.js.map