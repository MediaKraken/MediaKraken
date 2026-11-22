import { t as CustomMediaElement } from "./custom-media-element-LyJK1FLV.js";
import { t as safeDefine } from "./safe-define-THEasJ9B.js";
import { t as MediaAttachMixin } from "./media-attach-mixin-yt2XYok0.js";
import { t as MuxAudioMixin } from "./mixin-CFhFTi2U.js";
import { t as MuxVideoAdapter } from "./adapter-B8seKS7H.js";

//#region ../adapters/mux-audio/dist/dev/index.js
var MuxAudioAdapter = class extends MuxVideoAdapter {};

//#endregion
//#region ../html/dist/dev/media/mux-audio/hls-js.js
const MuxAudioBase = MuxAudioMixin(MediaAttachMixin(CustomMediaElement("audio", MuxAudioAdapter)));
var MuxAudio = class extends MuxAudioBase {};

//#endregion
//#region ../html/dist/dev/define/media/mux-audio/hls-js.js
var MuxAudioElement = class extends MuxAudio {
	static {
		this.tagName = "mux-audio";
	}
};
safeDefine(MuxAudioElement);

//#endregion
export { MuxAudioElement as t };
//# sourceMappingURL=hls-js-DESwrPCY.js.map