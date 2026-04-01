use shipyard::Component;
use bevy::{asset::RenderAssetUsages, prelude::*};
use bevy_ufbx::{Fbx, FbxLoaderSettings};

/// Fbx Importer
/// `Note: pub <params> is made if you importer with settings`
#[derive(Component)]
pub struct FbxSceneInfo
where
    Self: 'static
{
    /// Used only by importer with settings
    pub convert_coordinates:    bool,
    /// Used only by importer with settings
    pub include_source:         bool,
    /// Used only by importer with settings
    pub load_cameras:           bool,
    /// Used only by importer with settings
    pub load_lights:            bool,
    /// Used only by importer with settings
    pub load_materials:         RenderAssetUsages,
    /// Used only by importer with settings
    pub load_meshes:            RenderAssetUsages,
}

impl FbxSceneInfo {
    
    pub fn new() -> Self {
        Self {
            convert_coordinates: false,
            include_source: false,
            load_cameras: false,
            load_lights: false,
            load_materials: RenderAssetUsages::empty(),
            load_meshes: RenderAssetUsages::empty()
        }
    }

    /// Importer single file, with #Scene0 or Mesh#0, or Animation#0
    pub fn fbx_anime_object(
        commands: &mut Commands,
        asset_server: Res<AssetServer>,
        path: &'static str
    ) {
        if path.is_empty() {
            eprintln!("Fbx Path cannot be null");
        }

        commands.spawn(SceneRoot (
            asset_server.load(path),
        ));
    }

    /// Importer single file with settings
    pub fn fbx_anime_object_with_settings(
        self,
        asset_server: Res<AssetServer>,
        path: &'static str,
    ) {
        if path.is_empty() {
            eprintln!("Fbx Path cannot be null");
        }

        let _ = asset_server.load_with_settings::<Fbx, FbxLoaderSettings>(
            path, move |s| {
                
                s.convert_coordinates   = self.convert_coordinates;
                s.include_source        = self.include_source;
                s.load_cameras          = self.load_cameras;
                s.load_lights           = self.load_lights;
                s.load_materials        = self.load_materials;
                s.load_meshes           = self.load_meshes;
        });
    }
}