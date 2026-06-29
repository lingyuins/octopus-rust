use sqlx::SqlitePool;

/// Run all database migrations
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    create_users_table(pool).await?;
    create_channels_table(pool).await?;
    create_channel_keys_table(pool).await?;
    create_groups_table(pool).await?;
    create_group_items_table(pool).await?;
    create_api_keys_table(pool).await?;
    create_settings_table(pool).await?;
    create_llm_info_table(pool).await?;
    create_alert_rules_table(pool).await?;
    create_alert_notif_channels_table(pool).await?;
    create_alert_state_records_table(pool).await?;
    create_alert_history_table(pool).await?;
    create_stats_tables(pool).await?;
    create_relay_logs_table(pool).await?;
    create_audit_logs_table(pool).await?;
    create_sites_table(pool).await?;
    create_site_accounts_table(pool).await?;
    create_site_tokens_table(pool).await?;
    create_site_user_groups_table(pool).await?;
    create_site_models_table(pool).await?;
    create_site_channel_bindings_table(pool).await?;
    create_proxy_configs_table(pool).await?;
    create_remote_sites_table(pool).await?;
    create_balance_snapshots_table(pool).await?;
    create_checkin_records_table(pool).await?;
    create_api_credentials_table(pool).await?;
    create_webauthn_credentials_table(pool).await?;
    create_model_mappings_table(pool).await?;
    create_redemption_records_table(pool).await?;
    create_remote_usage_records_table(pool).await?;
    create_site_announcements_table(pool).await?;
    create_remote_site_tokens_table(pool).await?;
    create_ws_affinity_table(pool).await?;
    create_runtime_state_table(pool).await?;
    create_auto_strategy_state_table(pool).await?;
    create_circuit_breaker_state_table(pool).await?;
    create_ai_route_tables(pool).await?;
    create_migration_records_table(pool).await?;

    tracing::info!("All database migrations completed successfully");
    Ok(())
}

/// Run log database migrations
pub async fn run_log_migrations(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    create_relay_logs_table(pool).await?;
    create_relay_log_attempts_table(pool).await?;
    tracing::info!("Log database migrations completed successfully");
    Ok(())
}

async fn create_users_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'admin'
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_channels_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS channels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            group_id INTEGER NOT NULL DEFAULT 0,
            type TEXT NOT NULL DEFAULT 'openai',
            enabled INTEGER NOT NULL DEFAULT 1,
            base_urls TEXT NOT NULL DEFAULT '[]',
            model TEXT NOT NULL DEFAULT '',
            custom_model TEXT NOT NULL DEFAULT '',
            proxy_mode TEXT NOT NULL DEFAULT 'direct',
            proxy_config_id INTEGER,
            proxy INTEGER NOT NULL DEFAULT 0,
            auto_sync INTEGER NOT NULL DEFAULT 0,
            auto_group INTEGER NOT NULL DEFAULT 0,
            skip_model_test INTEGER NOT NULL DEFAULT 0,
            key_selection_strategy TEXT NOT NULL DEFAULT '',
            custom_header TEXT NOT NULL DEFAULT '[]',
            param_override TEXT,
            channel_proxy TEXT,
            request_rewrite TEXT,
            match_regex TEXT
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_channel_keys_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS channel_keys (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            channel_id INTEGER NOT NULL,
            enabled INTEGER NOT NULL DEFAULT 1,
            channel_key TEXT NOT NULL,
            status_code INTEGER NOT NULL DEFAULT 0,
            last_use_time_stamp INTEGER NOT NULL DEFAULT 0,
            total_cost REAL NOT NULL DEFAULT 0,
            remark TEXT NOT NULL DEFAULT '',
            FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE
        )"
    ).execute(pool).await?;
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_channel_keys_channel_id ON channel_keys(channel_id)"
    ).execute(pool).await?;
    Ok(())
}

async fn create_groups_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS groups_table (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            endpoint_type TEXT NOT NULL DEFAULT '*',
            endpoint_provider TEXT NOT NULL DEFAULT '',
            outbound_format TEXT NOT NULL DEFAULT '',
            mode INTEGER NOT NULL DEFAULT 1,
            match_regex TEXT NOT NULL DEFAULT '',
            first_token_time_out INTEGER NOT NULL DEFAULT 0,
            session_keep_time INTEGER NOT NULL DEFAULT 0,
            condition TEXT,
            last_test_passed INTEGER,
            last_test_at INTEGER NOT NULL DEFAULT 0
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_group_items_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS group_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            group_id INTEGER NOT NULL,
            channel_id INTEGER NOT NULL,
            model_name TEXT NOT NULL,
            priority INTEGER NOT NULL DEFAULT 0,
            weight INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (group_id) REFERENCES groups_table(id) ON DELETE CASCADE,
            FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE
        )"
    ).execute(pool).await?;
    sqlx::query(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_group_channel_model ON group_items(group_id, channel_id, model_name)"
    ).execute(pool).await?;
    Ok(())
}

