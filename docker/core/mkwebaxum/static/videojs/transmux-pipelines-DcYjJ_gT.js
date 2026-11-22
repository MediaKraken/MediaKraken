import { t as generateId } from "./generate-id-DHFQ63E5.js";
import { M as fetchStep, Mt as peek, Pt as update, Qt as getTracksByType, Xt as findTrackById, Yt as findResolvedVideoTrack, d as relocationStepsFor, j as dispatchStep } from "./error-surface-BHz7DxGx.js";
import { u as parseCodecs } from "./session-data-6y7aXrmJ.js";

//#region ../spf/dist/dev/media/hls/carried-audio.js
/**
* Audio renditions carried in video segments. HLS lets an `EXT-X-MEDIA` audio rendition omit its `URI`, meaning its
* media is muxed into the segments of every variant that references its group. The multivariant parser keeps such a
* rendition as a track with an empty `url`, which nothing can fetch: it never resolves, and the active-CDN scope (keyed
* on the URL's origin) never picks it. A composition that can extract an elementary stream from a muxed segment — the
* MPEG-TS transmux feature — resolves the rendition here instead: it borrows the playlist of the lowest-bandwidth
* variant carrying its group (its _carrier_), so it resolves, scopes to the video CDN, and plans segments like any
* audio rendition. Which variant's segment is actually fetched is the loader pipeline's call, made per segment against
* the selected video track so the bytes are shared with the video fetch.
*/
/** The lowest-bandwidth video track advertising the audio group. */
function carrierFor(videoTracks, groupId) {
	return videoTracks.filter((track) => track.audioGroupIds?.includes(groupId)).reduce((lowest, track) => !lowest || track.bandwidth < lowest.bandwidth ? track : lowest, void 0);
}
/** Give every URI-less audio rendition its carrier variant's playlist URL. Returns the input when nothing changes. */
function resolveCarriedAudioRenditions(presentation) {
	const videoTracks = getTracksByType(presentation, "video");
	let changed = false;
	const selectionSets = presentation.selectionSets.map((selectionSet) => {
		if (selectionSet.type !== "audio") return selectionSet;
		return {
			...selectionSet,
			switchingSets: selectionSet.switchingSets.map((switchingSet) => ({
				...switchingSet,
				tracks: switchingSet.tracks.map((track) => {
					const carrier = track.url === "" ? carrierFor(videoTracks, track.groupId) : void 0;
					if (!carrier) return track;
					changed = true;
					return {
						...track,
						url: carrier.url
					};
				})
			}))
		};
	});
	return changed ? {
		...presentation,
		selectionSets
	} : presentation;
}
/**
* Give a presentation that declares no audio, but whose variants carry it muxed in, one default audio track carried by
* the lowest-bandwidth such variant. That is Mux's single-audio MPEG-TS shape: no `EXT-X-MEDIA` audio, the audio codec
* listed on every variant. A video composition plays that audio from the video SourceBuffer; an audio-only one has no
* other track to select. Returns the input when it declares audio or no variant carries any.
*/
function addMuxedAudioRendition(presentation) {
	if (getTracksByType(presentation, "audio").length > 0) return presentation;
	const carrier = getTracksByType(presentation, "video").flatMap((track) => {
		const audioCodec = track.audioGroupIds ? void 0 : parseCodecs(track.codecs?.join(",") ?? "").audio;
		return audioCodec ? [{
			track,
			audioCodec
		}] : [];
	}).reduce((lowest, candidate) => !lowest || candidate.track.bandwidth < lowest.track.bandwidth ? candidate : lowest, void 0);
	if (!carrier) return presentation;
	const track = {
		type: "audio",
		id: generateId(),
		url: carrier.track.url,
		bandwidth: carrier.track.bandwidth,
		mimeType: "audio/mp4",
		codecs: [carrier.audioCodec],
		groupId: "default",
		name: "Default",
		sampleRate: 48e3,
		channels: 2,
		default: true,
		autoselect: true
	};
	const audioSelectionSet = {
		id: generateId(),
		type: "audio",
		switchingSets: [{
			id: generateId(),
			type: "audio",
			tracks: [track]
		}]
	};
	return {
		...presentation,
		selectionSets: [...presentation.selectionSets, audioSelectionSet]
	};
}
/** Whether an audio track's media is carried in video segments: it shares a playlist URL with a video track. */
function isCarriedAudioTrack(presentation, track) {
	if (track.type !== "audio" || track.url === "") return false;
	return getTracksByType(presentation, "video").some((video) => video.url === track.url);
}

