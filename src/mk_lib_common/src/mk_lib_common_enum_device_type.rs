#[non_exhaustive]
pub struct DLDeviceType;

// next number is 45
impl DLDeviceType {
    pub const AMPLIFIER: (&'static str, u16) = ("Audio Amplifier", 1);
    pub const BETA: (&'static str, u16) = ("Beta VCR", 2);
    pub const BLURAY: (&'static str, u16) = ("BluRay Player", 3);
    pub const BLURAY3D: (&'static str, u16) = ("BluRay Player 3D", 4);
    pub const BLURAYHD: (&'static str, u16) = ("BluRay Ultra HD Player", 5);
    pub const CABLE: (&'static str, u16) = ("Cable Box", 6);
    pub const CD: (&'static str, u16) = ("Compact Disk Player", 7);
    pub const DAC: (&'static str, u16) = ("Digital to Analog Converter", 8);
    pub const DAT: (&'static str, u16) = ("Digital Audio Tape", 9);
    pub const DECODER: (&'static str, u16) = ("Audio Decoder", 10);
    pub const DVD: (&'static str, u16) = ("DVD Player", 11);
    pub const DVD3D: (&'static str, u16) = ("DVD Player 3D", 12);
    pub const DVDHD: (&'static str, u16) = ("HD-DVD Player", 13);
    pub const DVR: (&'static str, u16) = ("Digital Video Recorder", 14);
    pub const EQUALIZER: (&'static str, u16) = ("Equalizer", 15);
    pub const GAMESYSTEM: (&'static str, u16) = ("Game System", 16);
    pub const LASERDISK: (&'static str, u16) = ("Laser Disc Player", 17);
    pub const LASERDISKHD: (&'static str, u16) = ("MUSE Laser Disc Player", 43);
    pub const MINIDISC: (&'static str, u16) = ("MiniDisc Player", 42);
    pub const MUSE: (&'static str, u16) = ("Hi Vision Muse Laserdisc Player", 44);
    pub const NETUSB: (&'static str, u16) = ("Net/USB", 18);
    pub const PHONOGRAPH: (&'static str, u16) = ("Phonograph Player", 19);
    pub const PREAMP: (&'static str, u16) = ("Audio Preamplifier", 20);
    pub const PREAMPAV: (&'static str, u16) = ("Audio/Video Preamplifier", 21);
    pub const PROJECTOR: (&'static str, u16) = ("Video Projector", 22);
    pub const PROJECTOR3D: (&'static str, u16) = ("Video Projector 3D", 23);
    pub const PROJECTORCRT: (&'static str, u16) = ("CRT Video Projector", 24);
    pub const RECEIVER: (&'static str, u16) = ("Audio/Video Receiver", 25);
    pub const REEL: (&'static str, u16) = ("Reel to Reel Tape Player", 26);
    pub const SACD: (&'static str, u16) = ("Super Audio Compact Disc Player", 27);
    pub const SATELLITE: (&'static str, u16) = ("Satellite Receiver", 28);
    pub const SCREEN: (&'static str, u16) = ("Front Projection Screen", 29);
    pub const SMARTBULB: (&'static str, u16) = ("Smart Bulb", 30);
    pub const SMARTSOCKET: (&'static str, u16) = ("Smart Socket", 31);
    pub const SPEAKER: (&'static str, u16) = ("Speaker", 32);
    pub const STREAM: (&'static str, u16) = ("Audio/Video Streaming Device", 33);
    pub const SUPERBETA: (&'static str, u16) = ("Super Beta VCR", 34);
    pub const SVHS: (&'static str, u16) = ("Super VHS VCR", 35);
    pub const TAPE: (&'static str, u16) = ("Tape Deck", 36);
    pub const TUNER: (&'static str, u16) = ("OTA Tuner", 37);
    pub const TV: (&'static str, u16) = ("Television", 38);
    pub const UPS: (&'static str, u16) = ("Uninterruptible Power Supply", 39);
    pub const VHS: (&'static str, u16) = ("VHS VCR", 40);
    pub const VIDEOPROCESSOR: (&'static str, u16) = ("Video Processor", 41);
}
