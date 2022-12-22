use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE, sprite::Anchor};
use bevy_node_editor::{
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
            Visibility { is_visible: true },
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

// fn setup_material_preview(
//     mut commands: Commands,
//     mut materials: ResMut<Assets<PreviewMaterial>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut images: ResMut<Assets<Image>>,
//     query: Query<(Entity, &MaterialPreviewWidget), With<ReadyForPreview>>,
// ) {
//     for (entity, widget) in query.iter() {
//         let size = Extent3d {
//             width: widget.size.x as u32,
//             height: widget.size.y as u32,
//             ..default()
//         };
//         let mut image = Image {
//             texture_descriptor: TextureDescriptor {
//                 label: None,
//                 size,
//                 dimension: TextureDimension::D2,
//                 format: TextureFormat::Bgra8UnormSrgb,
//                 mip_level_count: 1,
//                 sample_count: 1,
//                 usage: TextureUsages::TEXTURE_BINDING
//                     | TextureUsages::COPY_DST
//                     | TextureUsages::RENDER_ATTACHMENT,
//             },
//             ..default()
//         };

//         image.resize(size);

//         let image_handle = images.add(image);
//         let mesh = meshes.add(Mesh::from(shape::UVSphere {
//             radius: 6.0,
//             ..default()
//         }));
//         let material = materials.add(PreviewMaterial::default());
//         let first_pass_layer = RenderLayers::layer(1);

//         let pbr_entity = commands
//             .spawn((
//                 MaterialMeshBundle {
//                     mesh,
//                     material,
//                     ..default()
//                 },
//                 PreviewMesh,
//                 first_pass_layer,
//             ))
//             .id();
//         let camera_entity = commands
//             .spawn((
//                 Camera3dBundle {
//                     camera: Camera {
//                         priority: -1,
//                         target: RenderTarget::Image(image_handle.clone()),
//                         ..default()
//                     },
//                     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0))
//                         .looking_at(Vec3::ZERO, Vec3::Y),
//                     ..default()
//                 },
//                 first_pass_layer,
//             ))
//             .id();
//         let render_to_entity = commands
//             .spawn(SpriteBundle {
//                 texture: image_handle.clone(),
//                 ..default()
//             })
//             .id();

//         commands
//             .entity(entity)
//             .push_children(&[pbr_entity, camera_entity, render_to_entity])
//             .remove::<ReadyForPreview>();
//     }
// }
