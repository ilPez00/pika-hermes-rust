#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BudgetStatus {
    Ok,
    SoftLimit,
    HardLimit,
}

pub struct TaskBudget {
    hard_limit_tokens: i32,
    soft_limit_tokens: i32,
    hard_limit_cost: f64,
    soft_limit_cost: f64,
}

impl TaskBudget {
    pub fn from_settings() -> Self {
        Self {
            hard_limit_tokens: std::env::var("HARD_LIMIT_TOKENS").ok().and_then(|s| s.parse().ok()).unwrap_or(100_000),
            soft_limit_tokens: std::env::var("SOFT_LIMIT_TOKENS").ok().and_then(|s| s.parse().ok()).unwrap_or(50_000),
            hard_limit_cost: std::env::var("HARD_LIMIT_COST").ok().and_then(|s| s.parse().ok()).unwrap_or(0.10),
            soft_limit_cost: std::env::var("SOFT_LIMIT_COST").ok().and_then(|s| s.parse().ok()).unwrap_or(0.05),
        }
    }

    pub fn check(&self, total_tokens: i32, estimated_cost: f64) -> BudgetStatus {
        if total_tokens >= self.hard_limit_tokens || estimated_cost >= self.hard_limit_cost {
            return BudgetStatus::HardLimit;
        }
        if total_tokens >= self.soft_limit_tokens || estimated_cost >= self.soft_limit_cost {
            return BudgetStatus::SoftLimit;
        }
        BudgetStatus::Ok
    }
}
