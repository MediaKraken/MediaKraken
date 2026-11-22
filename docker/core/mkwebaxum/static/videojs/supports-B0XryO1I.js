//#region ../utils/dist/dom/supports.js
/** @internal */
function supportsAnchorPositioning() {
	return typeof CSS !== "undefined" && CSS.supports("anchor-name: --a");
}
/** @internal */
function supportsPopoverAPI() {
	return typeof HTMLElement !== "undefined" && "popover" in HTMLElement.prototype;
}
/**
* How the browser plays HLS without an engine:
*
* - `'preferred'`: natively by default, because there's no MSE for an engine to play through, as on iOS before 17.1.
* - `'supported'`: it can, but an MSE engine plays by default, with more features and the same behavior everywhere.
*   That's every browser with MSE, Safari included.
* - `'unsupported'`: it can't, which is also the answer without a DOM.
*
* @internal
*/
function nativeHlsSupport() {
	if (typeof document === "undefined" || typeof navigator === "undefined") return "unsupported";
	if (!document.createElement("video").canPlayType("application/vnd.apple.mpegurl")) return "unsupported";
	return supportsMediaSource() ? "supported" : "preferred";
}
/**
* Whether an MSE engine can run: `MediaSource`, or `ManagedMediaSource` on iOS 17.1+.
*
* @internal
*/
function supportsMediaSource() {
	const scope = globalThis;
	return !!(scope.MediaSource ?? scope.ManagedMediaSource ?? scope.WebKitMediaSource);
}
/**
* Whether `new CSSStyleSheet()` works. Safari exposed the interface long before 16.4 made it constructable, and
* constructing it there throws `TypeError: Illegal constructor`, so checking for the interface is not enough.
*
* @internal
*/
function supportsConstructableStyleSheets() {
	if (typeof globalThis.CSSStyleSheet === "undefined") return false;
	try {
		new globalThis.CSSStyleSheet();
		return true;
	} catch {
		return false;
	}
}

//#endregion
export { supportsPopoverAPI as i, supportsAnchorPositioning as n, supportsConstructableStyleSheets as r, nativeHlsSupport as t };
//# sourceMappingURL=supports-B0XryO1I.js.map