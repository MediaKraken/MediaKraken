import { t as nativeHlsSupport } from "./supports-B0XryO1I.js";
import { a as matchServiceId, i as matchServiceHost, n as SPOTIFY_URI, o as parseSourceUrl, r as adapterTypeForMimeType, s as resolveMimeType } from "./adapter-type-C7BiENB5.js";

//#region ../media/dist/dev/core/source/lazy-media.js
/**
* Work out a source once for every media's `match`. Returns `null` for an empty source.
*
* @internal
*/
function resolveLazySource(src, options) {
	const structured = options.source ?? null;
	const source = (structured ? structured.src ?? "" : src).trim();
	const type = structured ? structured.type : options.type;
	const idType = structured && matchServiceId(structured);
	if (!source && !idType) return null;
	const url = source ? parseSourceUrl(source) : null;
	const service = source ? matchService(source, url) : null;
	const mimeType = resolveMimeType(source);
	const typeAdapter = adapterTypeForMimeType(type);
	return {
		src: source,
		type,
		url,
		adapterType: idType ?? (!mimeType && !typeAdapter || service === "mux" || service === "cloudflare" || service === "playerjs" ? service : null) ?? typeAdapter ?? adapterTypeForMimeType(mimeType),
		playback: resolvePlayback(options),
		source: structured
	};
}
/**
* The first media that plays a source: `null` when none does, or `undefined` while an earlier one can't tell yet.
*
* @internal
*/
function findLazyMedia(medias, source) {
	if (!source) return null;
	for (const media of medias) {
		const matched = media.match(source);
		if (matched !== false) return matched ? media : void 0;
	}
	return null;
}
/**
* A `match` for media that play any of the given kinds of source.
*
* @internal
*/
function matchAdapterType(...types) {
	return ({ adapterType }) => !!adapterType && types.includes(adapterType);
}
/**
* A `match` for HLS media that play by one path, native or MSE.
*
* @internal
*/
function matchHls(playback) {
	return ({ adapterType, playback: path }) => adapterType === "hls" && path && path === playback;
}
/**
* Hand a lazy media's `preferPlayback` on to the media it rendered, keeping the rest of its `source`. Unset, the media
* falls back to its own default.
*
* @internal
*/
function applyPreferPlayback(media, preferPlayback) {
	if (media.source?.preferPlayback === preferPlayback) return;
	const { preferPlayback: _, ...source } = media.source ?? {};
	const next = preferPlayback ? {
		...source,
		preferPlayback
	} : source;
	media.source = Object.keys(next).length > 0 ? next : null;
}
function matchService(src, url) {
	if (SPOTIFY_URI.test(src)) return "spotify";
	return SHORTHAND.exec(src)?.[1] ?? (url && matchServiceHost(url));
}
const SHORTHAND = /^(youtube|vimeo)\//;
function resolvePlayback({ preferPlayback, nativeHls }) {
	if (preferPlayback === "mse") return "mse";
	if (!nativeHls) return void 0;
	return (preferPlayback === "native" ? nativeHls !== "unsupported" : nativeHls === "preferred") ? "native" : "mse";
}

