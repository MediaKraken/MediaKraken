import { i as walkAncestors } from "./ui-element-Ctm6zaRJ.js";

//#region ../utils/dist/dom/focus.js
const TABBABLE_SELECTOR = [
	"a[href]",
	"button:not([disabled])",
	"input:not([disabled])",
	"select:not([disabled])",
	"textarea:not([disabled])",
	"audio[controls]",
	"video[controls]",
	"iframe",
	"[contenteditable]:not([contenteditable=\"false\"])",
	"[tabindex]:not([tabindex=\"-1\"])"
].join(",");
/** @internal */
function getDeepActiveElement(root = document) {
	let active = root.activeElement;
	while (active?.shadowRoot?.activeElement) active = active.shadowRoot.activeElement;
	return active;
}
/**
* Returns the elements in a composed subtree that participate in sequential keyboard navigation.
*
* @internal
*/
function getTabbableElements(root) {
	const tabbable = [];
	const visited = /* @__PURE__ */ new Set();
	visitChildren(root);
	return tabbable;
	function visitChildren(parent) {
		for (const child of parent.children) visitElement(child);
	}
	function visitElement(element) {
		if (visited.has(element)) return;
		visited.add(element);
		if (element instanceof HTMLElement && isTabbableElement(element)) tabbable.push(element);
		if (element instanceof HTMLSlotElement) {
			const assigned = element.assignedElements({ flatten: true });
			if (assigned.length > 0) for (const child of assigned) visitElement(child);
			else visitChildren(element);
			return;
		}
		if (element.shadowRoot) visitChildren(element.shadowRoot);
		else visitChildren(element);
	}
}
function isTabbableElement(element) {
	if (!element.matches(TABBABLE_SELECTOR) || element.tabIndex < 0 || element.matches(":disabled")) return false;
	return !walkAncestors(element, (ancestor) => {
		if (ancestor instanceof HTMLElement && (ancestor.hidden || ancestor.hasAttribute("inert") || ancestor.getAttribute("aria-hidden") === "true")) return true;
	}, { composed: true });
}

//#endregion
export { getTabbableElements as n, getDeepActiveElement as t };
//# sourceMappingURL=focus-YOJlajXF.js.map