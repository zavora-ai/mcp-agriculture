mod agtech;
mod server;
mod types;

use adk_mcp_sdk::{HealthCheck, HealthStatus, ServerManifest};
use rmcp::{ServiceExt, transport::stdio};

use crate::agtech::AgTechClient;
use crate::server::AgricultureServer;

#[async_trait::async_trait]
impl HealthCheck for AgricultureServer {
    async fn check_health(&self) -> HealthStatus {
        match self.client.get("/fields").await {
            Ok(_) => HealthStatus { healthy: true, message: Some("Backend reachable".into()), latency_ms: Some(1) },
            Err(e) => HealthStatus { healthy: false, message: Some(format!("Backend unreachable: {e}")), latency_ms: None },
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let manifest = ServerManifest::from_file(std::path::Path::new("mcp-server.toml"))?;
    let errors = manifest.validate();
    if !errors.is_empty() {
        for e in &errors { tracing::error!("manifest: {e}"); }
        anyhow::bail!("invalid mcp-server.toml ({} error(s))", errors.len());
    }

    let base_url = std::env::var("AGTECH_API_URL").unwrap_or_else(|_| "http://localhost:7800/api/v1".into());
    let server = AgricultureServer { client: AgTechClient::new(base_url) };

    tracing::info!("{} v{} starting on stdio", manifest.display_name, manifest.version);
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
