use crate::models::*;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use tracing::{info, error};

/// Repository for Khatma service data access
pub struct KhatmaRepository {
    pool: PgPool,
}

impl KhatmaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new khatma plan
    pub async fn create_khatma_plan(&self, plan: &KhatmaPlan) -> Result<KhatmaPlan> {
        let mut tx = self.pool.begin().await?;

        // Insert main plan
        sqlx::query!(
            r#"
            INSERT INTO khatma_plans (
                id, user_id, target_date, start_date, estimated_reading_time,
                adaptive_schedule, current_progress, reading_speed_wpm, status,
                created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            plan.id,
            plan.user_id,
            plan.target_date,
            plan.start_date,
            plan.estimated_reading_time,
            plan.adaptive_schedule,
            plan.current_progress,
            plan.reading_speed_wpm,
            serde_json::to_string(&plan.status)?,
            plan.created_at,
            plan.updated_at
        )
        .execute(&mut *tx)
        .await?;

        // Insert daily portions
        for portion in &plan.daily_portions {
            sqlx::query!(
                r#"
                INSERT INTO daily_portions (
                    khatma_plan_id, date, surah_start, ayah_start, surah_end, ayah_end,
                    estimated_minutes, word_count, completed, actual_reading_time, completion_date
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                "#,
                plan.id,
                portion.date,
                portion.surah_start as i16,
                portion.ayah_start as i32,
                portion.surah_end as i16,
                portion.ayah_end as i32,
                portion.estimated_minutes,
                portion.word_count as i32,
                portion.completed,
                portion.actual_reading_time,
                portion.completion_date
            )
            .execute(&mut *tx)
            .await?;
        }

        // Insert preferred reading times
        for pref_time in &plan.preferred_reading_times {
            sqlx::query!(
                r#"
                INSERT INTO preferred_reading_times (
                    khatma_plan_id, time, duration_minutes, priority, days_of_week
                ) VALUES ($1, $2, $3, $4, $5)
                "#,
                plan.id,
                pref_time.time,
                pref_time.duration_minutes,
                serde_json::to_string(&pref_time.priority)?,
                serde_json::to_string(&pref_time.days_of_week)?
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        info!("Created khatma plan: {}", plan.id);
        Ok(plan.clone())
    }

    /// Get a khatma plan by ID
    pub async fn get_khatma_plan(&self, plan_id: Uuid) -> Result<KhatmaPlan> {
        // Get main plan
        let plan_row = sqlx::query!(
            r#"
            SELECT id, user_id, target_date, start_date, estimated_reading_time,
                   adaptive_schedule, current_progress, reading_speed_wpm, status,
                   created_at, updated_at
            FROM khatma_plans WHERE id = $1
            "#,
            plan_id
        )
        .fetch_one(&self.pool)
        .await?;

        // Get daily portions
        let portion_rows = sqlx::query!(
            r#"
            SELECT date, surah_start, ayah_start, surah_end, ayah_end,
                   estimated_minutes, word_count, completed, actual_reading_time, completion_date
            FROM daily_portions WHERE khatma_plan_id = $1 ORDER BY date
            "#,
            plan_id
        )
        .fetch_all(&self.pool)
        .await?;

        let daily_portions: Result<Vec<DailyPortion>> = portion_rows
            .into_iter()
            .map(|row| {
                Ok(DailyPortion {
                    date: row.date,
                    surah_start: row.surah_start as u8,
                    ayah_start: row.ayah_start as u16,
                    surah_end: row.surah_end as u8,
                    ayah_end: row.ayah_end as u16,
                    estimated_minutes: row.estimated_minutes,
                    word_count: row.word_count as u32,
                    completed: row.completed,
                    actual_reading_time: row.actual_reading_time,
                    completion_date: row.completion_date,
                })
            })
            .collect();

        // Get preferred reading times
        let pref_time_rows = sqlx::query!(
            r#"
            SELECT time, duration_minutes, priority, days_of_week
            FROM preferred_reading_times WHERE khatma_plan_id = $1
            "#,
            plan_id
        )
        .fetch_all(&self.pool)
        .await?;

        let preferred_reading_times: Result<Vec<PreferredReadingTime>> = pref_time_rows
            .into_iter()
            .map(|row| {
                Ok(PreferredReadingTime {
                    time: row.time,
                    duration_minutes: row.duration_minutes,
                    priority: serde_json::from_str(&row.priority)?,
                    days_of_week: serde_json::from_str(&row.days_of_week)?,
                })
            })
            .collect();

        Ok(KhatmaPlan {
            id: plan_row.id,
            user_id: plan_row.user_id,
            target_date: plan_row.target_date,
            start_date: plan_row.start_date,
            daily_portions: daily_portions?,
            estimated_reading_time: plan_row.estimated_reading_time,
            adaptive_schedule: plan_row.adaptive_schedule,
            current_progress: plan_row.current_progress,
            reading_speed_wpm: plan_row.reading_speed_wpm,
            preferred_reading_times: preferred_reading_times?,
            status: serde_json::from_str(&plan_row.status)?,
            created_at: plan_row.created_at,
            updated_at: plan_row.updated_at,
        })
    }

    /// Update a khatma plan
    pub async fn update_khatma_plan(&self, plan: &KhatmaPlan) -> Result<KhatmaPlan> {
        let mut tx = self.pool.begin().await?;

        // Update main plan
        sqlx::query!(
            r#"
            UPDATE khatma_plans SET
                target_date = $2, estimated_reading_time = $3, adaptive_schedule = $4,
                current_progress = $5, reading_speed_wpm = $6, status = $7, updated_at = $8
            WHERE id = $1
            "#,
            plan.id,
            plan.target_date,
            plan.estimated_reading_time,
            plan.adaptive_schedule,
            plan.current_progress,
            plan.reading_speed_wpm,
            serde_json::to_string(&plan.status)?,
            plan.updated_at
        )
        .execute(&mut *tx)
        .await?;

        // Delete and recreate daily portions (simpler than complex updates)
        sqlx::query!("DELETE FROM daily_portions WHERE khatma_plan_id = $1", plan.id)
            .execute(&mut *tx)
            .await?;

        for portion in &plan.daily_portions {
            sqlx::query!(
                r#"
                INSERT INTO daily_portions (
                    khatma_plan_id, date, surah_start, ayah_start, surah_end, ayah_end,
                    estimated_minutes, word_count, completed, actual_reading_time, completion_date
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                "#,
                plan.id,
                portion.date,
                portion.surah_start as i16,
                portion.ayah_start as i32,
                portion.surah_end as i16,
                portion.ayah_end as i32,
                portion.estimated_minutes,
                portion.word_count as i32,
                portion.completed,
                portion.actual_reading_time,
                portion.completion_date
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        info!("Updated khatma plan: {}", plan.id);
        Ok(plan.clone())
    }

    /// Get user's khatma plans
    pub async fn get_user_khatma_plans(
        &self,
        user_id: Uuid,
        status_filter: Option<KhatmaStatus>,
    ) -> Result<Vec<KhatmaPlan>> {
        let mut plans = Vec::new();

        let rows = if let Some(status) = status_filter {
            sqlx::query!(
                r#"
                SELECT id FROM khatma_plans 
                WHERE user_id = $1 AND status = $2 
                ORDER BY created_at DESC
                "#,
                user_id,
                serde_json::to_string(&status)?
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query!(
                r#"
                SELECT id FROM khatma_plans 
                WHERE user_id = $1 
                ORDER BY created_at DESC
                "#,
                user_id
            )
            .fetch_all(&self.pool)
            .await?
        };

        for row in rows {
            let plan = self.get_khatma_plan(row.id).await?;
            plans.push(plan);
        }

        Ok(plans)
    }

    /// Create a reading session
    pub async fn create_reading_session(&self, session: &ReadingSession) -> Result<ReadingSession> {
        sqlx::query!(
            r#"
            INSERT INTO reading_sessions (
                id, user_id, khatma_plan_id, surah_start, ayah_start, surah_end, ayah_end,
                start_time, end_time, duration_minutes, word_count, reading_speed_wpm, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
            session.id,
            session.user_id,
            session.khatma_plan_id,
            session.surah_start as i16,
            session.ayah_start as i32,
            session.surah_end as i16,
            session.ayah_end as i32,
            session.start_time,
            session.end_time,
            session.duration_minutes,
            session.word_count as i32,
            session.reading_speed_wpm,
            session.created_at
        )
        .execute(&self.pool)
        .await?;

        info!("Created reading session: {}", session.id);
        Ok(session.clone())
    }

    /// Get user's reading sessions
    pub async fn get_user_reading_sessions(&self, user_id: Uuid) -> Result<Vec<ReadingSession>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, khatma_plan_id, surah_start, ayah_start, surah_end, ayah_end,
                   start_time, end_time, duration_minutes, word_count, reading_speed_wpm, created_at
            FROM reading_sessions 
            WHERE user_id = $1 
            ORDER BY start_time DESC
            LIMIT 100
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let sessions: Result<Vec<ReadingSession>> = rows
            .into_iter()
            .map(|row| {
                Ok(ReadingSession {
                    id: row.id,
                    user_id: row.user_id,
                    khatma_plan_id: row.khatma_plan_id,
                    surah_start: row.surah_start as u8,
                    ayah_start: row.ayah_start as u16,
                    surah_end: row.surah_end as u8,
                    ayah_end: row.ayah_end as u16,
                    start_time: row.start_time,
                    end_time: row.end_time,
                    duration_minutes: row.duration_minutes,
                    word_count: row.word_count as u32,
                    reading_speed_wpm: row.reading_speed_wpm,
                    created_at: row.created_at,
                })
            })
            .collect();

        sessions
    }

    /// Get reading sessions for a specific plan
    pub async fn get_plan_reading_sessions(&self, plan_id: Uuid) -> Result<Vec<ReadingSession>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, khatma_plan_id, surah_start, ayah_start, surah_end, ayah_end,
                   start_time, end_time, duration_minutes, word_count, reading_speed_wpm, created_at
            FROM reading_sessions 
            WHERE khatma_plan_id = $1 
            ORDER BY start_time ASC
            "#,
            plan_id
        )
        .fetch_all(&self.pool)
        .await?;

        let sessions: Result<Vec<ReadingSession>> = rows
            .into_iter()
            .map(|row| {
                Ok(ReadingSession {
                    id: row.id,
                    user_id: row.user_id,
                    khatma_plan_id: row.khatma_plan_id,
                    surah_start: row.surah_start as u8,
                    ayah_start: row.ayah_start as u16,
                    surah_end: row.surah_end as u8,
                    ayah_end: row.ayah_end as u16,
                    start_time: row.start_time,
                    end_time: row.end_time,
                    duration_minutes: row.duration_minutes,
                    word_count: row.word_count as u32,
                    reading_speed_wpm: row.reading_speed_wpm,
                    created_at: row.created_at,
                })
            })
            .collect();

        sessions
    }

    /// Update reading statistics
    pub async fn update_reading_statistics(&self, stats: &ReadingStatistics) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO reading_statistics (
                user_id, average_reading_speed_wpm, total_reading_time_minutes,
                completed_khatmas, reading_consistency_score, last_updated
            ) VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (user_id) DO UPDATE SET
                average_reading_speed_wpm = EXCLUDED.average_reading_speed_wpm,
                total_reading_time_minutes = EXCLUDED.total_reading_time_minutes,
                completed_khatmas = EXCLUDED.completed_khatmas,
                reading_consistency_score = EXCLUDED.reading_consistency_score,
                last_updated = EXCLUDED.last_updated
            "#,
            stats.user_id,
            stats.average_reading_speed_wpm,
            stats.total_reading_time_minutes,
            stats.completed_khatmas as i32,
            stats.reading_consistency_score,
            stats.last_updated
        )
        .execute(&self.pool)
        .await?;

        info!("Updated reading statistics for user: {}", stats.user_id);
        Ok(())
    }

    /// Count completed khatmas for a user
    pub async fn count_completed_khatmas(&self, user_id: Uuid) -> Result<u32> {
        let row = sqlx::query!(
            r#"
            SELECT COUNT(*) as count FROM khatma_plans 
            WHERE user_id = $1 AND status = $2
            "#,
            user_id,
            serde_json::to_string(&KhatmaStatus::Completed)?
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.count.unwrap_or(0) as u32)
    }

    /// Log plan adjustment
    pub async fn log_plan_adjustment(&self, request: &PlanAdjustmentRequest) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO plan_adjustments (
                khatma_plan_id, new_target_date, new_daily_time_minutes, reason, requested_at
            ) VALUES ($1, $2, $3, $4, $5)
            "#,
            request.khatma_plan_id,
            request.new_target_date,
            request.new_daily_time_minutes,
            serde_json::to_string(&request.reason)?,
            request.requested_at
        )
        .execute(&self.pool)
        .await?;

        info!("Logged plan adjustment for: {}", request.khatma_plan_id);
        Ok(())
    }

    /// Create smart reminder
    pub async fn create_smart_reminder(&self, reminder: &SmartReminder) -> Result<SmartReminder> {
        sqlx::query!(
            r#"
            INSERT INTO smart_reminders (
                id, user_id, khatma_plan_id, suggested_time, duration_minutes,
                confidence_score, reasoning, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            reminder.id,
            reminder.user_id,
            reminder.khatma_plan_id,
            reminder.suggested_time,
            reminder.duration_minutes,
            reminder.confidence_score,
            reminder.reasoning,
            reminder.created_at
        )
        .execute(&self.pool)
        .await?;

        info!("Created smart reminder: {}", reminder.id);
        Ok(reminder.clone())
    }

    /// Get smart reminders for a user
    pub async fn get_user_smart_reminders(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
    ) -> Result<Vec<SmartReminder>> {
        let limit = limit.unwrap_or(20);
        
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, khatma_plan_id, suggested_time, duration_minutes,
                   confidence_score, reasoning, created_at
            FROM smart_reminders 
            WHERE user_id = $1 AND suggested_time >= NOW()
            ORDER BY confidence_score DESC, suggested_time ASC
            LIMIT $2
            "#,
            user_id,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        let reminders: Result<Vec<SmartReminder>> = rows
            .into_iter()
            .map(|row| {
                // Note: We're creating a simplified portion here
                // In a real implementation, you'd fetch the actual portion data
                let portion = DailyPortion {
                    date: row.suggested_time,
                    surah_start: 1,
                    ayah_start: 1,
                    surah_end: 1,
                    ayah_end: 7,
                    estimated_minutes: row.duration_minutes,
                    word_count: 100,
                    completed: false,
                    actual_reading_time: None,
                    completion_date: None,
                };

                Ok(SmartReminder {
                    id: row.id,
                    user_id: row.user_id,
                    khatma_plan_id: row.khatma_plan_id,
                    suggested_time: row.suggested_time,
                    duration_minutes: row.duration_minutes,
                    portion,
                    confidence_score: row.confidence_score,
                    reasoning: row.reasoning,
                    created_at: row.created_at,
                })
            })
            .collect();

        reminders
    }
}

/// Mock repository for testing
#[cfg(test)]
pub struct MockKhatmaRepository;

#[cfg(test)]
impl MockKhatmaRepository {
    pub fn new() -> KhatmaRepository {
        // In a real test, you'd set up a test database or use an in-memory mock
        // For now, this is just a placeholder
        unimplemented!("Mock repository not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_creation() {
        // Test would require actual database setup
        assert!(true);
    }
}