//#endregion
//#region ../spf/dist/dev/media/mp4/mux.js
function concat$2(parts) {
	const out = new Uint8Array(parts.reduce((n, p) => n + p.length, 0));
	let offset = 0;
	for (const part of parts) {
		out.set(part, offset);
		offset += part.length;
	}
	return out;
}
function u16(value) {
	return Uint8Array.from([value >> 8 & 255, value & 255]);
}
function u32(value) {
	return Uint8Array.from([
		value >>> 24 & 255,
		value >>> 16 & 255,
		value >>> 8 & 255,
		value & 255
	]);
}
function u64(value) {
	return concat$2([u32(Math.floor(value / 2 ** 32)), u32(value % 2 ** 32)]);
}
function fourcc(type) {
	return Uint8Array.from(type, (c) => c.charCodeAt(0));
}
const zeros = (count) => new Uint8Array(count);
/** `[u32 size][4cc type][payload…]` */
function box(type, ...payload) {
	const body = concat$2(payload);
	const out = new Uint8Array(8 + body.length);
	out.set(u32(out.length), 0);
	out.set(fourcc(type), 4);
	out.set(body, 8);
	return out;
}
function fullBox(type, version, flags, ...payload) {
	return box(type, Uint8Array.from([
		version,
		flags >> 16 & 255,
		flags >> 8 & 255,
		flags & 255
	]), ...payload);
}
const UNITY_MATRIX = concat$2([
	u32(65536),
	zeros(12),
	u32(65536),
	zeros(12),
	u32(1073741824)
]);
function tkhd(track) {
	const { width = 0, height = 0 } = track;
	return fullBox("tkhd", 0, 7, zeros(8), u32(track.id), zeros(4), zeros(4), zeros(8), zeros(4), u16(track.handler === "soun" ? 256 : 0), zeros(2), UNITY_MATRIX, u32(width << 16), u32(height << 16));
}
function mdhd(timescale) {
	return fullBox("mdhd", 0, 0, zeros(8), u32(timescale), zeros(4), u16(21956), zeros(2));
}
function hdlr(track) {
	return fullBox("hdlr", 0, 0, zeros(4), fourcc(track.handler), zeros(12), fourcc(track.handler === "vide" ? "VideoHandler" : "SoundHandler"), zeros(1));
}
const DINF = box("dinf", fullBox("dref", 0, 0, u32(1), fullBox("url ", 0, 1)));
const EMPTY_TABLES = concat$2([
	fullBox("stts", 0, 0, u32(0)),
	fullBox("stsc", 0, 0, u32(0)),
	fullBox("stsz", 0, 0, u32(0), u32(0)),
	fullBox("stco", 0, 0, u32(0))
]);
function trak(track) {
	const mediaHeader = track.handler === "vide" ? fullBox("vmhd", 0, 1, zeros(8)) : fullBox("smhd", 0, 0, zeros(4));
	return box("trak", tkhd(track), box("mdia", mdhd(track.timescale), hdlr(track), box("minf", mediaHeader, DINF, box("stbl", fullBox("stsd", 0, 0, u32(1), track.sampleEntry), EMPTY_TABLES))));
}
function trex(track) {
	return fullBox("trex", 0, 0, u32(track.id), u32(1), u32(0), u32(0), u32(65537));
}
const FTYP = box("ftyp", fourcc("isom"), u32(1), fourcc("isom"), fourcc("avc1"));
/** `ftyp` + `moov` for the given tracks. Movie timescale is the first track's; durations are left to the fragments. */
function buildInitSegment(tracks) {
	const mvhd = fullBox("mvhd", 0, 0, zeros(8), u32(tracks[0]?.timescale ?? 9e4), zeros(4), u32(65536), u16(256), zeros(10), UNITY_MATRIX, zeros(24), u32(4294967295));
	return concat$2([FTYP, box("moov", mvhd, ...tracks.map(trak), box("mvex", ...tracks.map(trex)))]);
}
const SAMPLE_FLAGS_KEY = 33554432;
const SAMPLE_FLAGS_NON_KEY = 16842752;
const TRUN_FLAGS = 3841;
const TFHD_FLAGS = 131072;
/**
* One track's `moof` + `mdat`. `mdat` payload is the concatenated sample bytes (callers prefix NAL units with their
* 4-byte lengths for video). `baseMediaDecodeTime` is the first sample's DTS in track ticks.
*/
function buildMediaFragment(trackId, sequenceNumber, baseMediaDecodeTime, samples, payload) {
	const rows = new Uint8Array(16 * samples.length);
	const view = new DataView(rows.buffer);
	samples.forEach((sample, i) => {
		view.setUint32(i * 16, sample.duration);
		view.setUint32(i * 16 + 4, sample.size);
		view.setUint32(i * 16 + 8, sample.key ? SAMPLE_FLAGS_KEY : SAMPLE_FLAGS_NON_KEY);
		view.setInt32(i * 16 + 12, sample.cts);
	});
	const dataOffset = 96 + rows.length;
	return concat$2([box("moof", fullBox("mfhd", 0, 0, u32(sequenceNumber)), box("traf", fullBox("tfhd", 0, TFHD_FLAGS, u32(trackId)), fullBox("tfdt", 1, 0, u64(baseMediaDecodeTime)), fullBox("trun", 1, TRUN_FLAGS, u32(samples.length), u32(dataOffset), rows))), box("mdat", payload)]);
}

