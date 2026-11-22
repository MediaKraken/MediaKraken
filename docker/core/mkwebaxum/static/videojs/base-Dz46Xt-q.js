import { t as pick } from "./pick-BAh7WYWl.js";
import { r as serializeAttributes, t as namedNodeMapToObject } from "./attributes-CI6LN9fK.js";
import { t as MediaAttachMixin } from "./media-attach-mixin-yt2XYok0.js";

//#region ../html/dist/dev/media/background-video/template.js
/**
* The shadow template both background-video elements render: a `<slot>` for a placeholder image, and a `<video>`
* stretched over the host.
*
* `attrs` go onto the `<video>` as given; each element decides which of its own attributes to forward. `src` is never
* among them, since serializing it would start a native load before an engine could take it.
*
* Shared so the two can't drift in presentation. `object-fit` and `object-position` are the only styling hooks — a
* background video has no controls to theme.
*/
function getTemplateHTML(attrs) {
	return `
    <style>
      :host {
        position: relative;
      }

      video {
        position: absolute;
        inset: 0;
        width: 100%;
        height: 100%;
        object-fit: var(--media-object-fit, inherit);
        object-position: var(--media-object-position, 50% 50%);
      }
    </style>
    <slot></slot>
    <video${serializeAttributes(attrs)}></video>
  `;
}
/**
* The `<video>` from `getTemplateHTML`. Always the shadow one: the template has no slot that would render a light-DOM
* `<video>`, so anything found there is either the placeholder or not displayed at all.
*/
function getTemplateVideo(shadowRoot) {
	const video = shadowRoot?.querySelector("video");
	return video instanceof HTMLVideoElement ? video : null;
}

//#endregion
//#region ../html/dist/dev/media/background-video/base.js
const HTMLElementBase = globalThis.HTMLElement ?? class {};
const MediaAttachBase = MediaAttachMixin(HTMLElementBase);
const PLAYBACK_DEFAULTS = [
	"muted",
	"loop",
	"autoplay"
];
/**
* The shadow setup, `src` wiring, and `target` that `<background-video>` and `<hls-background-video>` share. Each
* subclass supplies its own `src` and declares which playback defaults authors may opt out of.
*
* @internal
*/
var BackgroundVideoBase = class extends MediaAttachBase {
	static {
		this.shadowRootOptions = { mode: "open" };
	}
	static {
		this.getTemplateHTML = getTemplateHTML;
	}
	static {
		this.optOuts = [
			"muted",
			"loop",
			"autoplay"
		];
	}
	static {
		this.passthroughAttributes = [
			"controls",
			"controlslist",
			"crossorigin",
			"preload"
		];
	}
	static get observedAttributes() {
		return [
			"src",
			...this.optOuts,
			...this.optOuts.map((name) => `no${name}`),
			...this.passthroughAttributes
		];
	}
	constructor() {
		super();
		const ctor = this.constructor;
		if (!this.shadowRoot) {
			const attrs = {
				...pick(namedNodeMapToObject(this.attributes), ctor.passthroughAttributes),
				playsinline: "",
				disableremoteplayback: "",
				disablepictureinpicture: ""
			};
			for (const name of PLAYBACK_DEFAULTS) if (this.#isDefaultOn(name)) attrs[name] = "";
			this.attachShadow(ctor.shadowRootOptions);
			this.shadowRoot.innerHTML = ctor.getTemplateHTML(attrs);
		}
		this.syncPlaybackDefaults();
	}
	/**
	* Writes muted, loop, and autoplay onto the inner video as properties. The template's attributes only cover markup
	* parsed with its attributes in place; an element created by script gets its `no*` attributes afterwards, and neither
	* Chrome nor Firefox honor a `muted` attribute set after `document.createElement`.
	*/
	syncPlaybackDefaults() {
		const target = this.target;
		if (!target) return;
		for (const name of PLAYBACK_DEFAULTS) target[name] = this.#isDefaultOn(name);
	}
	/** On unless its `no*` attribute is set and allowed; an explicit attribute such as `muted` wins over its opt-out. */
	#isDefaultOn(name) {
		const { optOuts } = this.constructor;
		return this.hasAttribute(name) || !optOuts.includes(name) || !this.hasAttribute(`no${name}`);
	}
	attributeChangedCallback(name, oldValue, newValue) {
		if (oldValue === newValue) return;
		const { passthroughAttributes } = this.constructor;
		if (name === "src") this.src = newValue ?? "";
		else if (passthroughAttributes.includes(name)) {
			if (newValue === null) this.target?.removeAttribute(name);
			else this.target?.setAttribute(name, newValue);
		} else this.syncPlaybackDefaults();
	}
	/** The inner `<video>` in the shadow root: what plays, and what `play()` / `pause()` belong on. */
	get target() {
		return getTemplateVideo(this.shadowRoot);
	}
};

//#endregion
export { BackgroundVideoBase as t };
//# sourceMappingURL=base-Dz46Xt-q.js.map