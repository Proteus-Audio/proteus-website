use dioxus::prelude::*;

use super::types::DownloadManifest;

#[cfg(feature = "server")]
use super::types::DownloadAsset;

pub(crate) async fn fetch_download_manifest(
    manifest_url: &str,
    github_latest_api_url: &str,
) -> Result<DownloadManifest, String> {
    fetch_download_manifest_server(manifest_url.to_string(), github_latest_api_url.to_string())
        .await
        .map_err(|err| format!("Failed to fetch manifest: {err}"))
}

#[post("/api/download-manifest")]
async fn fetch_download_manifest_server(
    manifest_url: String,
    github_latest_api_url: String,
) -> Result<DownloadManifest> {
    #[derive(serde::Deserialize)]
    struct ManifestAssetWire {
        name: Option<String>,
        download_url: Option<String>,
    }

    #[derive(serde::Deserialize)]
    struct ManifestWire {
        version: Option<String>,
        release_url: Option<String>,
        assets: Option<Vec<ManifestAssetWire>>,
    }

    #[derive(serde::Deserialize)]
    struct GithubAssetWire {
        name: Option<String>,
        browser_download_url: Option<String>,
    }

    #[derive(serde::Deserialize)]
    struct GithubReleaseWire {
        tag_name: Option<String>,
        html_url: Option<String>,
        assets: Option<Vec<GithubAssetWire>>,
    }

    fn clean_assets(assets: Vec<(Option<String>, Option<String>)>) -> Vec<DownloadAsset> {
        assets
            .into_iter()
            .filter_map(|(name, url)| match (name, url) {
                (Some(name), Some(url)) if !name.is_empty() && !url.is_empty() => {
                    Some(DownloadAsset { name, url })
                }
                _ => None,
            })
            .collect()
    }

    fn version_from_tag(tag: &str) -> String {
        if let Some((_, rhs)) = tag.rsplit_once("-v") {
            rhs.to_string()
        } else {
            tag.to_string()
        }
    }

    #[cfg(feature = "server")]
    {
        // Prefer the project-provided downloads.json first. If it is unavailable or malformed,
        // fall back to the GitHub release API so downloads still work.
        let client = reqwest::Client::builder()
            .user_agent("proteus-website/0.1")
            .build()
            .map_err(|err| ServerFnError::new(format!("failed to build http client: {err}")))?;

        let manifest_attempt = client.get(&manifest_url).send().await;
        if let Ok(response) = manifest_attempt {
            if response.status().is_success() {
                if let Ok(raw) = response.json::<ManifestWire>().await {
                    let assets = clean_assets(
                        raw.assets
                            .unwrap_or_default()
                            .into_iter()
                            .map(|asset| (asset.name, asset.download_url))
                            .collect(),
                    );

                    return Ok(DownloadManifest {
                        version: raw.version.unwrap_or_default(),
                        release_url: raw.release_url.unwrap_or_default(),
                        assets,
                    });
                }
            }
        }

        let response = client
            .get(&github_latest_api_url)
            .header("Accept", "application/vnd.github+json")
            .send()
            .await
            .map_err(|err| ServerFnError::new(format!("github api request failed: {err}")))?;

        if !response.status().is_success() {
            return Err(ServerFnError::new(format!(
                "github api returned status {}",
                response.status()
            ))
            .into());
        }

        let raw = response.json::<GithubReleaseWire>().await.map_err(|err| {
            ServerFnError::new(format!("failed to parse github api response: {err}"))
        })?;

        let assets = clean_assets(
            raw.assets
                .unwrap_or_default()
                .into_iter()
                .map(|asset| (asset.name, asset.browser_download_url))
                .collect(),
        );

        return Ok(DownloadManifest {
            version: raw
                .tag_name
                .as_deref()
                .map(version_from_tag)
                .unwrap_or_default(),
            release_url: raw.html_url.unwrap_or_default(),
            assets,
        });
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = manifest_url;
        let _ = github_latest_api_url;
        Err(ServerFnError::new(
            "server feature is not enabled; cannot fetch manifests from backend",
        )
        .into())
    }
}