//#endregion
//#region ../spf/dist/dev/media/mpeg-ts/demux.js
const SYNC_BYTE = 71;
/** Container sniff: a TS segment starts on a sync byte and keeps the 188-byte cadence. */
function isTransportStream(bytes) {
	return bytes[0] === SYNC_BYTE && (bytes.length <= 188 || bytes[188] === SYNC_BYTE);
}
/**
* A stream's PES payloads as one continuous elementary stream plus the PES boundaries. The packager need not align PES
* packets with access units or audio frames (Mux's doesn't: a picture's slice data can run into the next PES), so codec
* framing walks the continuous stream and takes its timing from the PES in which each unit _begins_.
*/
function concatPes(packets) {
	const data = new Uint8Array(packets.reduce((n, p) => n + p.data.length, 0));
	const boundaries = [];
	let offset = 0;
	for (const packet of packets) {
		boundaries.push({
			offset,
			pts: packet.pts,
			dts: packet.dts
		});
		data.set(packet.data, offset);
		offset += packet.data.length;
	}
	return {
		data,
		boundaries
	};
}
/** 33-bit PTS/DTS from its 5-byte PES encoding — multiplies rather than shifts, since the value exceeds 32 bits. */
function readTimestamp(data, offset) {
	return (data[offset] & 14) * 2 ** 29 + data[offset + 1] * 2 ** 22 + (data[offset + 2] & 254) * 2 ** 14 + data[offset + 3] * 128 + (data[offset + 4] >> 1);
}
function concat$1(chunks) {
	if (chunks.length === 1) return chunks[0];
	const out = new Uint8Array(chunks.reduce((n, c) => n + c.length, 0));
	let offset = 0;
	for (const chunk of chunks) {
		out.set(chunk, offset);
		offset += chunk.length;
	}
	return out;
}
/** Parse one reassembled PES packet; `undefined` when the chunks don't start with a PES start code. */
function parsePes(chunks) {
	const data = concat$1(chunks);
	if (data.length < 9 || data[0] !== 0 || data[1] !== 0 || data[2] !== 1) return void 0;
	const flags = data[7];
	const headerLength = data[8];
	const pts = flags & 128 ? readTimestamp(data, 9) : void 0;
	return {
		pts,
		dts: flags & 64 ? readTimestamp(data, 14) : pts,
		data: data.subarray(9 + headerLength)
	};
}
/** PAT: the first program's PMT PID. */
function parsePat(data, offset) {
	return (data[offset + 10] & 31) << 8 | data[offset + 11];
}
/** PMT: `(pid, stream_type)` for every elementary stream in the program. */
function parsePmt(data, offset) {
	const sectionLength = (data[offset + 1] & 15) << 8 | data[offset + 2];
	const tableEnd = offset + 3 + sectionLength - 4;
	const programInfoLength = (data[offset + 10] & 15) << 8 | data[offset + 11];
	const streams = [];
	for (let i = offset + 12 + programInfoLength; i < tableEnd;) {
		const esInfoLength = (data[i + 3] & 15) << 8 | data[i + 4];
		streams.push({
			streamType: data[i],
			pid: (data[i + 1] & 31) << 8 | data[i + 2]
		});
		i += 5 + esInfoLength;
	}
	return streams;
}
/**
* Demux a complete TS segment into the PES packets of the requested PMT stream types, keyed by stream type (the first
* PID the PMT declares for each). Streams of other types are skipped at the packet level, so an audio-only consumer
* never assembles the video PES. Packets are delimited by the payload-unit-start indicator, so a PES whose length field
* is 0 (unbounded, the norm for video) still reassembles correctly; a trailing PES cut off by the segment end is kept
* as-is (its payload is simply short).
*/
function demuxTransportStream(bytes, streamTypes) {
	const assembling = /* @__PURE__ */ new Map();
	const claimed = /* @__PURE__ */ new Set();
	let pmtPid = -1;
	const flush = (entry) => {
		const pes = entry.chunks && parsePes(entry.chunks);
		if (pes) entry.stream.packets.push(pes);
	};
	for (let start = 0; start + 188 <= bytes.length; start += 188) {
		if (bytes[start] !== SYNC_BYTE) {
			start -= 187;
			continue;
		}
		const payloadStart = (bytes[start + 1] & 64) !== 0;
		const pid = (bytes[start + 1] & 31) << 8 | bytes[start + 2];
		const adaptation = (bytes[start + 3] & 48) >> 4;
		let offset = start + 4;
		if (adaptation & 2) offset += 1 + bytes[start + 4];
		if (!(adaptation & 1) || offset >= start + 188) continue;
		if (pid === 0) {
			pmtPid = parsePat(bytes, payloadStart ? offset + 1 + bytes[offset] : offset);
			continue;
		}
		if (pid === pmtPid) {
			for (const { pid: esPid, streamType } of parsePmt(bytes, payloadStart ? offset + 1 + bytes[offset] : offset)) if (!assembling.has(esPid) && streamTypes.has(streamType) && !claimed.has(streamType)) {
				claimed.add(streamType);
				assembling.set(esPid, {
					stream: {
						pid: esPid,
						streamType,
						packets: []
					},
					chunks: void 0
				});
			}
			continue;
		}
		const entry = assembling.get(pid);
		if (!entry) continue;
		if (payloadStart) {
			flush(entry);
			entry.chunks = [];
		}
		entry.chunks?.push(bytes.subarray(offset, start + 188));
	}
	const streams = /* @__PURE__ */ new Map();
	for (const entry of assembling.values()) {
		flush(entry);
		streams.set(entry.stream.streamType, entry.stream);
	}
	return streams;
}

