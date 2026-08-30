// AWS SES dedicated-IP warm-up.
//
// New/dedicated IPs need a gradual volume ramp so mailbox providers
// establish reputation. The schedule below follows AWS's published
// recommended warm-up plan.
//
// Reference: https://docs.aws.amazon.com/ses/latest/dg/dedicated-ip-warming.html

use lettre::{Message, Transport};
use std::time::Duration;
use tokio::time::sleep;

use crate::mk_lib_network_email_aws::mk_lib_network_email_aws_transport;

/// AWS-recommended daily send volume for a dedicated IP warm-up,
/// indexed by warm-up day (1-based). Day 1 is `WARMUP_PLAN[0]`.
pub const WARMUP_PLAN: &[u64] = &[
    50, 100, 500, 1_000, 5_000, 10_000, 20_000, 40_000, 70_000, 100_000, 150_000, 200_000, 300_000,
    400_000, 500_000, 750_000, 1_000_000, 1_250_000, 1_500_000, 2_000_000, 2_500_000, 3_000_000,
    3_500_000, 4_000_000, 4_500_000, 5_000_000, 6_000_000, 7_000_000, 8_000_000, 9_000_000,
    10_000_000, 11_000_000, 12_000_000, 13_000_000, 14_000_000, 15_000_000, 16_000_000, 17_000_000,
    18_000_000, 19_000_000, 20_000_000, 22_500_000, 25_000_000, 27_500_000, 30_000_000,
];

/// Daily target volume for the given 1-based warm-up day.
/// Days past the schedule return the final day's value (full reputation).
pub fn mk_lib_network_email_aws_warmup_daily_target(warmup_day: u32) -> u64 {
    if warmup_day == 0 {
        return 0;
    }
    let idx = (warmup_day as usize).saturating_sub(1);
    if idx >= WARMUP_PLAN.len() {
        *WARMUP_PLAN.last().unwrap_or(&0)
    } else {
        WARMUP_PLAN[idx]
    }
}

/// One recipient + its per-recipient message content.
#[derive(Debug, Clone)]
pub struct WarmupMessage {
    pub to: String,
    pub subject: String,
    pub body: String,
}

/// Input for a single warm-up batch run.
pub struct WarmupConfig {
    pub aws_region: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,

    /// Verified SES sender (e.g. `"Sender <sender@example.com>"`).
    pub email_from: String,
    pub email_reply_to: String,

    /// 1-based warm-up day. Used to cap the number of sends.
    pub warmup_day: u32,

    /// SES account max send rate (messages/second). Used for pacing.
    /// Must be > 0.
    pub max_per_second: u32,

    /// Messages to attempt. Will be truncated to the daily target.
    pub messages: Vec<WarmupMessage>,
}

/// Max number of per-message error strings retained on a report.
/// Daily targets reach tens of millions, so a bulk failure (outage,
/// auth error) could otherwise allocate a string per failed send.
/// `failed` still counts every failure; `errors` is a bounded sample.
pub const WARMUP_MAX_ERROR_SAMPLES: usize = 100;

/// Result of a warm-up batch run.
#[derive(Debug, Default, Clone)]
pub struct WarmupReport {
    pub attempted: u64,
    pub sent: u64,
    pub failed: u64,
    pub daily_target: u64,
    /// Sample of per-message error messages, capped at
    /// `WARMUP_MAX_ERROR_SAMPLES`. Use `failed` for the total count.
    pub errors: Vec<String>,
}

impl WarmupReport {
    fn record_failure(&mut self, msg: String) {
        self.failed += 1;
        if self.errors.len() < WARMUP_MAX_ERROR_SAMPLES {
            self.errors.push(msg);
        }
    }
}

/// Execute a warm-up batch: sends up to the day's daily target, paced to
/// `max_per_second`. Sends are blocking under the hood and run via
/// `spawn_blocking` so the async runtime is not blocked.
pub async fn mk_lib_network_email_aws_warmup_run(
    cfg: WarmupConfig,
) -> Result<WarmupReport, Box<dyn std::error::Error + Send + Sync>> {
    if cfg.max_per_second == 0 {
        return Err("max_per_second must be > 0".into());
    }

    let daily_target = mk_lib_network_email_aws_warmup_daily_target(cfg.warmup_day);
    let mut report = WarmupReport {
        daily_target,
        ..Default::default()
    };

    let from_mbox = cfg
        .email_from
        .parse()
        .map_err(|e| format!("invalid From {:?}: {e}", cfg.email_from))?;
    let reply_mbox = cfg
        .email_reply_to
        .parse()
        .map_err(|e| format!("invalid Reply-To {:?}: {e}", cfg.email_reply_to))?;

    let mailer = mk_lib_network_email_aws_transport(
        &cfg.aws_region,
        &cfg.aws_access_key_id,
        &cfg.aws_secret_access_key,
    )
    .map_err(|e| format!("smtp transport init failed: {e}"))?;

    let interval = Duration::from_secs_f64(1.0 / cfg.max_per_second as f64);
    let cap = daily_target.min(cfg.messages.len() as u64);

    for msg in cfg.messages.into_iter().take(cap as usize) {
        report.attempted += 1;

        let built = Message::builder()
            .from(from_mbox.clone())
            .reply_to(reply_mbox.clone())
            .to(match msg.to.parse() {
                Ok(t) => t,
                Err(e) => {
                    report.record_failure(format!("invalid To {:?}: {e}", msg.to));
                    sleep(interval).await;
                    continue;
                }
            })
            .subject(msg.subject)
            .body(msg.body);

        let email = match built {
            Ok(m) => m,
            Err(e) => {
                report.record_failure(format!("message build failed: {e}"));
                sleep(interval).await;
                continue;
            }
        };

        let mailer_clone = mailer.clone();
        let send_res = tokio::task::spawn_blocking(move || mailer_clone.send(&email)).await;

        match send_res {
            Ok(Ok(_)) => report.sent += 1,
            Ok(Err(e)) => report.record_failure(format!("send failed: {e}")),
            Err(e) => report.record_failure(format!("send task join failed: {e}")),
        }

        sleep(interval).await;
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn daily_target_zero_day_is_zero() {
        assert_eq!(mk_lib_network_email_aws_warmup_daily_target(0), 0);
    }

    #[test]
    fn daily_target_day_one() {
        assert_eq!(mk_lib_network_email_aws_warmup_daily_target(1), 50);
    }

    #[test]
    fn daily_target_past_end_clamps_to_final_day() {
        let last = *WARMUP_PLAN.last().unwrap();
        assert_eq!(mk_lib_network_email_aws_warmup_daily_target(9_999), last);
    }

    #[test]
    fn daily_target_is_monotonic() {
        for w in WARMUP_PLAN.windows(2) {
            assert!(w[1] >= w[0], "warm-up plan must be non-decreasing");
        }
    }

    #[test]
    fn record_failure_caps_error_samples() {
        let mut report = WarmupReport::default();
        for i in 0..(WARMUP_MAX_ERROR_SAMPLES + 50) {
            report.record_failure(format!("err {i}"));
        }
        assert_eq!(report.failed, (WARMUP_MAX_ERROR_SAMPLES + 50) as u64);
        assert_eq!(report.errors.len(), WARMUP_MAX_ERROR_SAMPLES);
    }
}
