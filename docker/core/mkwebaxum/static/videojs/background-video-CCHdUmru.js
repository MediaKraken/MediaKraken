import { t as safeDefine } from "./safe-define-THEasJ9B.js";
import { t as BackgroundVideoBase } from "./base-Dz46Xt-q.js";

//#region ../html/dist/dev/media/background-video/adapter.js
var BackgroundVideo = class extends BackgroundVideoBase {
	constructor() {
		super();
		this.target?.addEventListener("error", () => this.dispatchEvent(new Event("error")));
	}
	getMediaTarget() {
		return this.target;
	}
	/** URL of a video the browser can play directly. Assigning a new one restarts playback. */
	get src() {
		return this.target?.src ?? "";
	}
	set src(value) {
		const target = this.target;
		if (target) target.src = value;
	}
	/** The inner video's `MediaError`, or `null`. */
	get error() {
		return this.target?.error ?? null;
	}
};

//#endregion
//#region ../html/dist/dev/define/media/background-video.js
var BackgroundVideoElement = class extends BackgroundVideo {
	static {
		this.tagName = "background-video";
	}
};
safeDefine(BackgroundVideoElement);

//#endregion
export { BackgroundVideoElement as t };
//# sourceMappingURL=background-video-CCHdUmru.js.map