//#endregion
//#region ../html/dist/dev/media/lazy-media.js
const HTMLElementBase = globalThis.HTMLElement ?? class {};
const OWN_ATTRIBUTES = /* @__PURE__ */ new Set([
	"id",
	"class",
	"style",
	"slot",
	"part",
	"type",
	"prefer-playback"
]);
const LAZY_MEDIA_DOCS_URL = "https://videojs.org/docs/framework/html/reference/components/lazy-video";
/**
* Base for `<lazy-video>` and `<lazy-audio>`: resolves `src` to a media element, loads the module that registers it,
* and renders it as a child with this element's attributes. The child registers itself with the player, as it would if
* it were written in the page.
*
* @experimental
*/
var LazyMediaElement = class extends HTMLElementBase {
	static get observedAttributes() {
		return [
			"src",
			"type",
			"prefer-playback"
		];
	}
	#media;
	#mediaDefinition;
	#definition;
	#source;
	#mediaSource;
	#token;
	#updateQueued;
	#observer;
	/** Source URL. Its host or file extension decides which media element plays it. Ignored while `source` is set. */
	get src() {
		return this.getAttribute("src") ?? "";
	}
	set src(value) {
		this.setAttribute("src", value);
	}
	/**
	* MIME type of the source. Takes precedence over the file extension, so signed or extensionless URLs resolve. Ignored
	* while `source` is set.
	*/
	get type() {
		return this.getAttribute("type") ?? "";
	}
	set type(value) {
		this.setAttribute("type", value);
	}
	/**
	* Structured source, in place of `src` and `type`. Its `src`, `type`, or an id such as a Mux `playbackId` or a Wistia
	* `mediaId` decides which media element plays it, and the media takes the whole object on its `source`. A native
	* `<video>` or `<audio>` takes only its `src`.
	*/
	get source() {
		return this.#source;
	}
	set source(value) {
		if (value === this.#source) return;
		this.#source = value ?? null;
		this.#queueUpdate();
	}
	/**
	* Playback path for HLS: `'native'` uses the browser's own HLS support wherever it has one, and `'mse'` always loads
	* an engine. Unset, HLS plays through an engine wherever there's MSE, and natively only where there's none. Also
	* passed on to the hls.js and Mux media, on their `source`.
	*/
	get preferPlayback() {
		const value = this.getAttribute("prefer-playback");
		return value === "native" || value === "mse" ? value : void 0;
	}
	set preferPlayback(value) {
		if (value) this.setAttribute("prefer-playback", value);
		else this.removeAttribute("prefer-playback");
	}
	constructor() {
		super();
		this.#media = null;
		this.#mediaDefinition = null;
		this.#definition = void 0;
		this.#source = null;
		this.#mediaSource = null;
		this.#token = 0;
		this.#updateQueued = false;
		this.#observer = null;
		this.medias = [];
		if (!this.shadowRoot) this.attachShadow({ mode: "open" }).innerHTML = TEMPLATE;
	}
	connectedCallback() {
		this.#observer ??= new MutationObserver((records) => this.#onMutation(records));
		this.#observer.observe(this, { attributes: true });
		if (this.#media) this.#copyAttributes(this.#media);
		this.#queueUpdate();
	}
	disconnectedCallback() {
		this.#observer?.disconnect();
	}
	attributeChangedCallback() {
		this.#queueUpdate();
	}
	#queueUpdate() {
		if (this.#updateQueued) return;
		this.#updateQueued = true;
		queueMicrotask(() => {
			this.#updateQueued = false;
			this.#update();
		});
	}
	#update() {
		if (!this.isConnected) return;
		const src = this.#source ? this.#source.src ?? "" : this.src;
		const source = resolveLazySource(src, {
			type: this.type || void 0,
			preferPlayback: this.#preferPlayback(),
			nativeHls: nativeHlsSupport(),
			source: this.#source
		});
		const definition = findLazyMedia(this.medias, source);
		if (definition === void 0) return;
		if (definition === this.#definition) {
			if (definition && this.#media && this.#mediaDefinition === definition) this.#syncSource(this.#media, definition);
			return;
		}
		this.#definition = definition;
		if (!definition) {
			this.#token++;
			this.#setMedia(null, null);
			if (src) console.error(`[${this.localName}] No media plays "${src}". If the URL has no file extension, set its MIME type in the \`type\` attribute, or use the media element for it. Supported sources: ${LAZY_MEDIA_DOCS_URL}`);
			return;
		}
		this.#load(definition);
	}
	async #load(definition) {
		const token = ++this.#token;
		try {
			await definition.load?.();
		} catch (error) {
			console.error(`[${this.localName}] Failed to load <${definition.tag}>.`, error);
			if (token === this.#token) {
				this.#definition = void 0;
				this.#setMedia(null, null);
			}
			return;
		}
		if (token !== this.#token) return;
		const media = document.createElement(definition.tag);
		this.#copyAttributes(media);
		if (this.hasAttribute("muted") && "muted" in media) media.muted = true;
		this.#mediaSource = null;
		this.#syncSource(media, definition);
		this.#setMedia(media, definition);
	}
	#setMedia(next, definition) {
		const previous = this.#media;
		const children = (previous ?? this).querySelectorAll(":scope > track, :scope > source");
		if (next) {
			next.append(...children);
			this.append(next);
		} else this.append(...children);
		previous?.remove();
		this.#media = next;
		this.#mediaDefinition = definition;
	}
	#copyAttributes(media) {
		for (const { name } of this.attributes) this.#syncAttribute(media, name);
		if (!this.hasAttribute("preload")) this.#syncAttribute(media, "preload");
	}
	#syncAttribute(media, name) {
		if (OWN_ATTRIBUTES.has(name) || name === "src") return;
		const value = this.getAttribute(name) ?? (name === "preload" ? "none" : null);
		if (value === null) media.removeAttribute(name);
		else if (media.getAttribute(name) !== value) media.setAttribute(name, value);
	}
	#syncSource(media, definition) {
		const source = this.#source;
		if (!source) {
			if (this.#mediaSource && "source" in media) {
				media.source = null;
				media.removeAttribute("src");
			}
			this.#mediaSource = null;
			this.#setSrc(media, this.getAttribute("src"));
		} else if (!("source" in media)) this.#setSrc(media, source.src ?? null);
		else if (source !== this.#mediaSource) {
			this.#mediaSource = source;
			media.source = source;
		}
		if (definition.preferPlayback) applyPreferPlayback(media, this.#preferPlayback());
	}
	#setSrc(media, value) {
		if (value === null) media.removeAttribute("src");
		else if (media.getAttribute("src") !== value) media.setAttribute("src", value);
	}
	#preferPlayback() {
		const fromSource = this.#source?.preferPlayback;
		return this.preferPlayback ?? (fromSource === "native" || fromSource === "mse" ? fromSource : void 0);
	}
	#onMutation(records) {
		const media = this.#media;
		if (!media) return;
		for (const { attributeName } of records) if (attributeName !== "src") this.#syncAttribute(media, attributeName);
	}
};
const TEMPLATE = `<style>
  :host {
    display: contents;
  }

  ::slotted(video),
  ::slotted([data-cross-origin-frame]) {
    display: block;
    width: 100%;
    height: 100%;
  }

  ::slotted([data-cross-origin-frame]) {
    min-width: 0;
    min-height: 0;
    aspect-ratio: 16 / 9;
  }

  ::slotted(video) {
    border-radius: var(--media-video-border-radius);
    object-fit: var(--media-object-fit, contain);
    object-position: var(--media-object-position, center);
  }
</style><slot></slot>`;

//#endregion
export { matchAdapterType as n, matchHls as r, LazyMediaElement as t };
//# sourceMappingURL=lazy-media-BfiTAvWe.js.map