//#endregion
//#region ../spf/dist/dev/media/mpeg-ts/adts.js
/**
* ADTS AAC elementary-stream parsing for the TS transmux path: walks the ADTS frames across a stream's PES payloads,
* strips each 7/9-byte header to the raw AAC frame MP4 carries, and derives the `AudioSpecificConfig` for `esds`.
*
* Frame timing re-anchors on each PES that opens on a frame boundary (its PTS); frames that follow inside the same PES
* advance by one AAC frame (1024 samples) each.
*/
const SAMPLE_RATES = [
	96e3,
	88200,
	64e3,
	48e3,
	44100,
	32e3,
	24e3,
	22050,
	16e3,
	12e3,
	11025,
	8e3,
	7350
];
const AAC_SAMPLES_PER_FRAME = 1024;
const MPEG_TS_TIMESCALE$1 = 9e4;
function isSyncWord(data, offset) {
	return data[offset] === 255 && (data[offset + 1] & 246) === 240;
}
function readConfig(data, offset) {
	const objectType = (data[offset + 2] >> 6 & 3) + 1;
	const samplingIndex = data[offset + 2] >> 2 & 15;
	const channels = (data[offset + 2] & 1) << 2 | data[offset + 3] >> 6 & 3;
	return {
		sampleRate: SAMPLE_RATES[samplingIndex] ?? 48e3,
		channels,
		audioSpecificConfig: Uint8Array.from([objectType << 3 | samplingIndex >> 1, (samplingIndex & 1) << 7 | channels << 3]),
		codec: `mp4a.40.${objectType}`
	};
}
/** Split a stream's PES payloads into raw AAC frames with PTS. A frame cut off by the segment end is dropped. */
function parseAdtsFrames(packets) {
	const { data, boundaries } = concatPes(packets);
	const ptsAtOffset = new Map(boundaries.filter((b) => b.pts !== void 0).map((b) => [b.offset, b.pts]));
	const samples = [];
	let config;
	let pts = 0;
	let frameDuration = 0;
	for (let offset = 0; offset + 7 <= data.length;) {
		if (!isSyncWord(data, offset)) {
			offset++;
			continue;
		}
		const headerLength = data[offset + 1] & 1 ? 7 : 9;
		const frameLength = (data[offset + 3] & 3) << 11 | data[offset + 4] << 3 | data[offset + 5] >> 5;
		if (frameLength < headerLength || offset + frameLength > data.length) break;
		if (!config) {
			config = readConfig(data, offset);
			frameDuration = AAC_SAMPLES_PER_FRAME * MPEG_TS_TIMESCALE$1 / config.sampleRate;
		}
		pts = ptsAtOffset.get(offset) ?? pts + frameDuration;
		samples.push({
			pts,
			data: data.subarray(offset + headerLength, offset + frameLength)
		});
		offset += frameLength;
	}
	return config ? {
		samples,
		config
	} : { samples };
}

