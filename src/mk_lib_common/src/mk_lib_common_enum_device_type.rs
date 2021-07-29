#![allow(unused)]

#[non_exhaustive]
pub struct DLDeviceType;

impl DLDeviceType {
    pub const LASERDISK: [&'static str; 2] = ["Laser Disc Player", 1];
    pub const VHS: [&'static str; 2] = ["VHS VCR", 1];
    pub const SVHS: [&'static str; 2] = ["Super VHS VCR", 1];
    pub const BETA: [&'static str; 2] = ["Beta VCR", 1];
    pub const SUPERBETA: [&'static str; 2] = ["Super Beta VCR", 1];
    pub const TV: [&'static str; 2] = ["Television", 1];
    pub const PROJECTOR: [&'static str; 2] = ["Video Projector", 1];
    pub const PROJECTORCRT: [&'static str; 2] = ["CRT Video Projector", 1];
    pub const BLURAY3D: [&'static str; 2] = ["BluRay Player 3D", 1];
    pub const DVD: [&'static str; 2] = ["DVD Player", 1];
    pub const DVDHD: [&'static str; 2] = ["HD-DVD Player", 1];
    pub const DVR: [&'static str; 2] = ["Digital Video Recorder", 1];
    pub const BLURAY: [&'static str; 2] = ["BluRay Player", 1];
    pub const BLURAYHD: [&'static str; 2] = ["BluRay Ultra HD Player", 1];
    pub const DVD3D: [&'static str; 2] = ["DVD Player 3D", 1];
    pub const PROJECTOR3D: [&'static str; 2] = 206;
    pub const PHONOGRAPH: [&'static str; 2] = ["Phonograph Player", 1];
    pub const DAC: [&'static str; 2] = ["Digital to Analog Converter", 1];
    pub const TAPE: [&'static str; 2] = ["Tape Deck", 1];
    pub const CD: [&'static str; 2] = ["Compact Disk Player", 1];
    pub const UPS: [&'static str; 2] = ["Uninterruptible Power Supply", 1];
    pub const SACD: [&'static str; 2] = ["Super Audio Compact Disc Player", 1];
    pub const REEL: [&'static str; 2] = ["Reel to Reel Tape Player", 1];
    pub const RECEIVER: [&'static str; 2] = ["Audio/Video Receiver", 1];
    pub const AMPLIFIER: [&'static str; 2] = ["Audio Amplifier", 1];
    pub const DECODER: [&'static str; 2] = ["Audio Decoder", 1];
    pub const DAT: [&'static str; 2] = ["Digital Audio Tape", 1];
    pub const CABLE: [&'static str; 2] = ["Cable Box", 1];
    pub const SATELLITE: [&'static str; 2] = ["Satellite Receiver", 1];
    pub const NETUSB: [&'static str; 2] = ["Net/USB", 1];
    pub const VIDEOPROCESSOR: [&'static str; 2] = ["Video Processor", 1];
    pub const STREAM: [&'static str; 2] = ["Audio/Video Streaming Device", 1];
    pub const GAMESYSTEM: [&'static str; 2] = ["Game System", 1];
    pub const PREAMP: [&'static str; 2] = ["Audio Preamplifier", 1];
    pub const PREAMPAV: [&'static str; 2] = ["Audio/Video Preamplifier", 1];
    pub const TUNER: [&'static str; 2] = ["OTA Tuner", 1];
    pub const SCREEN: [&'static str; 2] = ["Front Projection Screen", 1];
    pub const EQUALIZER: [&'static str; 2] = ["Equalizer", 1];
    pub const SMARTBULB: [&'static str; 2] = ["Smart Bulb", 1];
    pub const SMARTSOCKET: [&'static str; 2] = ["Smart Socket", 1];
    pub const SPEAKER: [&'static str; 2] = ["Speaker", 1];
}