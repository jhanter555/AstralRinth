use crate::state::CachedEntry;
use crate::State;
pub use daedalus::minecraft::VersionManifest;
pub use daedalus::modded::Manifest;

#[tracing::instrument]
pub async fn get_minecraft_versions() -> crate::Result<VersionManifest> {
    let state = State::get().await?;
    let minecraft_versions = CachedEntry::get_minecraft_manifest(
        None,
        &state.pool,
        &state.api_semaphore,
    )
    .await?
    .ok_or_else(|| {
        crate::ErrorKind::NoValueFor("minecraft versions".to_string())
    })?;

    Ok(minecraft_versions)
}

// #[tracing::instrument]
pub async fn get_loader_versions(loader: &str) -> crate::Result<Manifest> {
    let state = State::get().await?;
    let loaders = CachedEntry::get_loader_manifest(
        loader,
        None,
        &state.pool,
        &state.api_semaphore,
    )
    .await;

    let loaders = match loaders {
        Ok(Some(loaders)) => loaders,
        Ok(None) if loader == "quilt" => {
            tracing::warn!(
                "Quilt loader metadata is unavailable; skipping optional metadata"
            );
            return Ok(Manifest {
                game_versions: Vec::new(),
            });
        }
        Err(error) if loader == "quilt" => {
            tracing::warn!(
                error = %error,
                "Unable to fetch Quilt loader metadata; skipping optional metadata"
            );
            return Ok(Manifest {
                game_versions: Vec::new(),
            });
        }
        Ok(None) => {
            return Err(crate::ErrorKind::NoValueFor(format!(
                "{} loader versions",
                loader
            ))
            .into());
        }
        Err(error) => return Err(error),
    };

    Ok(loaders.manifest)
}
