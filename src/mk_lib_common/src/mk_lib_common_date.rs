use chrono::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn system_time_to_date_time(t: SystemTime) -> DateTime<Utc> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => {
            // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        }
    };
    // timestamp_opt can return None for out-of-range values; fall back to Unix epoch
    Utc.timestamp_opt(sec, nsec).single().unwrap_or(DateTime::<Utc>::UNIX_EPOCH)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_system_time_to_date_time_unix_epoch() {
        let result = system_time_to_date_time(UNIX_EPOCH);
        assert_eq!(result, DateTime::<Utc>::UNIX_EPOCH);
    }

    #[test]
    fn test_system_time_to_date_time_current() {
        let now = SystemTime::now();
        let result = system_time_to_date_time(now);
        let current = Utc::now();
        let diff = (current - result).abs();
        assert!(diff < Duration::from_secs(2));
    }

    #[test]
    fn test_system_time_to_date_time_future() {
        let future = SystemTime::now() + Duration::from_secs(3600);
        let result = system_time_to_date_time(future);
        let expected = Utc::now() + Duration::from_secs(3600);
        let diff = (expected - result).abs();
        assert!(diff < Duration::from_secs(2));
    }

    #[test]
    fn test_system_time_to_date_time_past() {
        let past = UNIX_EPOCH - Duration::from_secs(3600);
        let result = system_time_to_date_time(past);
        let expected = DateTime::<Utc>::UNIX_EPOCH - Duration::from_secs(3600);
        let diff = (expected - result).abs();
        assert!(diff < Duration::from_secs(1));
    }

    #[test]
    fn test_system_time_to_date_time_zero_nanos() {
        let t = UNIX_EPOCH + Duration::from_secs(1234567890);
        let result = system_time_to_date_time(t);
        assert_eq!(result.timestamp(), 1234567890);
        assert_eq!(result.timestamp_nanos_opt().unwrap() % 1_000_000_000, 0);
    }
}
