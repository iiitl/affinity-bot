use serenity::model::id::UserId;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct MessageTracker {
    pub first_message: Instant,
    pub message_count: u32,
}

#[derive(Clone)]
pub struct SpamChecker {
    message_tracker: Arc<Mutex<HashMap<UserId, MessageTracker>>>,
}

impl SpamChecker {
    pub fn new() -> Self {
        Self {
            message_tracker: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn is_spam(&self, user_id: UserId) -> bool {
        let mut tracker = self.message_tracker.lock().await;
        let now = Instant::now();

        if let Some(user_tracker) = tracker.get_mut(&user_id) {
            if now.duration_since(user_tracker.first_message) > Duration::from_secs(5) {
                user_tracker.first_message = now;
                user_tracker.message_count = 1;
                return false;
            }

            user_tracker.message_count += 1;
            user_tracker.message_count > 5
        } else {
            tracker.insert(
                user_id,
                MessageTracker {
                    first_message: now,
                    message_count: 1,
                },
            );
            false
        }
    }

    pub fn get_tracker(&self) -> Arc<Mutex<HashMap<UserId, MessageTracker>>> {
        self.message_tracker.clone()
    }
}
