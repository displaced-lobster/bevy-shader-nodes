use bevy::{
    asset::load_internal_asset,
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        render_resource::{
            AsBindGroup,
            Extent3d,
            ShaderRef,
            TextureDescriptor,
            TextureDimension,
            TextureFormat,
            TextureUsages,
        },
        view::RenderLayers,
    },
};
use bevy_node_editor::{
    assets::DefaultAssets,
    widget::{Widget, WidgetPlugin},
    NodeEvent,
    SlotWidget,
};

use crate::shader::ShaderNodes;

const PREVIEW_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 452747839445324907);

#[derive(Default)]
pub struct MaterialPreviewWidgetPlugin;

impl Plugin for MaterialPreviewWidgetPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PREVIEW_SHADER_HANDLE,
            "preview.wgsl",
            Shader::from_wgsl
        );
        app.add_plugin(WidgetPlugin::<ShaderNodes, MaterialPreviewWidget>::default())
            .add_plugin(MaterialPlugin::<PreviewMaterial>::default())
            .add_system(rotate_preview_mesh)
            .add_system(setup_material_preview)
            .add_system(update_preview_material);
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct MaterialPreviewWidget {
    pub size: Vec2,
}

#[derive(Component)]
struct ReadyForPreview;

impl Widget for MaterialPreviewWidget {
    type WidgetValue = ();

    fn build(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        area: Vec2,
        _assets: &Res<DefaultAssets>,
    ) {
        self.size = area;

        commands.entity(entity).insert(ReadyForPreview);
    }

    fn size(&self) -> Vec2 {
        self.size
    }
}

impl SlotWidget<Self, MaterialPreviewWidget> for ShaderNodes {
    fn get_widget(&self) -> Option<MaterialPreviewWidget> {
        match self {
            ShaderNodes::MaterialPreview => Some(MaterialPreviewWidget::default()),
            _ => None,
        }
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default)]
#[uuid = "038b1fc4-f4ff-4735-8442-ff561df3fbf2"]
pub struct PreviewMaterial {
    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
}

impl Material for PreviewMaterial {
    fn fragment_shader() -> ShaderRef {
        PREVIEW_SHADER_HANDLE.typed().into()
    }
}

#[derive(Component)]
struct PreviewMesh;

fn rotate_preview_mesh(time: Res<Time>, mut query: Query<&mut Transform, With<PreviewMesh>>) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(1.3 * time.delta_seconds());
    }
}

fn setup_material_preview(
    mut commands: Commands,
    mut materials: ResMut<Assets<PreviewMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<(Entity, &MaterialPreviewWidget), With<ReadyForPreview>>,
) {
    for (entity, widget) in query.iter() {
        let size = Extent3d {
            width: widget.size.x as u32,
            height: widget.size.y as u32,
            ..default()
        };
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
            },
            ..default()
        };

        image.resize(size);

        let image_handle = images.add(image);
        let mesh = meshes.add(Mesh::from(shape::UVSphere {
            radius: 6.0,
            ..default()
        }));
        let material = materials.add(PreviewMaterial::default());
        let first_pass_layer = RenderLayers::layer(1);

        let pbr_entity = commands
            .spawn((
                MaterialMeshBundle {
                    mesh,
                    material,
                    ..default()
                },
                PreviewMesh,
                first_pass_layer,
            ))
            .id();
        let camera_entity = commands
            .spawn((
                Camera3dBundle {
                    camera: Camera {
                        priority: -1,
                        target: RenderTarget::Image(image_handle.clone()),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0))
                        .looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                },
                first_pass_layer,
            ))
            .id();
        let render_to_entity = commands
            .spawn(SpriteBundle {
                texture: image_handle.clone(),
                ..default()
            })
            .id();

        commands
            .entity(entity)
            .push_children(&[pbr_entity, camera_entity, render_to_entity])
            .remove::<ReadyForPreview>();
    }
}

fn update_preview_material(
    mut shaders: ResMut<Assets<Shader>>,
    mut ev_node: EventReader<NodeEvent<ShaderNodes>>,
) {
    if let Some(ev) = ev_node.iter().next() {
        if let NodeEvent::Resolved((_, value)) = ev {
            let shader_str = value.build().unwrap();
            let shader_handle = shaders
                .get_mut(&PREVIEW_SHADER_HANDLE.typed().into())
                .unwrap();

            *shader_handle = Shader::from_wgsl(shader_str);
        }
    }
}