async fn create_api_keys_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS api_keys (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            enabled INTEGER NOT NULL DEFAULT 1,
            allowed_models TEXT,
            expiry TEXT,
            max_cost REAL,
            rpm INTEGER,
            tpm INTEGER,
            per_model_quotas TEXT,
            ip_allowlist TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            last_used_at TEXT,
            total_cost REAL NOT NULL DEFAULT 0,
            remark TEXT NOT NULL DEFAULT ''
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_settings_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL DEFAULT '',
            description TEXT NOT NULL DEFAULT '',
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_llm_info_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS llm_info (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            model_name TEXT NOT NULL UNIQUE,
            display_name TEXT NOT NULL DEFAULT '',
            provider TEXT NOT NULL DEFAULT '',
            input_price REAL NOT NULL DEFAULT 0,
            output_price REAL NOT NULL DEFAULT 0,
            cache_input_price REAL NOT NULL DEFAULT 0,
            unit TEXT NOT NULL DEFAULT '1M tokens',
            currency TEXT NOT NULL DEFAULT 'USD',
            type TEXT NOT NULL DEFAULT '',
            enabled INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_alert_rules_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS alert_rules (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            enabled INTEGER NOT NULL DEFAULT 1,
            condition_type TEXT NOT NULL,
            threshold REAL NOT NULL DEFAULT 0,
            scope TEXT NOT NULL DEFAULT '',
            notif_channel_id INTEGER NOT NULL DEFAULT 0,
            cooldown_minutes INTEGER NOT NULL DEFAULT 5,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_alert_notif_channels_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS alert_notif_channels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            enabled INTEGER NOT NULL DEFAULT 1,
            channel_type TEXT NOT NULL,
            config TEXT NOT NULL DEFAULT '{}',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_alert_state_records_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS alert_state_records (
            rule_id INTEGER PRIMARY KEY,
            is_firing INTEGER NOT NULL DEFAULT 0,
            last_fired_at TEXT,
            last_resolved_at TEXT,
            fire_count INTEGER NOT NULL DEFAULT 0
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_alert_history_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS alert_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            rule_id INTEGER NOT NULL,
            rule_name TEXT NOT NULL,
            alert_type TEXT NOT NULL,
            message TEXT NOT NULL,
            notif_channel TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_stats_tables(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS stats_total (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            total_requests INTEGER NOT NULL DEFAULT 0,
            total_tokens INTEGER NOT NULL DEFAULT 0,
            total_cost REAL NOT NULL DEFAULT 0,
            success_count INTEGER NOT NULL DEFAULT 0,
            fail_count INTEGER NOT NULL DEFAULT 0,
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS stats_daily (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL UNIQUE,
            requests INTEGER NOT NULL DEFAULT 0,
            tokens INTEGER NOT NULL DEFAULT 0,
            cost REAL NOT NULL DEFAULT 0,
            success_count INTEGER NOT NULL DEFAULT 0,
            fail_count INTEGER NOT NULL DEFAULT 0
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS stats_hourly (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            hour TEXT NOT NULL UNIQUE,
            requests INTEGER NOT NULL DEFAULT 0,
            tokens INTEGER NOT NULL DEFAULT 0,
            cost REAL NOT NULL DEFAULT 0,
            success_count INTEGER NOT NULL DEFAULT 0,
            fail_count INTEGER NOT NULL DEFAULT 0
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS stats_channel (
            channel_id INTEGER PRIMARY KEY,
            requests INTEGER NOT NULL DEFAULT 0,
            tokens INTEGER NOT NULL DEFAULT 0,
            cost REAL NOT NULL DEFAULT 0,
            success_count INTEGER NOT NULL DEFAULT 0,
            fail_count INTEGER NOT NULL DEFAULT 0,
            avg_latency REAL NOT NULL DEFAULT 0
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS stats_model (
            model_name TEXT PRIMARY KEY,
            requests INTEGER NOT NULL DEFAULT 0,
            tokens INTEGER NOT NULL DEFAULT 0,
            cost REAL NOT NULL DEFAULT 0,
            success_count INTEGER NOT NULL DEFAULT 0,
            fail_count INTEGER NOT NULL DEFAULT 0,
            avg_latency REAL NOT NULL DEFAULT 0
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS stats_api_key (
            api_key_id INTEGER PRIMARY KEY,
            requests INTEGER NOT NULL DEFAULT 0,
            tokens INTEGER NOT NULL DEFAULT 0,
            cost REAL NOT NULL DEFAULT 0,
            success_count INTEGER NOT NULL DEFAULT 0,
            fail_count INTEGER NOT NULL DEFAULT 0
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS stats_site_model_hourly (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site_id INTEGER NOT NULL,
            model_name TEXT NOT NULL,
            hour TEXT NOT NULL,
            requests INTEGER NOT NULL DEFAULT 0,
            tokens INTEGER NOT NULL DEFAULT 0,
            cost REAL NOT NULL DEFAULT 0,
            success_count INTEGER NOT NULL DEFAULT 0,
            fail_count INTEGER NOT NULL DEFAULT 0
        )"
    ).execute(pool).await?;

    Ok(())
}

async fn create_relay_logs_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS relay_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            request_id TEXT NOT NULL,
            api_key_id INTEGER NOT NULL DEFAULT 0,
            group_id INTEGER NOT NULL DEFAULT 0,
            channel_id INTEGER NOT NULL DEFAULT 0,
            key_id INTEGER NOT NULL DEFAULT 0,
            model_name TEXT NOT NULL DEFAULT '',
            upstream_model TEXT,
            request_body TEXT,
            response_body TEXT,
            status_code INTEGER NOT NULL DEFAULT 0,
            latency_ms INTEGER NOT NULL DEFAULT 0,
            prompt_tokens INTEGER NOT NULL DEFAULT 0,
            completion_tokens INTEGER NOT NULL DEFAULT 0,
            total_tokens INTEGER NOT NULL DEFAULT 0,
            cost REAL NOT NULL DEFAULT 0,
            error_message TEXT,
            client_ip TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_relay_logs_created_at ON relay_logs(created_at)"
    ).execute(pool).await?;
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_relay_logs_request_id ON relay_logs(request_id)"
    ).execute(pool).await?;
    Ok(())
}

async fn create_relay_log_attempts_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS relay_log_attempts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            relay_log_id INTEGER NOT NULL,
            attempt INTEGER NOT NULL DEFAULT 0,
            channel_id INTEGER NOT NULL DEFAULT 0,
            key_id INTEGER NOT NULL DEFAULT 0,
            status_code INTEGER NOT NULL DEFAULT 0,
            latency_ms INTEGER NOT NULL DEFAULT 0,
            error_message TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_audit_logs_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS audit_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL DEFAULT 0,
            username TEXT NOT NULL DEFAULT '',
            action TEXT NOT NULL,
            resource TEXT NOT NULL DEFAULT '',
            resource_id TEXT,
            details TEXT,
            ip_address TEXT NOT NULL DEFAULT '',
            user_agent TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_sites_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sites (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            platform TEXT NOT NULL DEFAULT 'new-api',
            base_url TEXT NOT NULL DEFAULT '',
            proxy_mode TEXT NOT NULL DEFAULT 'direct',
            proxy_config_id INTEGER,
            archived INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_site_accounts_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS site_accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site_id INTEGER NOT NULL,
            name TEXT NOT NULL DEFAULT '',
            username TEXT NOT NULL DEFAULT '',
            password TEXT NOT NULL DEFAULT '',
            access_token TEXT NOT NULL DEFAULT '',
            api_key TEXT NOT NULL DEFAULT '',
            sync_enabled INTEGER NOT NULL DEFAULT 1,
            checkin_enabled INTEGER NOT NULL DEFAULT 0,
            proxy_mode TEXT NOT NULL DEFAULT 'inherit',
            proxy_config_id INTEGER,
            last_sync_at TEXT,
            last_checkin_at TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (site_id) REFERENCES sites(id) ON DELETE CASCADE
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_site_tokens_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS site_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site_id INTEGER NOT NULL,
            account_id INTEGER NOT NULL,
            name TEXT NOT NULL DEFAULT '',
            key TEXT NOT NULL,
            enabled INTEGER NOT NULL DEFAULT 1,
            balance REAL NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (site_id) REFERENCES sites(id) ON DELETE CASCADE,
            FOREIGN KEY (account_id) REFERENCES site_accounts(id) ON DELETE CASCADE
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_site_user_groups_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS site_user_groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site_id INTEGER NOT NULL,
            account_id INTEGER NOT NULL,
            name TEXT NOT NULL DEFAULT '',
            key TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (site_id) REFERENCES sites(id) ON DELETE CASCADE
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_site_models_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS site_models (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site_id INTEGER NOT NULL,
            account_id INTEGER NOT NULL,
            model_name TEXT NOT NULL DEFAULT '',
            enabled INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (site_id) REFERENCES sites(id) ON DELETE CASCADE
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_site_channel_bindings_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS site_channel_bindings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site_id INTEGER NOT NULL,
            account_id INTEGER NOT NULL,
            channel_id INTEGER NOT NULL,
            token_id INTEGER NOT NULL DEFAULT 0,
            model_name TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (site_id) REFERENCES sites(id) ON DELETE CASCADE
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_proxy_configs_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS proxy_configurations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            proxy_type TEXT NOT NULL DEFAULT 'http',
            host TEXT NOT NULL DEFAULT '',
            port INTEGER NOT NULL DEFAULT 0,
            username TEXT NOT NULL DEFAULT '',
            password TEXT NOT NULL DEFAULT '',
            enabled INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_remote_sites_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS remote_sites (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL DEFAULT '',
            url TEXT NOT NULL DEFAULT '',
            token TEXT NOT NULL DEFAULT '',
            enabled INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_balance_snapshots_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS balance_snapshots (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site_id INTEGER NOT NULL DEFAULT 0,
            account_id INTEGER NOT NULL DEFAULT 0,
            balance REAL NOT NULL DEFAULT 0,
            currency TEXT NOT NULL DEFAULT 'USD',
            captured_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_checkin_records_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS check_in_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site_id INTEGER NOT NULL DEFAULT 0,
            account_id INTEGER NOT NULL DEFAULT 0,
            result TEXT NOT NULL DEFAULT '',
            points REAL NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_api_credentials_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS api_credential_profiles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            base_url TEXT NOT NULL DEFAULT '',
            api_key TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_webauthn_credentials_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS web_authn_credentials (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            credential_id TEXT NOT NULL UNIQUE,
            public_key TEXT NOT NULL,
            attestation_type TEXT NOT NULL DEFAULT '',
            sign_count INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_model_mappings_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS model_mappings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            pattern TEXT NOT NULL DEFAULT '',
            replacement TEXT NOT NULL DEFAULT '',
            priority INTEGER NOT NULL DEFAULT 0,
            match_type TEXT NOT NULL DEFAULT 'exact',
            group_id INTEGER,
            enabled INTEGER NOT NULL DEFAULT 1
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_redemption_records_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS redemption_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site_id INTEGER NOT NULL DEFAULT 0,
            account_id INTEGER NOT NULL DEFAULT 0,
            code TEXT NOT NULL DEFAULT '',
            result TEXT NOT NULL DEFAULT '',
            points REAL NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_remote_usage_records_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS remote_usage_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site_id INTEGER NOT NULL DEFAULT 0,
            account_id INTEGER NOT NULL DEFAULT 0,
            model_name TEXT NOT NULL DEFAULT '',
            tokens INTEGER NOT NULL DEFAULT 0,
            cost REAL NOT NULL DEFAULT 0,
            recorded_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_site_announcements_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS site_announcements (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site_id INTEGER NOT NULL DEFAULT 0,
            title TEXT NOT NULL DEFAULT '',
            content TEXT NOT NULL DEFAULT '',
            published_at TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_remote_site_tokens_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS remote_site_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site_id INTEGER NOT NULL DEFAULT 0,
            token TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_ws_affinity_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ws_response_affinities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            request_id TEXT NOT NULL DEFAULT '',
            channel_id INTEGER NOT NULL DEFAULT 0,
            key_id INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_runtime_state_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS runtime_state (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL DEFAULT '',
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_auto_strategy_state_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS auto_strategy_states (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            group_id INTEGER NOT NULL,
            channel_id INTEGER NOT NULL,
            key_id INTEGER NOT NULL DEFAULT 0,
            model_name TEXT NOT NULL DEFAULT '',
            success_count INTEGER NOT NULL DEFAULT 0,
            fail_count INTEGER NOT NULL DEFAULT 0,
            last_updated TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_circuit_breaker_state_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS circuit_breaker_states (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            channel_id INTEGER NOT NULL,
            key_id INTEGER NOT NULL DEFAULT 0,
            model_name TEXT NOT NULL DEFAULT '',
            state TEXT NOT NULL DEFAULT 'closed',
            fail_count INTEGER NOT NULL DEFAULT 0,
            last_fail_time TEXT,
            last_updated TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_ai_route_tables(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ai_routes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL DEFAULT '',
            prompt TEXT NOT NULL DEFAULT '',
            model TEXT NOT NULL DEFAULT '',
            enabled INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ai_route_tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            route_id INTEGER NOT NULL DEFAULT 0,
            status TEXT NOT NULL DEFAULT 'pending',
            result TEXT,
            error_message TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn create_migration_records_table(pool: &SqlitePool) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS migration_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            version TEXT NOT NULL UNIQUE,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;
    Ok(())
}