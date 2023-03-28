use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE, sprite::Anchor};
use bevy_flow_node::{
    assets::DefaultAssets,
    widget::{Widget, WidgetPlugin},
    SlotWidget,
};
use nfd::Response;

use crate::shader::ShaderNodes;

use super::material_preview_widget::PreviewMaterial;

#[derive(Default)]
pub struct TextureWidgetPlugin;

impl Plugin for TextureWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WidgetPlugin::<ShaderNodes, TextureWidget>::default())
            .add_system(load_texture);
    }
}

#[derive(Component, Clone, Default)]
pub struct TextureWidget {
    pub size: Vec2,
    pub texture: Option<Handle<Image>>,
    pub to_load: Option<String>,
}

impl Widget for TextureWidget {
    type WidgetValue = ();

    fn build(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        area: Vec2,
        _assets: &Res<DefaultAssets>,
    ) {
        self.size = area;

        commands.entity(entity).insert((
            Sprite {
                custom_size: Some(self.size),
                anchor: Anchor::Center,
                ..default()
            },
            Visibility::Inherited,
            ComputedVisibility::default(),
            DEFAULT_IMAGE_HANDLE.typed::<Image>(),
        ));
    }

    fn can_click(&self) -> bool {
        true
    }

    fn focus(&mut self) {
        let result = nfd::open_file_dialog(Some("png"), None).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        match result {
            Response::Okay(file_path) => {
                self.to_load = Some(file_path);
            }
            Response::OkayMultiple(_) => {}
            Response::Cancel => {}
        }
    }

    fn size(&self) -> Vec2 {
        self.size
    }
}

impl SlotWidget<Self, TextureWidget> for ShaderNodes {
    fn get_widget(&self) -> Option<TextureWidget> {
        match self {
            ShaderNodes::Texture => Some(TextureWidget::default()),
            _ => None,
        }
    }
}

fn load_texture(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut materials: ResMut<Assets<PreviewMaterial>>,
    mut texture_widgets: Query<(Entity, &mut TextureWidget)>,
) {
    let mut materials = materials.iter_mut().collect::<Vec<_>>();

    for (entity, mut widget) in texture_widgets.iter_mut() {
        if let Some(path) = widget.to_load.take() {
            let handle: Handle<Image> = server.load(path);

            for mut material in &mut materials {
                material.1.texture = Some(handle.clone());
            }

            commands.entity(entity).insert(handle);
        }
    }
}
