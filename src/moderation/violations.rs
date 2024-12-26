use serenity::all::UserId;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub enum ModAction {
    None,
    Mute(std::time::Duration),
    Ban,
}

pub struct ViolationThresholds {
    short_mute_min: u32,
    long_mute_min: u32,
    ban_min: u32,
    short_mute_duration: std::time::Duration,
    long_mute_duration: std::time::Duration,
}
impl Default for ViolationThresholds {
    fn default() -> Self {
        Self {
            short_mute_min: 2,
            long_mute_min: 5,
            ban_min: 7,
            short_mute_duration: std::time::Duration::from_secs(60),
            long_mute_duration: std::time::Duration::from_secs(3600),
        }
    }
}

struct Violations {
    violations: u32,
}

pub struct ViolationsTracker {
    violations: Arc<Mutex<HashMap<UserId, Violations>>>,
}

impl ViolationsTracker {
    pub fn new() -> Self {
        Self {
            violations: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn increment_violations(&self, user_id: UserId) -> Result<(), Box<dyn std::error::Error>> {
        self.violations
            .lock()
            .map_err(|_| "Mutex poisoned")?
            .entry(user_id)
            .and_modify(|v| v.violations += 1)
            .or_insert(Violations { violations: 1 });
        Ok(())
    }

    pub fn get_violation_count(&self, user_id: UserId) -> Option<u32> {
        self.violations
            .lock()
            .ok()?
            .get(&user_id)
            .map_or(Some(0), |v| Some(v.violations))
    }
    pub fn get_appropriate_action(
        &self,
        user_id: UserId,
        thresholds: &ViolationThresholds,
    ) -> Option<ModAction> {
        let violations = self.get_violation_count(user_id)?;

        Some(match violations {
            v if v < thresholds.short_mute_min => ModAction::None,
            v if v < thresholds.long_mute_min => ModAction::Mute(thresholds.short_mute_duration),
            v if v < thresholds.ban_min => ModAction::Mute(thresholds.long_mute_duration),
            _ => ModAction::Ban,
        })
    }
}
