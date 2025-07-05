use crate::prelude::*;
use crate::plugins::TileMapAsset;

// This resource is used to request a map to be loaded
// It holds the handle to the TileMapAsset that needs to be loaded
#[derive(Resource, Default)]
pub struct MapLoadState {
    pub handle: Option<Handle<TileMapAsset>>,
}

// This resource holds the name of the currently loaded map
#[derive(Resource, Debug, Clone, Default)]
pub struct LoadedMap(pub String);

// This resource indicates whether the map has been loaded successfully
#[derive(Resource, Default)]
pub struct MapHasLoaded(pub bool);
