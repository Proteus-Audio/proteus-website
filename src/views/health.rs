use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct HealthStatus {
    pub ok: bool,
    pub message: String,
    pub timestamp: String,
}

#[get("/api/health")]
#[allow(dead_code)]
pub async fn healthcheck() -> Result<HealthStatus> {
    #[cfg(feature = "server")]
    {
        let message = "ok".to_string();

        return Ok(HealthStatus {
            ok: true,
            message,
            timestamp: chrono::Utc::now().to_rfc3339(),
        });
    }

    #[cfg(not(feature = "server"))]
    Ok(HealthStatus {
        ok: false,
        message: "healthcheck unavailable on client build".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}
