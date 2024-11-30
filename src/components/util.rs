use crate::StepTrace;
use serde_with::chrono::{DateTime, TimeDelta, Utc};

pub fn get_duration_string(delta: TimeDelta) -> String {
    let total_seconds = delta.to_std().unwrap().as_secs();
    let hours = (total_seconds / 3600) as u64;
    let minutes = ((total_seconds % 3600) / 60) as u64;
    let seconds = (total_seconds % 60) as u64;

    let mut result = String::new();

    if hours > 0 {
        result.push_str(&format!(
            "{} Hour{}",
            hours,
            if hours > 1 { "s" } else { "" }
        ));
    }

    if minutes > 0 {
        if !result.is_empty() {
            result.push_str(" and ");
        }
        result.push_str(&format!(
            "{} Minute{}",
            minutes,
            if minutes > 1 { "s" } else { "" }
        ));
    }

    if seconds > 0 && result.is_empty() {
        result.push_str(&format!(
            "{} Second{}",
            seconds,
            if seconds > 1 { "s" } else { "" }
        ));
    }

    if result.is_empty() {
        result.push_str("0 Seconds"); // Handle zero duration case
    }

    result
}

// Based on all steps, tries to find the max completion time
pub(crate) fn find_max_completion(
    steps: &Vec<StepTrace>,
    durable_scheduled_at: DateTime<Utc>,
    durable_completed_at: Option<DateTime<Utc>>,
) -> DateTime<Utc> {
    let max_completion = steps
        .iter()
        .map(|step| step.completed_at)
        .reduce(|a, b| a.max(b))
        .unwrap_or_default()
        .max(durable_completed_at)
        .or_else(||durable_scheduled_at.checked_add_signed(TimeDelta::seconds(15)));
    max_completion.unwrap()
}