//#endregion
//#region ../spf/dist/dev/media/mpeg-ts/codecs/aac.js
const MPEG_TS_TIMESCALE = 9e4;
function mp4a(sampleRate, channels, asc) {
	const esds = fullBox("esds", 0, 0, Uint8Array.from([
		3,
		23 + asc.length,
		0,
		1,
		0,
		4,
		15 + asc.length,
		64,
		21,
		0,
		0,
		0,
		0,
		0,
		0,
		0,
		0,
		0,
		0,
		0,
		5,
		asc.length
	]), asc, Uint8Array.from([
		6,
		1,
		2
	]));
	return box("mp4a", zeros(6), u16(1), zeros(8), u16(channels), u16(16), zeros(4), u32(sampleRate << 16), esds);
}
const aacCodec = {
	streamType: 15,
	family: "mp4a",
	frame(packets) {
		const { samples, config } = parseAdtsFrames(packets);
		if (!config || samples.length === 0) return void 0;
		const { sampleRate, channels, audioSpecificConfig } = config;
		const track = {
			id: 2,
			handler: "soun",
			timescale: sampleRate,
			sampleEntry: mp4a(sampleRate, channels, audioSpecificConfig)
		};
		const payload = new Uint8Array(samples.reduce((n, s) => n + s.data.length, 0));
		let offset = 0;
		for (const sample of samples) {
			payload.set(sample.data, offset);
			offset += sample.data.length;
		}
		return {
			track,
			baseMediaDecodeTime: Math.round(samples[0].pts * sampleRate / MPEG_TS_TIMESCALE),
			samples: samples.map((sample) => ({
				duration: AAC_SAMPLES_PER_FRAME,
				size: sample.data.length,
				key: true,
				cts: 0
			})),
			payload
		};
	}
};

