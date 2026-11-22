import { t as safeDefine } from "./safe-define-THEasJ9B.js";
import { i as liveAudioFeatures, t as createPlayer } from "./create-player-iNhj1wGG.js";

//#region ../html/dist/dev/presets/live-audio/player.js
const { PlayerElement, PlayerController: LiveAudioPlayerController } = createPlayer({ features: liveAudioFeatures });
var LiveAudioPlayerElement = class extends PlayerElement {
	static {
		this.tagName = "live-audio-player";
	}
};

//#endregion
//#region ../html/dist/dev/define/live-audio/player.js
safeDefine(LiveAudioPlayerElement);

//#endregion
//# sourceMappingURL=player-hzNou9D3.js.map