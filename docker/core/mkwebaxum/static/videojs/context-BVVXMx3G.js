import { n as ContextRequestEvent } from "./create-context-Dp8VhaeT.js";

//#region ../../node_modules/.pnpm/@lit+context@1.1.6/node_modules/@lit/context/development/lib/controllers/context-consumer.js
/**
* @license
* Copyright 2021 Google LLC
* SPDX-License-Identifier: BSD-3-Clause
*/
/**
* A ReactiveController which adds context consuming behavior to a custom
* element by dispatching `context-request` events.
*
* When the host element is connected to the document it will emit a
* `context-request` event with its context key. When the context request
* is satisfied the controller will invoke the callback, if present, and
* trigger a host update so it can respond to the new value.
*
* It will also call the dispose method given by the provider when the
* host element is disconnected.
*/
var ContextConsumer$1 = class {
	constructor(host, contextOrOptions, callback, subscribe) {
		this.subscribe = false;
		this.provided = false;
		this.value = void 0;
		this._callback = (value, unsubscribe) => {
			if (this.unsubscribe) {
				if (this.unsubscribe !== unsubscribe) {
					this.provided = false;
					this.unsubscribe();
				}
				if (!this.subscribe) this.unsubscribe();
			}
			this.value = value;
			this.host.requestUpdate();
			if (!this.provided || this.subscribe) {
				this.provided = true;
				if (this.callback) this.callback(value, unsubscribe);
			}
			this.unsubscribe = unsubscribe;
		};
		this.host = host;
		if (contextOrOptions.context !== void 0) {
			const options = contextOrOptions;
			this.context = options.context;
			this.callback = options.callback;
			this.subscribe = options.subscribe ?? false;
		} else {
			this.context = contextOrOptions;
			this.callback = callback;
			this.subscribe = subscribe ?? false;
		}
		this.host.addController(this);
	}
	hostConnected() {
		this.dispatchRequest();
	}
	hostDisconnected() {
		if (this.unsubscribe) {
			this.unsubscribe();
			this.unsubscribe = void 0;
		}
	}
	dispatchRequest() {
		this.host.dispatchEvent(new ContextRequestEvent(this.context, this.host, this._callback, this.subscribe));
	}
};

//#endregion
//#region ../element/dist/dev/context.js
const rootedDocuments = /* @__PURE__ */ new WeakSet();
/**
* Context consumer that also works when its provider is defined or connected after it.
*
* Before its first request it attaches one shared root to the host's document, which buffers subscribing requests that
* no provider answered and replays them when a matching provider connects. Custom elements can upgrade in any order, so
* a consumer cannot rely on its provider existing when it connects.
*
* @internal
*/
var ContextConsumer = class extends ContextConsumer$1 {
	hostConnected() {
		attachDocumentContextRoot(this.host.ownerDocument);
		super.hostConnected();
	}
};
/**
* Same buffering as Lit's `ContextRoot`, except replays skip consumers that left the document: a detached subtree can
* still hold its old provider, and a consumer that reconnects requests again on its own.
*/
function attachDocumentContextRoot(document) {
	if (rootedDocuments.has(document)) return;
	rootedDocuments.add(document);
	const pending = /* @__PURE__ */ new Map();
	document.addEventListener("context-request", (event) => {
		const request = event;
		if (request.subscribe !== true) return;
		const element = request.contextTarget ?? request.composedPath()[0];
		const requests = pending.get(request.context) ?? [];
		if (requests.some((entry) => entry.element.deref() === element && entry.callback.deref() === request.callback)) return;
		requests.push({
			element: new WeakRef(element),
			callback: new WeakRef(request.callback)
		});
		pending.set(request.context, requests);
	});
	document.addEventListener("context-provider", (event) => {
		const { context } = event;
		const requests = pending.get(context);
		if (!requests) return;
		pending.delete(context);
		for (const entry of requests) {
			const element = entry.element.deref();
			const callback = entry.callback.deref();
			if (element?.isConnected && callback) element.dispatchEvent(new ContextRequestEvent(context, element, callback, true));
		}
	});
}

//#endregion
export { ContextConsumer as t };
//# sourceMappingURL=context-BVVXMx3G.js.map