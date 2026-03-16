use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    db::AppState,
    error::{AppError, Result},
    models::Milestone,
};

pub async fn release_milestone(
    State(state): State<AppState>,
    Path((job_id, milestone_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Milestone>> {
    // Verify milestone belongs to job
    let milestone = sqlx::query_as!(
        Milestone,
        r#"SELECT id, job_id, index, title, amount_usdc, status, tx_hash, released_at
           FROM milestones WHERE id = $1 AND job_id = $2"#,
        milestone_id,
        job_id,
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("milestone not found".into()))?;

    if milestone.status != "pending" {
        return Err(AppError::BadRequest("milestone already released".into()));
    }

    // TODO: call Soroban escrow contract via stellar.rs service
    // services::stellar::release_milestone(&job_id.to_string(), milestone.index).await?;

    let updated = sqlx::query_as!(
        Milestone,
        r#"UPDATE milestones SET status = 'released', released_at = now()
           WHERE id = $1
           RETURNING id, job_id, index, title, amount_usdc, status, tx_hash, released_at"#,
        milestone_id,
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(updated))
}
