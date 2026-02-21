//! Optional Vaultwarden secret fetching.
//!
//! When the `vault` feature is enabled and `VAULT_API_URL` + `VAULT_API_KEY`
//! are set, fetches secrets from a Vaultwarden-API instance and injects them
//! into the process environment (only for vars not already set).
//!
//! Use `VAULT_SECRET_PREFIX` to namespace secrets in Vaultwarden (e.g. prefix
//! `RIVVO_` fetches `RIVVO_DATABASE_URL` from the vault but sets `DATABASE_URL`).

// ── Feature-disabled stub ──────────────────────────────────────────

#[cfg(not(feature = "vault"))]
pub async fn fetch_secrets() {
    // Vault feature not compiled in — nothing to do.
}

// ── Feature-enabled implementation ─────────────────────────────────

#[cfg(feature = "vault")]
use serde::Deserialize;

#[cfg(feature = "vault")]
#[derive(Debug, Deserialize)]
struct VaultResponse {
    #[allow(dead_code)]
    name: String,
    value: String,
}

#[cfg(feature = "vault")]
const SECRET_KEYS: &[&str] = &["DATABASE_URL", "JWT_SECRET"];

#[cfg(feature = "vault")]
struct SecretMapping {
    env_key: &'static str,
    vault_key: String,
}

#[cfg(feature = "vault")]
fn build_secret_mappings(prefix: &str) -> Vec<SecretMapping> {
    SECRET_KEYS
        .iter()
        .map(|&key| SecretMapping {
            env_key: key,
            vault_key: format!("{prefix}{key}"),
        })
        .collect()
}

#[cfg(feature = "vault")]
pub async fn fetch_secrets() {
    let api_url = match std::env::var("VAULT_API_URL") {
        Ok(url) => url.trim_end_matches('/').to_string(),
        Err(_) => {
            tracing::debug!("Vault not configured, skipping");
            return;
        }
    };

    let api_key = match std::env::var("VAULT_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            tracing::debug!("Vault not configured, skipping");
            return;
        }
    };

    let prefix = std::env::var("VAULT_SECRET_PREFIX").unwrap_or_default();

    tracing::info!("Fetching secrets from Vaultwarden API at {api_url}");

    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Failed to create HTTP client for vault: {e}");
            return;
        }
    };

    let mappings = build_secret_mappings(&prefix);

    for mapping in &mappings {
        if std::env::var(mapping.env_key).is_ok() {
            tracing::warn!(
                "Secret {} already set via env, skipping vault",
                mapping.env_key
            );
            continue;
        }

        let url = format!("{api_url}/secret/{}", mapping.vault_key);

        match client.get(&url).bearer_auth(&api_key).send().await {
            Ok(response) => {
                if !response.status().is_success() {
                    tracing::warn!(
                        "Failed to fetch secret {} from vault: HTTP {}",
                        mapping.vault_key,
                        response.status()
                    );
                    continue;
                }

                match response.json::<VaultResponse>().await {
                    Ok(secret) => {
                        // SAFETY: Called during single-threaded startup before any
                        // worker threads or connection pools are created.
                        unsafe {
                            std::env::set_var(mapping.env_key, &secret.value);
                        }
                        tracing::info!("Loaded secret {} from vault", mapping.env_key);
                    }
                    Err(e) => {
                        tracing::warn!(
                            "Failed to fetch secret {} from vault: {e}",
                            mapping.vault_key
                        );
                    }
                }
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to fetch secret {} from vault: {e}",
                    mapping.vault_key
                );
            }
        }
    }
}

// ── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
#[cfg(feature = "vault")]
mod tests {
    use super::*;

    #[test]
    fn build_secret_mappings_with_prefix() {
        let mappings = build_secret_mappings("RIVVO_");
        assert_eq!(mappings.len(), 2);
        assert_eq!(mappings[0].env_key, "DATABASE_URL");
        assert_eq!(mappings[0].vault_key, "RIVVO_DATABASE_URL");
        assert_eq!(mappings[1].env_key, "JWT_SECRET");
        assert_eq!(mappings[1].vault_key, "RIVVO_JWT_SECRET");
    }

    #[test]
    fn build_secret_mappings_empty_prefix() {
        let mappings = build_secret_mappings("");
        assert_eq!(mappings[0].vault_key, "DATABASE_URL");
        assert_eq!(mappings[1].vault_key, "JWT_SECRET");
    }

    #[test]
    fn vault_response_deserialization() {
        let json = r#"{"name": "MY_SECRET", "value": "secret_value_123"}"#;
        let resp: VaultResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.name, "MY_SECRET");
        assert_eq!(resp.value, "secret_value_123");
    }

    #[test]
    fn vault_response_rejects_missing_value() {
        let json = r#"{"name": "MY_SECRET"}"#;
        assert!(serde_json::from_str::<VaultResponse>(json).is_err());
    }
}
