//#region ../utils/dist/dom/popover.js
const ZERO_OFFSETS = {
	sideOffset: 0,
	boundaryOffset: 0
};
const OPPOSITE_SIDE = {
	top: "bottom",
	bottom: "top",
	left: "right",
	right: "left"
};
function getSideAvailable(triggerRect, boundaryRect, side, offsets) {
	const boundaryOffset = offsets.boundaryOffset ?? 0;
	switch (side) {
		case "top": return triggerRect.top - boundaryRect.top - boundaryOffset - offsets.sideOffset;
		case "bottom": return boundaryRect.bottom - triggerRect.bottom - boundaryOffset - offsets.sideOffset;
		case "left": return triggerRect.left - boundaryRect.left - boundaryOffset - offsets.sideOffset;
		case "right": return boundaryRect.right - triggerRect.right - boundaryOffset - offsets.sideOffset;
	}
}
/**
* Resolve the preferred side against a positioning boundary.
*
* @internal
*/
function getPositionedSide(triggerRect, positionedRect, boundaryRect, opts, offsets = ZERO_OFFSETS) {
	const preferred = opts.side;
	const opposite = OPPOSITE_SIDE[preferred];
	const size = preferred === "top" || preferred === "bottom" ? positionedRect.height : positionedRect.width;
	const preferredSpace = getSideAvailable(triggerRect, boundaryRect, preferred, offsets);
	if (preferredSpace >= size) return preferred;
	return getSideAvailable(triggerRect, boundaryRect, opposite, offsets) > preferredSpace ? opposite : preferred;
}
/** @internal */
function tryShowPopover(el) {
	try {
		el?.showPopover?.();
	} catch {}
}
/** @internal */
function tryHidePopover(el) {
	try {
		el?.hidePopover?.();
	} catch {}
}
/**
* Hides every open popover inside `root`, including those in its own and nested shadow roots.
*
* @internal
*/
function hideOpenPopovers(root) {
	if (root instanceof Element && root.shadowRoot) hideOpenPopovers(root.shadowRoot);
	for (const el of root.querySelectorAll("*")) {
		if (isOpenPopover(el)) tryHidePopover(el);
		if (el.shadowRoot) hideOpenPopovers(el.shadowRoot);
	}
}
function isOpenPopover(el) {
	try {
		return el.matches(":popover-open");
	} catch {
		return false;
	}
}

//#endregion
export { tryShowPopover as i, hideOpenPopovers as n, tryHidePopover as r, getPositionedSide as t };
//# sourceMappingURL=popover-CL3vBHX-.js.map