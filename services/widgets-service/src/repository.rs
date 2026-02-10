use crate::models::*;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use anyhow::Result;

#[derive(Clone)]
pub struct WidgetRepository {
    pool: PgPool,
}

impl WidgetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new widget for a user
    pub async fn create_widget(
        &self,
        user_id: Uuid,
        widget_type: WidgetType,
        title: String,
        layout: WidgetLayout,
        configuration: Option<serde_json::Value>,
        refresh_interval_minutes: i32,
    ) -> Result<Widget, WidgetError> {
        let widget_id = Uuid::new_v4();
        let layout_json = serde_json::to_value(layout)?;
        let config_json = configuration.unwrap_or(serde_json::Value::Null);
        let now = Utc::now();

        let widget = sqlx::query_as::<_, Widget>(
            "INSERT INTO widgets (
                id, user_id, widget_type, title, is_enabled, layout, 
                configuration, refresh_interval_minutes, last_updated, 
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING 
                id, user_id, widget_type, title, is_enabled, layout, configuration, 
                refresh_interval_minutes, last_updated, created_at, updated_at"
        )
        .bind(widget_id)
        .bind(user_id)
        .bind(widget_type)
        .bind(title)
        .bind(true)
        .bind(layout_json)
        .bind(config_json)
        .bind(refresh_interval_minutes)
        .bind(now)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(widget)
    }

    /// Get all widgets for a user
    pub async fn get_user_widgets(&self, user_id: Uuid) -> Result<Vec<Widget>, WidgetError> {
        let widgets = sqlx::query_as::<_, Widget>(
            "SELECT 
                id, user_id, widget_type, title, is_enabled, layout, configuration, 
                refresh_interval_minutes, last_updated, created_at, updated_at
            FROM widgets 
            WHERE user_id = $1 AND is_enabled = true
            ORDER BY created_at ASC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(widgets)
    }

    /// Get a specific widget by ID
    pub async fn get_widget(&self, widget_id: Uuid, user_id: Uuid) -> Result<Widget, WidgetError> {
        let widget = sqlx::query_as::<_, Widget>(
            "SELECT 
                id, user_id, widget_type, title, is_enabled, layout, configuration, 
                refresh_interval_minutes, last_updated, created_at, updated_at
            FROM widgets 
            WHERE id = $1 AND user_id = $2"
        )
        .bind(widget_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(WidgetError::WidgetNotFound { widget_id })?;

        Ok(widget)
    }

    /// Update a widget
    pub async fn update_widget(
        &self,
        widget_id: Uuid,
        user_id: Uuid,
        title: Option<String>,
        is_enabled: Option<bool>,
        layout: Option<WidgetLayout>,
        configuration: Option<serde_json::Value>,
        refresh_interval_minutes: Option<i32>,
    ) -> Result<Widget, WidgetError> {
        let layout_json = if let Some(layout) = layout {
            Some(serde_json::to_value(layout)?)
        } else {
            None
        };

        let widget = sqlx::query_as::<_, Widget>(
            "UPDATE widgets 
            SET 
                title = COALESCE($3, title),
                is_enabled = COALESCE($4, is_enabled),
                layout = COALESCE($5, layout),
                configuration = COALESCE($6, configuration),
                refresh_interval_minutes = COALESCE($7, refresh_interval_minutes),
                updated_at = $8
            WHERE id = $1 AND user_id = $2
            RETURNING 
                id, user_id, widget_type, title, is_enabled, layout, configuration, 
                refresh_interval_minutes, last_updated, created_at, updated_at"
        )
        .bind(widget_id)
        .bind(user_id)
        .bind(title)
        .bind(is_enabled)
        .bind(layout_json)
        .bind(configuration)
        .bind(refresh_interval_minutes)
        .bind(Utc::now())
        .fetch_optional(&self.pool)
        .await?
        .ok_or(WidgetError::WidgetNotFound { widget_id })?;

        Ok(widget)
    }

    /// Delete a widget
    pub async fn delete_widget(&self, widget_id: Uuid, user_id: Uuid) -> Result<(), WidgetError> {
        let result = sqlx::query(
            "DELETE FROM widgets WHERE id = $1 AND user_id = $2"
        )
        .bind(widget_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(WidgetError::WidgetNotFound { widget_id });
        }

        Ok(())
    }

    /// Update widget last_updated timestamp
    pub async fn update_widget_timestamp(&self, widget_id: Uuid) -> Result<(), WidgetError> {
        sqlx::query(
            "UPDATE widgets SET last_updated = $1 WHERE id = $2"
        )
        .bind(Utc::now())
        .bind(widget_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Create a new dashboard
    pub async fn create_dashboard(
        &self,
        user_id: Uuid,
        name: String,
        is_default: bool,
        layout_config: Option<serde_json::Value>,
        widget_ids: Vec<Uuid>,
    ) -> Result<WidgetDashboard, WidgetError> {
        let dashboard_id = Uuid::new_v4();
        let layout_json = layout_config.unwrap_or(serde_json::Value::Null);
        let widgets_json = serde_json::to_value(widget_ids)?;
        let now = Utc::now();

        // If this is set as default, unset other defaults for this user
        if is_default {
            sqlx::query(
                "UPDATE widget_dashboards SET is_default = false WHERE user_id = $1"
            )
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        }

        let dashboard = sqlx::query_as::<_, WidgetDashboard>(
            "INSERT INTO widget_dashboards (
                id, user_id, name, is_default, layout_config, widgets, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, user_id, name, is_default, layout_config, widgets, created_at, updated_at"
        )
        .bind(dashboard_id)
        .bind(user_id)
        .bind(name)
        .bind(is_default)
        .bind(layout_json)
        .bind(widgets_json)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(dashboard)
    }

    /// Get user's default dashboard
    pub async fn get_default_dashboard(&self, user_id: Uuid) -> Result<Option<WidgetDashboard>, WidgetError> {
        let dashboard = sqlx::query_as::<_, WidgetDashboard>(
            "SELECT id, user_id, name, is_default, layout_config, widgets, created_at, updated_at
            FROM widget_dashboards 
            WHERE user_id = $1 AND is_default = true"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(dashboard)
    }

    /// Get all dashboards for a user
    pub async fn get_user_dashboards(&self, user_id: Uuid) -> Result<Vec<WidgetDashboard>, WidgetError> {
        let dashboards = sqlx::query_as::<_, WidgetDashboard>(
            "SELECT id, user_id, name, is_default, layout_config, widgets, created_at, updated_at
            FROM widget_dashboards 
            WHERE user_id = $1
            ORDER BY is_default DESC, created_at ASC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(dashboards)
    }

    /// Get a specific dashboard
    pub async fn get_dashboard(&self, dashboard_id: Uuid, user_id: Uuid) -> Result<WidgetDashboard, WidgetError> {
        let dashboard = sqlx::query_as::<_, WidgetDashboard>(
            "SELECT id, user_id, name, is_default, layout_config, widgets, created_at, updated_at
            FROM widget_dashboards 
            WHERE id = $1 AND user_id = $2"
        )
        .bind(dashboard_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(WidgetError::DashboardNotFound { dashboard_id })?;

        Ok(dashboard)
    }

    /// Update a dashboard
    pub async fn update_dashboard(
        &self,
        dashboard_id: Uuid,
        user_id: Uuid,
        name: Option<String>,
        is_default: Option<bool>,
        layout_config: Option<serde_json::Value>,
        widget_ids: Option<Vec<Uuid>>,
    ) -> Result<WidgetDashboard, WidgetError> {
        // If setting as default, unset other defaults for this user
        if let Some(true) = is_default {
            sqlx::query(
                "UPDATE widget_dashboards SET is_default = false WHERE user_id = $1 AND id != $2"
            )
            .bind(user_id)
            .bind(dashboard_id)
            .execute(&self.pool)
            .await?;
        }

        let widgets_json = if let Some(widget_ids) = widget_ids {
            Some(serde_json::to_value(widget_ids)?)
        } else {
            None
        };

        let dashboard = sqlx::query_as::<_, WidgetDashboard>(
            "UPDATE widget_dashboards 
            SET 
                name = COALESCE($3, name),
                is_default = COALESCE($4, is_default),
                layout_config = COALESCE($5, layout_config),
                widgets = COALESCE($6, widgets),
                updated_at = $7
            WHERE id = $1 AND user_id = $2
            RETURNING id, user_id, name, is_default, layout_config, widgets, created_at, updated_at"
        )
        .bind(dashboard_id)
        .bind(user_id)
        .bind(name)
        .bind(is_default)
        .bind(layout_config)
        .bind(widgets_json)
        .bind(Utc::now())
        .fetch_optional(&self.pool)
        .await?
        .ok_or(WidgetError::DashboardNotFound { dashboard_id })?;

        Ok(dashboard)
    }

    /// Delete a dashboard
    pub async fn delete_dashboard(&self, dashboard_id: Uuid, user_id: Uuid) -> Result<(), WidgetError> {
        let result = sqlx::query(
            "DELETE FROM widget_dashboards WHERE id = $1 AND user_id = $2"
        )
        .bind(dashboard_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(WidgetError::DashboardNotFound { dashboard_id });
        }

        Ok(())
    }

    /// Get widgets that need refresh based on their interval
    pub async fn get_widgets_needing_refresh(&self) -> Result<Vec<Widget>, WidgetError> {
        let widgets = sqlx::query_as::<_, Widget>(
            "SELECT 
                id, user_id, widget_type, title, is_enabled, layout, configuration, 
                refresh_interval_minutes, last_updated, created_at, updated_at
            FROM widgets 
            WHERE is_enabled = true 
            AND last_updated < NOW() - INTERVAL '1 minute' * refresh_interval_minutes
            ORDER BY last_updated ASC
            LIMIT 100"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(widgets)
    }
}