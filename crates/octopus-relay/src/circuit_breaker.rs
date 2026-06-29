use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;
use once_cell::sync::Lazy;

static BREAKER_STATE: Lazy<Mutex<HashMap<(i32, i32, String), BreakerState>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone)]
struct BreakerState {
    pub state: CircuitState,
    pub fail_count: i32,
    pub last_fail: Option<Instant>,
    pub opened_at: Option<Instant>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState { Closed, Open, HalfOpen }

const FAIL_THRESHOLD: i32 = 5;
const COOLDOWN_SECS: u64 = 30;

pub fn is_open(channel_id: i32, key_id: i32, model: &str) -> bool {
    let key = (channel_id, key_id, model.to_string());
    if let Ok(state) = BREAKER_STATE.lock() {
        if let Some(bs) = state.get(&key) {
            if bs.state == CircuitState::Open {
                if let Some(opened) = bs.opened_at {
                    if opened.elapsed().as_secs() > COOLDOWN_SECS {
                        return false; // Transition to half-open
                    }
                }
                return true;
            }
        }
    }
    false
}

pub fn record_failure(channel_id: i32, key_id: i32, model: &str) {
    let key = (channel_id, key_id, model.to_string());
    if let Ok(mut state) = BREAKER_STATE.lock() {
        let entry = state.entry(key).or_insert_with(|| BreakerState {
            state: CircuitState::Closed,
            fail_count: 0,
            last_fail: None,
            opened_at: None,
        });
        entry.fail_count += 1;
        entry.last_fail = Some(Instant::now());
        if entry.fail_count >= FAIL_THRESHOLD {
            entry.state = CircuitState::Open;
            entry.opened_at = Some(Instant::now());
        }
    }
}

pub fn record_success(channel_id: i32, key_id: i32, model: &str) {
    let key = (channel_id, key_id, model.to_string());
    if let Ok(mut state) = BREAKER_STATE.lock() {
        if let Some(entry) = state.get_mut(&key) {
            entry.state = CircuitState::Closed;
            entry.fail_count = 0;
            entry.opened_at = None;
        }
    }
}