//#endregion
//#region ../spf/dist/dev/media/mpeg-ts/transmux.js
/**
* MPEG-TS → fMP4 transmuxer: demux a complete TS segment, frame each carried codec's elementary stream, and write the
* frames back as fMP4 fragments the MSE pipeline already handles. Composed from codec modules (`codecs/`), so a
* composition bundles only the codecs it carries. Stateful per SourceBuffer: it numbers fragments and keeps each
* codec's last sample entry, so a segment missing one track's data still describes every track. It builds an init
* segment whenever the sample entries differ from the ones the caller last delivered (first segment, rendition switch).
* The caller tracks delivery because only it knows whether the append carrying an init completed.
*
* Timestamps stay native (first DTS in `tfdt`, in each track's own timescale) — relocation onto the presentation
* timeline is the existing `timestampOffset` machinery's job, exactly as for native fMP4.
*/
function concat(parts) {
	const out = new Uint8Array(parts.reduce((n, p) => n + p.length, 0));
	let offset = 0;
	for (const part of parts) {
		out.set(part, offset);
		offset += part.length;
	}
	return out;
}
/** A transmuxer for one SourceBuffer's stream of segments, carrying the given codecs. */
function createTransmuxer(codecs) {
	let sequenceNumber = 0;
	const configs = /* @__PURE__ */ new Map();
	return (segment, { codecs: carried = codecs, deliveredInitKey, ...options } = {}) => {
		const streams = demuxTransportStream(segment, new Set(carried.map((codec) => codec.streamType)));
		const fragments = [];
		const result = {
			data: /* @__PURE__ */ new Uint8Array(0),
			tracks: {}
		};
		for (const codec of carried) {
			const stream = streams.get(codec.streamType);
			const framed = stream && codec.frame(stream.packets, options);
			if (!framed) continue;
			const { track, baseMediaDecodeTime, samples, payload } = framed;
			configs.set(codec.family, track);
			fragments.push(buildMediaFragment(track.id, ++sequenceNumber, baseMediaDecodeTime, samples, payload));
			result.tracks[codec.family] = {
				trackId: track.id,
				timescale: track.timescale,
				baseMediaDecodeTime
			};
		}
		const tracks = carried.flatMap((codec) => configs.get(codec.family) ?? []);
		const initKey = tracks.map((track) => `${track.id}:${track.sampleEntry.join()}`).join("|");
		if (tracks.length) {
			result.initKey = initKey;
			if (initKey !== deliveredInitKey) result.init = buildInitSegment(tracks);
		}
		result.data = concat(fragments);
		return result;
	};
}

