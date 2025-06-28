// Custom Bevy asset loader for loading tile maps from JSON files
use crate::prelude::*;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::asset::io::{Reader};

// Asset: A loaded tile map
// Derives Asset and TypePath so Bevy can manage it as an asset
// Derives Deserialize for JSON parsing
// Contains a 2D vector of strings representing the tile grid
#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct TileMapAsset {
    pub tiles: Vec<Vec<String>>,
}

// TileMapLoader: Implements Bevy's AssetLoader trait for loading TileMapAsset from JSON
// Default is derived for easy registration
#[derive(Default)]
pub struct TileMapLoader;

// Implement the AssetLoader trait for TileMapLoader
// Asset: The type produced (TileMapAsset)
// Settings: Loader settings ()
// Error: anyhow::Error for flexible error handling
impl AssetLoader for TileMapLoader {
    type Asset = TileMapAsset;
    type Settings = ();
    type Error = anyhow::Error;

    // The load function is async and returns a future
    // reader: Provides async access to the asset file's bytes
    // _settings: Loader settings ()
    // _context: LoadContext for asset registration ()
    // Returns: A future that resolves to Result<TileMapAsset, anyhow::Error>
    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _context: &mut LoadContext,
    ) -> impl std::future::Future<Output = Result<Self::Asset, Self::Error>> + Send {
        async move {
            // Read the entire file into a byte buffer asynchronously
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            // Deserialize the bytes into a TileMapAsset using serde_json
            let tilemap: TileMapAsset = serde_json::from_slice(&bytes)?;
            Ok(tilemap)
        }
    }

    // extensions: Returns the file extensions this loader supports
    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}