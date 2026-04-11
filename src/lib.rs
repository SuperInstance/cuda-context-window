//! cuda-context-window: Manage agent context windows efficiently.
//!
//! Token budgeting, message compression, and summarization triggers.

use serde::{Deserialize, Serialize};

/// A message in the context window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMessage {
    pub role: String,       // "user", "assistant", "system"
    pub content: String,
    pub token_count: u32,
    pub priority: u8,       // 0=discardable, 5=normal, 9=critical
    pub created_at: u64,
}

/// Context window state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextWindow {
    pub messages: Vec<ContextMessage>,
    pub budget_tokens: u32,
    pub used_tokens: u32,
    pub compression_level: u8,
}

/// A compaction decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactionAdvice {
    pub should_compact: bool,
    pub reason: String,
    pub messages_to_summarize: usize,
    pub estimated_savings: u32,
}

impl ContextWindow {
    pub fn new(budget_tokens: u32) -> Self {
        Self { messages: Vec::new(), budget_tokens, used_tokens: 0, compression_level: 0 }
    }

    pub fn add(&mut self, msg: ContextMessage) -> Result<(), String> {
        if self.used_tokens + msg.token_count > self.budget_tokens {
            return Err(format!("budget exceeded: {} + {} > {}", self.used_tokens, msg.token_count, self.budget_tokens));
        }
        self.used_tokens += msg.token_count;
        self.messages.push(msg);
        Ok(())
    }

    pub fn utilization(&self) -> f64 {
        if self.budget_tokens == 0 { return 1.0; }
        self.used_tokens as f64 / self.budget_tokens as f64
    }

    /// Check if compaction is needed
    pub fn check_compaction(&self) -> CompactionAdvice {
        let util = self.utilization();
        if util < 0.7 {
            return CompactionAdvice { should_compact: false, reason: "under 70% utilization".into(), messages_to_summarize: 0, estimated_savings: 0 };
        }

        // Find low-priority messages to summarize
        let discardable: Vec<&ContextMessage> = self.messages.iter().filter(|m| m.priority < 3).collect();
        let savings: u32 = discardable.iter().map(|m| m.token_count).sum();

        CompactionAdvice {
            should_compact: util > 0.85,
            reason: if util > 0.85 { "over 85% utilization".into() } else { "approaching limit".into() },
            messages_to_summarize: discardable.len(),
            estimated_savings: savings,
        }
    }

    /// Remove oldest low-priority messages
    pub fn compact(&mut self, keep_critical: bool) -> u32 {
        let before = self.used_tokens;
        if keep_critical {
            let mut keep = Vec::new();
            let mut freed = 0u32;
            for msg in self.messages.drain(..) {
                if msg.priority >= 5 || freed + msg.token_count > self.budget_tokens as u32 / 4 {
                    keep.push(msg);
                } else {
                    freed += msg.token_count;
                }
            }
            self.used_tokens -= freed;
            self.messages = keep;
            freed
        } else {
            let removed: Vec<ContextMessage> = self.messages.drain(..self.messages.len().saturating_sub(5)).collect();
            let freed: u32 = removed.iter().map(|m| m.token_count).sum();
            self.used_tokens = self.used_tokens.saturating_sub(freed);
            freed
        }
    }

    pub fn message_count(&self) -> usize { self.messages.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_msg(role: &str, tokens: u32, priority: u8) -> ContextMessage {
        ContextMessage { role: role.into(), content: "x".repeat(tokens as usize), token_count: tokens, priority, created_at: 0 }
    }

    #[test]
    fn test_add_and_utilization() {
        let mut w = ContextWindow::new(1000);
        w.add(make_msg("user", 300, 5)).unwrap();
        w.add(make_msg("assistant", 200, 5)).unwrap();
        assert!((w.utilization() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_budget_exceeded() {
        let mut w = ContextWindow::new(100);
        assert!(w.add(make_msg("user", 200, 5)).is_err());
    }

    #[test]
    fn test_compaction_trigger() {
        let mut w = ContextWindow::new(1000);
        for i in 0..20 {
            w.add(make_msg("user", 50, if i < 15 { 1 } else { 9 })).unwrap();
        }
        let advice = w.check_compaction();
        assert!(advice.should_compact);
    }

    #[test]
    fn test_compact_preserves_critical() {
        let mut w = ContextWindow::new(1000);
        for i in 0..10 {
            w.add(make_msg("user", 50, if i < 8 { 1 } else { 9 })).unwrap();
        }
        let freed = w.compact(true);
        assert!(freed > 0);
        assert!(w.messages.iter().all(|m| m.priority >= 5));
    }

    #[test]
    fn test_compact_keep_recent() {
        let mut w = ContextWindow::new(1000);
        for _ in 0..20 {
            w.add(make_msg("user", 50, 5)).unwrap();
        }
        let freed = w.compact(false);
        assert!(freed > 0);
        assert_eq!(w.message_count(), 5);
    }
}
