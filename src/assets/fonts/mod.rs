use bevy::{asset::embedded_asset, prelude::*};
use std::path::Path;

pub(crate) struct BuiltInFontsPlugin;

impl Plugin for BuiltInFontsPlugin {
    fn build(&self, app: &mut App) {
        let path = Path::new("src/assets");
        embedded_asset!(app, path, "FiraSans-Bold.ttf");
        embedded_asset!(app, path, "FiraSans-BoldItalic.ttf");
        embedded_asset!(app, path, "FiraSans-Italic.ttf");
        embedded_asset!(app, path, "FiraSans-Medium.ttf");
        embedded_asset!(app, path, "FiraSans-MediumItalic.ttf");
        embedded_asset!(app, path, "FiraSans-Regular.ttf");
        embedded_asset!(app, path, "FiraSansCondensed-Bold.ttf");
        embedded_asset!(app, path, "FiraSansCondensed-BoldItalic.ttf");
        embedded_asset!(app, path, "FiraSansCondensed-Italic.ttf");
        embedded_asset!(app, path, "FiraSansCondensed-Regular.ttf");
        embedded_asset!(app, path, "MaterialIcons-Regular.ttf");
    }
}
