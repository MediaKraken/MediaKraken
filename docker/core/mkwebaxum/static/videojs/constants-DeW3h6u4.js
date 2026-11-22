//#region ../media/dist/dev/core/constants.js
/**
* A frozen, empty `TimeRanges`-like value for hosts that report ranges but have none yet, such as an embed before it
* loads. A host that never reports ranges returns `undefined` instead.
*
* @internal
*/
const NO_TIME_RANGES_YET = Object.freeze({
	length: 0,
	start: () => 0,
	end: () => 0
});

//#endregion
export { NO_TIME_RANGES_YET as t };
//# sourceMappingURL=constants-DeW3h6u4.js.map