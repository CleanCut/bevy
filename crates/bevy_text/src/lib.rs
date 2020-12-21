mod draw;
mod error;
mod font;
mod font_atlas;
mod font_atlas_set;
mod font_loader;
mod glyph_brush;
mod pipeline;
mod text2d;

pub use draw::*;
pub use error::*;
pub use font::*;
pub use font_atlas::*;
pub use font_atlas_set::*;
pub use font_loader::*;
pub use glyph_brush::*;
pub use pipeline::*;
pub use text2d::*;

pub mod prelude {
    pub use crate::{Font, Text, Text2dBundle, TextAlignment, TextError, TextStyle};
    pub use glyph_brush_layout::{HorizontalAlign, VerticalAlign};
}

pub mod stage {
    pub const TEXT2D: &str = "text2d";
}

use bevy_app::prelude::*;
use bevy_asset::AddAsset;
use bevy_ecs::{Entity, IntoSystem, SystemStage};

pub type DefaultTextPipeline = TextPipeline<Entity>;

#[derive(Default)]
pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_before(
            bevy_app::stage::POST_UPDATE,
            stage::TEXT2D,
            SystemStage::parallel(),
        )
        .add_asset::<Font>()
        .add_asset::<FontAtlasSet>()
        .init_asset_loader::<FontLoader>()
        .add_resource(DefaultTextPipeline::default())
        .add_system_to_stage(stage::TEXT2D, text2d_system.system())
        .add_system_to_stage(
            bevy_render::stage::DRAW,
            text2d::draw_text2d_system.system(),
        );
    }
}
