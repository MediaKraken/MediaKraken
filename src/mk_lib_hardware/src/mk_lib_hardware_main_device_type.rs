pub const DEVICE_ITEM_TYPES: [&str; 39] = [
    "Amplifier",
    "Blu-ray Player",
    "Blu-ray Ultra HD Player",
    "Cable Box",
    "CD Player",
    "Chromecast",
    "Chromecast Ultra",
    "DAC",
    "DAT",
    "DVD Player",
    "DVR",
    "Game System",
    "HD-DVD Player",
    "HDHomeRun",
    "Internet Radio",
    "LaserDisc",
    "MiniDisc",
    "MUSE Laserdisc",
    "OTA Tuner",
    "PC",
    "Preamplifier",
    "Projector",
    "Radio",
    "Receiver",
    "Roku",
    "SACD",
    "Satellite Receiver",
    "Screen",
    "Tape Deck",
    "Television",
    "Tuner",
    "Turntable",
    "VCR - SVHS",
    "VCR - VHS",
    "VCR - Beta",
    "VCR - Super Beta",
    "Video Processor",
    "Video Switcher",
];

#[cfg(test)]
mod tests {
    use super::DEVICE_ITEM_TYPES;

    #[test]
    fn device_item_types_has_39_entries() {
        assert_eq!(DEVICE_ITEM_TYPES.len(), 39);
    }

    #[test]
    fn device_item_types_no_duplicates() {
        let mut sorted = DEVICE_ITEM_TYPES.to_vec();
        sorted.sort();
        for i in 1..sorted.len() {
            assert_ne!(sorted[i - 1], sorted[i]);
        }
    }

    #[test]
    fn device_item_types_no_empty_strings() {
        for item in &DEVICE_ITEM_TYPES {
            assert!(!item.is_empty(), "Found empty string at index {}", item);
        }
    }

    #[test]
    fn device_item_types_contains_expected_entries() {
        assert!(DEVICE_ITEM_TYPES.contains(&"Chromecast"));
        assert!(DEVICE_ITEM_TYPES.contains(&"HDHomeRun"));
        assert!(DEVICE_ITEM_TYPES.contains(&"Roku"));
        assert!(DEVICE_ITEM_TYPES.contains(&"Television"));
    }
}