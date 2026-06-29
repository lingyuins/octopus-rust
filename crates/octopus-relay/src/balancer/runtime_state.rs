use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static RUNTIME_STATE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn get(key: &str) -> Option<String> {
    RUNTIME_STATE.lock().ok()?.get(key).cloned()
}
pub fn set(key: &str, value: &str) {
    if let Ok(mut state) = RUNTIME_STATE.lock() {
        state.insert(key.to_string(), value.to_string());
    }
}
pub async fn save() -> Result<(), octopus_core::error::AppError> { Ok(()) }
pub async fn load() -> Result<(), octopus_core::error::AppError> { Ok(()) }