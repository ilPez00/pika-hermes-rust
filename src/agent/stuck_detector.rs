use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecoveryLevel {
    Hint,
    ForceDirective,
    AutoKill,
}

#[derive(Debug)]
pub struct StuckSignal {
    pub description: String,
    pub level: RecoveryLevel,
}

pub struct StuckDetector {
    history: VecDeque<String>,
    screen_hash_history: VecDeque<i32>,
    threshold: usize,
}

impl StuckDetector {
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(8),
            screen_hash_history: VecDeque::with_capacity(8),
            threshold: 4,
        }
    }

    pub fn record(&mut self, action: String, screen_hash: i32, screen_diff_count: usize, _error: Option<&str>) -> Option<StuckSignal> {
        self.history.push_back(action);
        self.screen_hash_history.push_back(screen_hash);

        if self.history.len() > 8 {
            self.history.pop_front();
            self.screen_hash_history.pop_front();
        }

        if self.history.len() < self.threshold {
            return None;
        }

        // Check if all recent actions are the same
        let recent: HashSet<&String> = self.history.iter().rev().take(self.threshold).collect();
        if recent.len() == 1 {
            return Some(StuckSignal {
                description: format!("Same action repeated {} times", self.threshold),
                level: RecoveryLevel::Hint,
            });
        }

        // Check if screen hasn't changed
        let screens: HashSet<&i32> = self.screen_hash_history.iter().rev().take(self.threshold).collect();
        if screens.len() == 1 && screen_diff_count == 0 {
            return Some(StuckSignal {
                description: "Screen unchanged for multiple rounds".to_string(),
                level: RecoveryLevel::ForceDirective,
            });
        }

        if self.history.len() >= 7 {
            let all_screens: HashSet<&i32> = self.screen_hash_history.iter().collect();
            if all_screens.len() <= 2 {
                return Some(StuckSignal {
                    description: "Stuck in screen loop".to_string(),
                    level: RecoveryLevel::AutoKill,
                });
            }
        }

        None
    }
}