//#endregion
//#region ../spf/dist/dev/playback/primitives/transmux-pipelines.js
/** How long the audio pipeline waits for the video pipeline to publish a carried segment before fetching it itself. */
const SHARE_WAIT_MS = 1e3;
function transmuxState(deps) {
	return deps.state;
}
/** The loader's own fetch, folded into the threaded config (see `segment-load-pipeline`'s `stepWiring`). */
function loaderFetch(deps) {
	return deps.config.fetch;
}
async function collect(data) {
	const chunks = [];
	for await (const chunk of data) chunks.push(chunk);
	if (chunks.length === 1) return chunks[0];
	const out = new Uint8Array(chunks.reduce((n, c) => n + c.length, 0));
	let offset = 0;
	for (const chunk of chunks) {
		out.set(chunk, offset);
		offset += chunk.length;
	}
	return out;
}
async function* streamOf(...parts) {
	yield* parts;
}
/** The share key of a segment request: its URL plus any byte range. */
function shareKey({ url, byteRange }) {
	return byteRange ? `${url}#${byteRange.start}-${byteRange.end}` : url;
}
/**
* Which of the pipeline's codecs this SourceBuffer carries this time, from the track's declared codecs: a muxed TS
* rendition whose audio is demuxed into its own renditions (`AUDIO` group) declares only a video codec, so its in-band
* audio is dropped rather than appended twice. An undeclared `CODECS` carries every codec the pipeline has.
*/
function transmuxOptionsFor(codecs, deps, trackId) {
	const presentation = peek(transmuxState(deps).presentation);
	const track = presentation && findTrackById(presentation, trackId);
	const declared = track && "codecs" in track ? track.codecs ?? [] : [];
	const declares = (family) => declared.some((codec) => codec.toLowerCase().startsWith(family));
	const resolution = track && "width" in track && track.width && track.height ? {
		width: track.width,
		height: track.height
	} : void 0;
	return {
		codecs: declared.length === 0 ? codecs : codecs.filter((codec) => declares(codec.family)),
		...resolution && { resolution }
	};
}
/**
* Transmux pipelines for one track type — the TS analog of `relocationPipelinesFor`, carrying `codecs` (the first is
* the type's own: the one whose track relocation keys on). One transmuxer per actor (built inside the per-actor
* factory), so fragment numbering and the delivered-init memory reset with the source. The relocation stamp reads
* `deriveStartMediaTime` from the composition config, as in the relocation pipelines.
*/
function transmuxPipelinesFor(trackType, codecs, { share } = {}) {
	const { readInitTrackInfo, readSegmentOrigin, stampStartMediaTime } = relocationStepsFor(trackType);
	const ownFamily = codecs[0].family;
	/** Video media fetch: publish the segment to the share while it downloads, for the audio pipeline to take. */
	const fetchVideoSegment = async (frame, signal, deps) => {
		const { op } = frame;
		if (op.type !== "append-segment" || !share) return fetchStep(frame, signal, deps);
		const bytes = (async () => {
			await fetchStep(frame, signal, deps);
			return collect(frame.data);
		})();
		share.publish(shareKey(op), bytes);
		frame.data = streamOf(await bytes);
	};
	/**
	* Audio media fetch: a carried rendition's segment is fetched from the selected video track at the same position (the
	* carrier's own when that track hasn't resolved), taken from the share when the video pipeline has it in flight.
	*/
	const fetchAudioSegment = async (frame, signal, deps) => {
		const { op } = frame;
		if (op.type !== "append-segment") return;
		const state = transmuxState(deps);
		const presentation = peek(state.presentation);
		const track = presentation && findTrackById(presentation, op.meta.trackId);
		if (!presentation || !track || !isCarriedAudioTrack(presentation, track)) return fetchStep(frame, signal, deps);
		const addressable = findResolvedVideoTrack(presentation, state.selectedVideoTrackId && peek(state.selectedVideoTrackId))?.segments.find((s) => Math.abs(s.startTime - op.meta.startTime) < 1e-4) ?? op;
		const shared = share && await share.take(shareKey(addressable), SHARE_WAIT_MS);
		if (signal.aborted) return;
		frame.data = shared ? streamOf(shared) : await loaderFetch(deps)(addressable, { signal });
	};
	const fetchSegment = trackType === "video" ? fetchVideoSegment : fetchAudioSegment;
	return () => {
		const transmux = createTransmuxer(codecs);
		let deliveredInitKey;
		const pendingInitKeys = /* @__PURE__ */ new WeakMap();
		/** Init step: nothing to fetch for a TS rendition; a real init URL takes the fMP4 path. */
		const fetchInit = async (frame, signal, deps) => {
			if (frame.op.type !== "append-init") return;
			if (frame.op.url === "") {
				frame.data = streamOf(/* @__PURE__ */ new Uint8Array(0));
				return;
			}
			await fetchStep(frame, signal, deps);
			await readInitTrackInfo(frame, signal, deps);
		};
		/**
		* Media step: buffer the fetched segment, rewrite TS as fMP4, and record the init-segment half of this type's
		* `mediaContainerData` (track id + timescale, what `readInitTrackInfo` reads from a real init) so the relocation
		* discover step that follows can match the fragment's `tfdt`.
		*/
		const transmuxStep = async (frame, signal, deps) => {
			const { op } = frame;
			if (op.type !== "append-segment" || !frame.data) return;
			const bytes = await collect(frame.data);
			if (signal.aborted) return;
			if (!isTransportStream(bytes)) {
				frame.data = streamOf(bytes);
				return;
			}
			const result = transmux(bytes, {
				...transmuxOptionsFor(codecs, deps, op.meta.trackId),
				deliveredInitKey
			});
			const own = result.tracks[ownFamily];
			const slot = transmuxState(deps).mediaContainerData;
			if (slot && own && peek(slot)?.[trackType]?.timescale === void 0) update(slot, (current) => ({
				...current,
				[trackType]: {
					...current?.[trackType],
					trackId: own.trackId,
					timescale: own.timescale
				}
			}));
			if (result.init && result.initKey !== void 0) pendingInitKeys.set(frame, result.initKey);
			frame.data = result.init ? streamOf(result.init, result.data) : streamOf(result.data);
		};
		/** After the append: the init this frame carried has reached the SourceBuffer. The runner skips it on abort. */
		const confirmInit = (frame) => {
			const key = pendingInitKeys.get(frame);
			if (key !== void 0) deliveredInitKey = key;
		};
		return {
			remove: [dispatchStep],
			"append-init": [fetchInit, dispatchStep],
			"append-segment": [
				fetchSegment,
				transmuxStep,
				readSegmentOrigin,
				stampStartMediaTime,
				dispatchStep,
				confirmInit
			]
		};
	};
}

//#endregion
export { u16 as a, addMuxedAudioRendition as c, box as i, resolveCarriedAudioRenditions as l, aacCodec as n, u32 as o, concatPes as r, zeros as s, transmuxPipelinesFor as t };
//# sourceMappingURL=transmux-pipelines-DcYjJ_gT.js.map