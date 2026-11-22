//#region ../utils/dist/function/public-promise.js
/**
* A promise that can be settled from outside its executor.
*
* Useful as a barrier that one piece of code waits on and another settles — for example an in-flight load that callers
* await until whichever event finishes it resolves.
*
* @example
*   ```ts
*   const ready = createPublicPromise<void>();
*   element.addEventListener('load', () => ready.resolve(), { once: true });
*   await ready;
*   ```;
*
* @internal
*/
function createPublicPromise() {
	let resolve;
	let reject;
	const promise = new Promise((res, rej) => {
		resolve = res;
		reject = rej;
	});
	promise.resolve = resolve;
	promise.reject = reject;
	return promise;
}

//#endregion
export { createPublicPromise as t };
//# sourceMappingURL=public-promise-OBz_YnUq.js.map