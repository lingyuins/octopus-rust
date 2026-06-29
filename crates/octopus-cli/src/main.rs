use clap::{Parser, Subcommand};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(name = "octopus", about = "A Simple, Beautiful, and Elegant LLM API Aggregation & Load Balancing Service")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Config file path (default: data/config.json)
    #[arg(short, long)]
    config: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the Octopus server
    Start,
    /// Show version information
    Version,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Start => run_start(cli.config).await?,
        Commands::Version => run_version(),
    }

    Ok(())
}

fn run_version() {
    println!("{} v{}", octopus_core::constants::APP_NAME, env!("CARGO_PKG_VERSION"));
    println!("Description: {}", octopus_core::constants::APP_DESC);
    println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Repository: https://github.com/lingyuins/octopus");
}

async fn run_start(config_path: Option<String>) -> anyhow::Result<()> {
    // Print banner
    print_banner();

    // Load configuration
    let config = octopus_core::AppConfig::load(config_path.as_deref())?;
    tracing::info!("Config loaded: server={}:{}", config.server.host, config.server.port);

    // Set log level
    let level = config.log.level.to_lowercase();
    tracing::info!("Log level: {}", level);

    // Initialize encryption
    if !config.security.encryption_key.is_empty() {
        tracing::info!("Encryption key configured");
    } else if !config.auth.jwt_secret.is_empty() {
        tracing::info!("Using JWT secret for encryption derivation");
    } else {
        anyhow::bail!("Security encryption key (or persistent JWT secret) is required to prevent data loss on restart");
    }

    // Initialize database
    let db_path = config.database.path.clone();
    tracing::info!("Initializing database: {} ({})", db_path, config.database.r#type);
    octopus_db::init_db(&db_path).await?;

    // Initialize log database (if separate)
    if !config.database.log_type.is_empty() && !config.database.log_path.is_empty() {
        tracing::info!("Initializing log database: {}", config.database.log_path);
        octopus_db::init_log_db(&config.database.log_path).await?;
    }

    // Initialize caches
    octopus_db::ops::init_caches().await?;
    tracing::info!("Caches initialized");

    // Ensure admin user exists
    if !octopus_db::ops::user::ensure_admin_exists().await? {
        tracing::warn!("No admin user found. Use the bootstrap API to create one.");
    }

    // Start background tasks
    octopus_task::init_tasks().await;

    // Start HTTP server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("Starting Octopus server on {}", addr);

    // Graceful shutdown handling
    let server_task = tokio::spawn(async move {
        if let Err(e) = octopus_server::start(&addr).await {
            tracing::error!("Server error: {}", e);
        }
    });

    // Wait for shutdown signal
    tokio::signal::ctrl_c().await?;
    tracing::info!("Shutting down...");

    // Shutdown tasks
    octopus_task::shutdown().await;

    // Close database
    octopus_db::close().await?;

    // Cancel server
    server_task.abort();
    let _ = server_task.await;

    tracing::info!("Octopus stopped");
    Ok(())
}

fn print_banner() {
    let banner = r#"
   ___        _                           
  / _ \  ___ | |_  ___  _ __  _   _  ___ 
 | | | |/ __|| __|/ _ \| '_ \| | | |/ __|
 | |_| |\__ \| |_| (_) | |_) | |_| |\__ \
  \___/ |___/ \__|\___/| .__/ \__,_||___/
                       |_|                
   LLM API Aggregation & Load Balancing
"#;
    println!("{}", banner);
    println!("  v{}  |  https://github.com/lingyuins/octopus\n", env!("CARGO_PKG_VERSION"));
}