//! Background worker — stub.
//! TODO: See docs/ISSUES.md for full implementation spec.

use sqlx::PgPool;

pub async fn run_judge_worker(_pool: PgPool) {
    // TODO: Poll open disputes, call JudgeService, persist verdicts, resolve on-chain.
    // See docs/ISSUES.md — "Background Judge Worker".
    todo!("Implement judge worker — see docs/ISSUES.md")
}
