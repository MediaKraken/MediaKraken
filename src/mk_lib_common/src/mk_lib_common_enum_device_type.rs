#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#![allow(unused)]

#[non_exhaustive]
pub struct DLDeviceType;

// next number is 44
impl DLDeviceType {
    pub const AMPLIFIER: [&'static str; 2] = ["Audio Amplifier", 1];
    pub const BETA: [&'static str; 2] = ["Beta VCR", 2];
    pub const BLURAY: [&'static str; 2] = ["BluRay Player", 3];
    pub const BLURAY3D: [&'static str; 2] = ["BluRay Player 3D", 4];
    pub const BLURAYHD: [&'static str; 2] = ["BluRay Ultra HD Player", 5];
    pub const CABLE: [&'static str; 2] = ["Cable Box", 6];
    pub const CD: [&'static str; 2] = ["Compact Disk Player", 7];
    pub const DAC: [&'static str; 2] = ["Digital to Analog Converter", 8];
    pub const DAT: [&'static str; 2] = ["Digital Audio Tape", 9];
    pub const DECODER: [&'static str; 2] = ["Audio Decoder", 10];
    pub const DVD: [&'static str; 2] = ["DVD Player", 11];
    pub const DVD3D: [&'static str; 2] = ["DVD Player 3D", 12];
    pub const DVDHD: [&'static str; 2] = ["HD-DVD Player", 13];
    pub const DVR: [&'static str; 2] = ["Digital Video Recorder", 14];
    pub const EQUALIZER: [&'static str; 2] = ["Equalizer", 15];
    pub const GAMESYSTEM: [&'static str; 2] = ["Game System", 16];
    pub const LASERDISK: [&'static str; 2] = ["Laser Disc Player", 17];
    pub const LASERDISKHD: [&'static str; 2] = ["MUSE Laser Disc Player", 43];
    pub const MINIDISC: [&'static str; 2] = ["MiniDisc Player", 42];
    pub const NETUSB: [&'static str; 2] = ["Net/USB", 18];
    pub const PHONOGRAPH: [&'static str; 2] = ["Phonograph Player", 19];
    pub const PREAMP: [&'static str; 2] = ["Audio Preamplifier", 20];
    pub const PREAMPAV: [&'static str; 2] = ["Audio/Video Preamplifier", 21];
    pub const PROJECTOR: [&'static str; 2] = ["Video Projector", 22];
    pub const PROJECTOR3D: [&'static str; 2] = ["Video Projector 3D", 23];
    pub const PROJECTORCRT: [&'static str; 2] = ["CRT Video Projector", 24];
    pub const RECEIVER: [&'static str; 2] = ["Audio/Video Receiver", 25];
    pub const REEL: [&'static str; 2] = ["Reel to Reel Tape Player", 26];
    pub const SACD: [&'static str; 2] = ["Super Audio Compact Disc Player", 27];
    pub const SATELLITE: [&'static str; 2] = ["Satellite Receiver", 28];
    pub const SCREEN: [&'static str; 2] = ["Front Projection Screen", 29];
    pub const SMARTBULB: [&'static str; 2] = ["Smart Bulb", 30];
    pub const SMARTSOCKET: [&'static str; 2] = ["Smart Socket", 31];
    pub const SPEAKER: [&'static str; 2] = ["Speaker", 32];
    pub const STREAM: [&'static str; 2] = ["Audio/Video Streaming Device", 33];
    pub const SUPERBETA: [&'static str; 2] = ["Super Beta VCR", 34];
    pub const SVHS: [&'static str; 2] = ["Super VHS VCR", 35];
    pub const TAPE: [&'static str; 2] = ["Tape Deck", 36];
    pub const TUNER: [&'static str; 2] = ["OTA Tuner", 37];
    pub const TV: [&'static str; 2] = ["Television", 38];
    pub const UPS: [&'static str; 2] = ["Uninterruptible Power Supply", 39];
    pub const VHS: [&'static str; 2] = ["VHS VCR", 40];
    pub const VIDEOPROCESSOR: [&'static str; 2] = ["Video Processor", 41];
}