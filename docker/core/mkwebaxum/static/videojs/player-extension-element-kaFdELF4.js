import { f as isUndefined } from "./predicate-3rF1m2uv.js";
import { t as shallowEqual } from "./shallow-equal-C7S8rj2f.js";
import { t as ContextConsumer } from "./context-BVVXMx3G.js";
import { t as UIElement } from "./ui-element-Ctm6zaRJ.js";
import { n as extensionContext } from "./context-CL8SSE10.js";

//#region ../html/dist/dev/extensions/metadata-attributes.js
/** Maps a `metadata-*` attribute name to a snake_case metadata key, or `null` when the name is not prefixed. */
function metadataKeyFromAttribute(attributeName) {
	if (!attributeName.startsWith("metadata-")) return null;
	return attributeName.slice(9).replace(/-/g, "_");
}
/** Collects string values from every `metadata-*` attribute on `element`. */
function readMetadataAttributes(element) {
	const metadata = {};
	for (const { name, value } of element.attributes) {
		const key = metadataKeyFromAttribute(name);
		if (key) metadata[key] = value;
	}
	return metadata;
}
/** Merges attribute and property metadata. Property values override attributes with the same key. */
function mergeExtensionMetadata(fromAttributes, fromProperty) {
	const hasAttributes = Object.keys(fromAttributes).length > 0;
	if (!hasAttributes && isUndefined(fromProperty)) return void 0;
	if (!hasAttributes) return fromProperty;
	if (isUndefined(fromProperty)) return { ...fromAttributes };
	return {
		...fromAttributes,
		...fromProperty
	};
}

//#endregion
//#region ../html/dist/dev/extensions/player-extension-element.js
/**
* Abstract base for elements that register a player extension (e.g. Mux Data, Google Cast) with the surrounding player.
*
* Place anywhere inside a player. The extension is registered when the element connects, follows the player's media
* (plain `<video>` / `<audio>` included) through the player itself, is released when this element disconnects, and is
* destroyed with this element.
*/
var PlayerExtensionElement = class extends UIElement {
	#extension = null;
	#register = null;
	#release = null;
	#metadataFromProperty;
	#metadataObserver = new MutationObserver((records) => {
		if (records.some((r) => r.attributeName?.startsWith("metadata-"))) this.#pushMetadataToExtension();
	});
	/** The player extension instance registered with the player. */
	get extension() {
		return this.#getExtension();
	}
	constructor() {
		super();
		new ContextConsumer(this, {
			context: extensionContext,
			subscribe: true,
			callback: (value) => this.#setRegister(value.registerExtension)
		});
	}
	connectedCallback() {
		super.connectedCallback();
		this.#metadataObserver.observe(this, { attributes: true });
		this.#pushMetadataToExtension();
	}
	disconnectedCallback() {
		this.#metadataObserver.disconnect();
		this.#setRegister(null);
		super.disconnectedCallback();
	}
	destroyCallback() {
		this.#setRegister(null);
		this.#extension?.destroy?.();
		super.destroyCallback();
	}
	/**
	* Merged metadata the element forwards to its extension: `metadata-*` attributes plus any `metadata` property, with
	* the property overriding attribute keys.
	*
	* @internal
	*/
	getResolvedMetadata() {
		return mergeExtensionMetadata(readMetadataAttributes(this), this.#metadataFromProperty);
	}
	/**
	* Stores programmatic `metadata` and refreshes the registered extension.
	*
	* @internal
	*/
	setMetadataFromProperty(value) {
		this.#metadataFromProperty = value;
		this.#pushMetadataToExtension();
	}
	#setRegister(register) {
		if (this.#register === register) return;
		this.#release?.();
		this.#release = null;
		this.#register = register;
		if (register) {
			this.#release = register(this.#getExtension());
			this.#pushMetadataToExtension();
		}
	}
	#getExtension() {
		if (this.#extension) return this.#extension;
		const extension = this.createExtension();
		this.#extension = extension;
		this.#pushMetadataToExtension();
		return extension;
	}
	#pushMetadataToExtension() {
		const extension = this.#extension;
		if (!extension || !("metadata" in extension)) return;
		const target = extension;
		const next = this.getResolvedMetadata();
		if (shallowEqual(next, target.metadata)) return;
		target.metadata = next;
	}
};

//#endregion
export { PlayerExtensionElement as t };
//# sourceMappingURL=player-extension-element-kaFdELF4.js.map