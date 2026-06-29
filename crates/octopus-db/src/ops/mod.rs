pub mod user;
pub mod channel;
pub mod group;
pub mod api_key;
pub mod setting;
pub mod stats;
pub mod alert;
pub mod audit;

use dashmap::DashMap;
use once_cell::sync::OnceCell;
use std::sync::Arc;

use octopus_core::model::{channel::Channel, user::User, api_key::ApiKey, group::Group};

pub type ChannelCache = Arc<DashMap<i32, Channel>>;
pub type UserCache = Arc<DashMap<i32, User>>;
pub type ApiKeyCache = Arc<DashMap<i32, ApiKey>>;
pub type GroupCache = Arc<DashMap<i32, Group>>;

static CHANNEL_CACHE: OnceCell<ChannelCache> = OnceCell::new();
static USER_CACHE: OnceCell<UserCache> = OnceCell::new();
static API_KEY_CACHE: OnceCell<ApiKeyCache> = OnceCell::new();
static GROUP_CACHE: OnceCell<GroupCache> = OnceCell::new();

/// Initialize all caches from database
pub async fn init_caches() -> Result<(), octopus_core::error::AppError> {
    init_channel_cache().await?;
    init_user_cache().await?;
    init_api_key_cache().await?;
    init_group_cache().await?;
    tracing::info!("All caches initialized");
    Ok(())
}

async fn init_channel_cache() -> Result<(), octopus_core::error::AppError> {
    let cache = Arc::new(DashMap::new());
    let channels = channel::list_channels().await?;
    for ch in channels {
        cache.insert(ch.id, ch);
    }
    CHANNEL_CACHE.set(cache).ok();
    Ok(())
}

async fn init_user_cache() -> Result<(), octopus_core::error::AppError> {
    let cache = Arc::new(DashMap::new());
    let users = user::list_users().await?;
    for u in users {
        cache.insert(u.id, u);
    }
    USER_CACHE.set(cache).ok();
    Ok(())
}

async fn init_api_key_cache() -> Result<(), octopus_core::error::AppError> {
    let cache = Arc::new(DashMap::new());
    let keys = api_key::list_api_keys().await?;
    for k in keys {
        cache.insert(k.id, k);
    }
    API_KEY_CACHE.set(cache).ok();
    Ok(())
}

async fn init_group_cache() -> Result<(), octopus_core::error::AppError> {
    let cache = Arc::new(DashMap::new());
    let groups = group::list_groups().await?;
    for g in groups {
        cache.insert(g.id, g);
    }
    GROUP_CACHE.set(cache).ok();
    Ok(())
}

pub fn get_channel_cache() -> &'static ChannelCache {
    CHANNEL_CACHE.get().expect("channel cache not initialized")
}

pub fn get_user_cache() -> &'static UserCache {
    USER_CACHE.get().expect("user cache not initialized")
}

pub fn get_api_key_cache() -> &'static ApiKeyCache {
    API_KEY_CACHE.get().expect("api key cache not initialized")
}

pub fn get_group_cache() -> &'static GroupCache {
    GROUP_CACHE.get().expect("group cache not initialized")
}