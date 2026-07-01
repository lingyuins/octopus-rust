use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::{ConnectInfo, Request, State},
    middleware::Next,
    response::Response,
};
use octopus_core::AppConfig;
use octopus_db::ops::audit;

/// Audit middleware for management write operations.
///
/// Resolves the client IP honoring `server.trusted_proxies`: only when the
/// direct peer is a configured trusted proxy do we trust `X-Forwarded-For`
/// (the left-most address). Otherwise the peer address itself is used.
pub async fn audit_middleware(
    State(config): State<Arc<AppConfig>>,
    req: Request,
    next: Next,
) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let ip = client_ip(&req, &config.server.trusted_proxies);
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let response = next.run(req).await;

    // Only audit write operations
    if matches!(method, axum::http::Method::POST | axum::http::Method::PUT | axum::http::Method::DELETE | axum::http::Method::PATCH) {
        let status = response.status();
        if status.is_success() || status.is_client_error() {
            // Extract user info from response extensions
            let user_id = response.extensions().get::<i32>().copied().unwrap_or(0);
            let username = response.extensions().get::<String>().map(|s| s.as_str()).unwrap_or("unknown");

            let action = format!("{} {}", method, path);
            let resource = path.split('/').nth(2).unwrap_or("unknown");

            let _ = audit::create_audit_log(
                user_id,
                username,
                &action,
                resource,
                None,
                None,
                &ip,
                &user_agent,
            ).await;
        }
    }

    response
}

/// Resolve the client IP, trusting `X-Forwarded-For` only when the direct
/// peer is listed in `trusted_proxies` (comma-separated CIDRs/IPs).
///
/// When `trusted_proxies` is empty, the peer address is used directly and
/// `X-Forwarded-For` is ignored — the correct behavior for a direct
/// (non-proxied) deployment.
fn client_ip(req: &Request, trusted_proxies: &str) -> String {
    let peer = req
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ci| ci.0);

    let peer_trusted = match peer {
        Some(addr) => is_trusted(addr.ip(), trusted_proxies),
        None => false,
    };

    if peer_trusted {
        if let Some(xff) = req.headers().get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
            // X-Forwarded-For: client, proxy1, proxy2 — the left-most entry
            // is the original client.
            if let Some(first) = xff.split(',').next() {
                let trimmed = first.trim();
                if !trimmed.is_empty() {
                    return trimmed.to_string();
                }
            }
        }
    }

    peer
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Check whether an IP matches any entry in the comma-separated
/// `trusted_proxies` list. Entries may be single IPs or CIDR ranges
/// (e.g. `172.17.0.0/16`). Non-parseable entries are silently skipped.
fn is_trusted(ip: std::net::IpAddr, trusted_proxies: &str) -> bool {
    if trusted_proxies.trim().is_empty() {
        return false;
    }
    trusted_proxies
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .any(|entry| ip_matches_entry(ip, entry))
}

/// Match an IP against a single entry (bare IP or CIDR).
fn ip_matches_entry(ip: std::net::IpAddr, entry: &str) -> bool {
    // CIDR notation: `addr/prefix`
    if let Some((net_str, prefix_str)) = entry.split_once('/') {
        let net: std::net::IpAddr = match net_str.parse() {
            Ok(n) => n,
            Err(_) => return false,
        };
        let prefix: u8 = match prefix_str.parse() {
            Ok(p) => p,
            Err(_) => return false,
        };
        return ip_in_cidr(ip, net, prefix);
    }

    // Bare IP — exact match.
    entry.parse().map(|parsed: std::net::IpAddr| parsed == ip).unwrap_or(false)
}

/// Test whether `ip` falls within the `net`/`prefix` CIDR.
fn ip_in_cidr(ip: std::net::IpAddr, net: std::net::IpAddr, prefix: u8) -> bool {
    match (ip, net) {
        (std::net::IpAddr::V4(a), std::net::IpAddr::V4(b)) => {
            if prefix > 32 {
                return false;
            }
            let mask = if prefix == 0 {
                0
            } else {
                !0u32 << (32 - prefix)
            };
            (u32::from(a) & mask) == (u32::from(b) & mask)
        }
        (std::net::IpAddr::V6(a), std::net::IpAddr::V6(b)) => {
            if prefix > 128 {
                return false;
            }
            let mask = if prefix == 0 {
                0
            } else {
                !0u128 << (128 - prefix)
            };
            (u128::from(a) & mask) == (u128::from(b) & mask)
        }
        _ => false, // v4 address vs v6 network or vice versa
    }
}
