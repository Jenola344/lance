//! Stellar Horizon + Soroban RPC service — stub.
//! TODO: See docs/ISSUES.md for full implementation spec.

use anyhow::Result;

pub struct StellarService;

impl StellarService {
    pub fn from_env() -> Self { Self }

    /// TODO: Build and sign XDR transaction to call escrow.release_milestone.
    pub async fn release_milestone(&self, _job_id: &str, _milestone_index: i32) -> Result<String> {
        todo!("Wire Soroban contract call — see docs/ISSUES.md")
    }

    /// TODO: Call escrow.open_dispute via Soroban RPC.
    pub async fn open_dispute(&self, _job_id: &str) -> Result<String> {
        todo!("Wire Soroban contract call — see docs/ISSUES.md")
    }

    /// TODO: Call escrow.resolve_dispute via Soroban RPC.
    pub async fn resolve_dispute(&self, _job_id: &str, _bps: u32) -> Result<String> {
        todo!("Wire Soroban contract call — see docs/ISSUES.md")
    }
}
