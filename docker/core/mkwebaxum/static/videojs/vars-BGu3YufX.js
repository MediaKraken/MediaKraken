import { d as isString } from "./predicate-3rF1m2uv.js";

//#region ../core/dist/dev/dom/utils/layout.js
/** @internal */
function forceLayout(element) {
	element?.getBoundingClientRect();
}
/** @internal */
function createDOMRect(left, top, width, height) {
	const right = left + width;
	const bottom = top + height;
	return {
		x: left,
		y: top,
		width,
		height,
		top,
		right,
		bottom,
		left,
		toJSON() {
			return {
				x: left,
				y: top,
				width,
				height,
				top,
				right,
				bottom,
				left
			};
		}
	};
}
/** @internal */
function intersectDOMRects(firstRect, secondRect) {
	const left = Math.max(firstRect.left, secondRect.left);
	const top = Math.max(firstRect.top, secondRect.top);
	const right = Math.min(firstRect.right, secondRect.right);
	const bottom = Math.min(firstRect.bottom, secondRect.bottom);
	return createDOMRect(left, top, Math.max(0, right - left), Math.max(0, bottom - top));
}
/** @internal */
function getPositioningBoundaryRect(boundaryElement) {
	const viewportRect = document.documentElement.getBoundingClientRect();
	return boundaryElement ? intersectDOMRects(viewportRect, boundaryElement.getBoundingClientRect()) : viewportRect;
}
/** @internal */
function resolvePositioningBoundary(boundary, options = {}) {
	if (!boundary) return null;
	if (!isString(boundary)) return boundary;
	if (boundary === "viewport") return null;
	if (boundary === "container") return options.container ?? null;
	try {
		return (options.root ?? document).querySelector(boundary);
	} catch {
		return null;
	}
}

//#endregion
//#region ../core/dist/dev/core/ui/popover/vars.js
/** @internal */
const PopoverCSSVars = {
	/** Distance between the popup and the trigger along the side axis. */
	sideOffset: "--media-popover-side-offset",
	/** Distance between the popup and the trigger along the alignment axis. */
	alignOffset: "--media-popover-align-offset",
	/** Minimum distance between the popup and the positioning boundary. */
	boundaryOffset: "--media-popover-boundary-offset",
	/** The anchor element's width. */
	anchorWidth: "--media-popover-anchor-width",
	/** The anchor element's height. */
	anchorHeight: "--media-popover-anchor-height",
	/** Available width between the trigger and the boundary edge. */
	availableWidth: "--media-popover-available-width",
	/** Available height between the trigger and the boundary edge. */
	availableHeight: "--media-popover-available-height"
};

//#endregion
export { resolvePositioningBoundary as a, getPositioningBoundaryRect as i, createDOMRect as n, forceLayout as r, PopoverCSSVars as t };
//# sourceMappingURL=vars-BGu3YufX.js.map