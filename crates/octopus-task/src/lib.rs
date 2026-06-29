pub mod scheduler;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

/// Task scheduler - manages background tasks
pub struct TaskScheduler {
    tasks: Arc<RwLock<HashMap<String, TaskHandle>>>,
}

struct TaskHandle {
    cancel_tx: tokio::sync::oneshot::Sender<()>,
    join_handle: JoinHandle<()>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a periodic task
    pub async fn register<F, Fut>(&self, name: &str, interval_secs: u64, f: F)
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let (cancel_tx, mut cancel_rx) = tokio::sync::oneshot::channel::<()>();
        let name_clone = name.to_string();
        let join_handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(interval_secs));
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        f().await;
                    }
                    _ = &mut cancel_rx => {
                        tracing::info!("Task {} cancelled", name_clone);
                        break;
                    }
                }
            }
        });

        let mut tasks = self.tasks.write().await;
        tasks.insert(name.to_string(), TaskHandle { cancel_tx, join_handle });
        tracing::info!("Registered task: {} (interval: {}s)", name, interval_secs);
    }

    /// Unregister and cancel a task
    pub async fn unregister(&self, name: &str) {
        if let Some(handle) = self.tasks.write().await.remove(name) {
            let _ = handle.cancel_tx.send(());
            let _ = handle.join_handle.await;
            tracing::info!("Unregistered task: {}", name);
        }
    }

    /// Shutdown all tasks
    pub async fn shutdown(&self) {
        let mut tasks = self.tasks.write().await;
        for (name, handle) in tasks.drain() {
            let _ = handle.cancel_tx.send(());
            let _ = handle.join_handle.await;
            tracing::info!("Shutdown task: {}", name);
        }
    }
}

impl Default for TaskScheduler {
    fn default() -> Self { Self::new() }
}

/// Global task scheduler
use once_cell::sync::OnceCell;
static SCHEDULER: OnceCell<TaskScheduler> = OnceCell::new();

pub fn global() -> &'static TaskScheduler {
    SCHEDULER.get_or_init(TaskScheduler::new)
}

/// Initialize all background tasks
pub async fn init_tasks() {
    let sched = global();

    // Price update - every 6 hours
    sched.register("price_update", 21600, || async {
        tracing::info!("Price update task running");
    }).await;

    // Stats save - every 30 seconds
    sched.register("stats_save", 30, || async {
        // persist in-memory stats
    }).await;

    // Runtime state save - every 60 seconds
    sched.register("runtime_state_save", 60, || async {
        let _ = octopus_relay::balancer::runtime_state::save().await;
    }).await;

    // Relay log save - every 10 seconds
    sched.register("relay_log_save", 10, || async {
        // flush relay logs
    }).await;

    // Alert evaluate - every 60 seconds
    sched.register("alert_evaluate", 60, || async {
        // evaluate alert rules
    }).await;

    // Sync LLM models - every 30 minutes
    sched.register("sync_llm", 1800, || async {
        tracing::info!("LLM sync task running");
    }).await;

    // Hub balance capture - every 30 minutes
    sched.register("hub_balance_capture", 1800, || async {
        // capture hub balances
    }).await;

    // Hub auto checkin - every 4 hours
    sched.register("hub_auto_checkin", 14400, || async {
        // auto checkin hub accounts
    }).await;

    // Hub announcement fetch - every 30 minutes
    sched.register("hub_announcement_fetch", 1800, || async {
        // fetch announcements
    }).await;

    // WebDAV backup - schedule driven
    sched.register("webdav_backup", 86400, || async {
        // cloud backup
    }).await;

    // Site sync - every 5 minutes
    sched.register("site_sync", 300, || async {
        tracing::info!("Site sync task running");
    }).await;

    // Site checkin - random schedule
    sched.register("site_checkin", 3600, || async {
        tracing::info!("Site checkin task running");
    }).await;

    // Base URL delay probe - every 5 minutes
    sched.register("base_url_delay", 300, || async {
        // probe channel base URL latency
    }).await;

    tracing::info!("All background tasks initialized");
}

/// Shutdown the task scheduler
pub async fn shutdown() {
    global().shutdown().await;
}