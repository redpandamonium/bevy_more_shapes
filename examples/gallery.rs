use bevy::prelude::*;
use bevy::app::App;
use bevy::asset::{AssetServer, Assets};
use bevy::input::Input;
use bevy::math::{Quat, Rect, Vec3};
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::pbr::{AmbientLight, DirectionalLight, PbrBundle, StandardMaterial};
use bevy::render::mesh::shape::Icosphere;
use bevy::render::options::WgpuFeatures;
use bevy::render::options::WgpuOptions;
use bevy::render::primitives::Sphere;
use bevy::text::{Text, Text2dBundle, TextAlignment, TextStyle};
use bevy::ui::{AlignSelf, PositionType, Style, Val};
use bevy::DefaultPlugins;
use bevy_flycam::FlyCam;
use bevy_more_shapes::{Cone, Cylinder};
use crate::CursorIcon::Default;

fn spawn_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    // Start out without wireframes, but you can toggle them.
    wireframe_config.global = false;

    // Default cone
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Cone::default())),
        material: materials.add(StandardMaterial::from(Color::GOLD)),
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..Default::default()
    });

    // Default cylinder
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Cylinder::default())),
        material: materials.add(StandardMaterial::from(Color::CRIMSON)),
        transform: Transform::from_xyz(2.0, 0.0, 5.0),
        ..Default::default()
    });

    // Taller regular cylinder
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Cylinder::new_regular(2.2, 0.5, 16))),
        material: materials.add(StandardMaterial::from(Color::FUCHSIA)),
        transform: Transform::from_xyz(2.0, 0.0, 7.0),
        ..Default::default()
    });

    // Irregular cylinder
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Cylinder {
            height: 1.0,
            radius_bottom: 0.6,
            radius_top: 0.2,
            subdivisions: 40,
        })),
        material: materials.add(StandardMaterial::from(Color::ORANGE_RED)),
        transform: Transform::from_xyz(2.0, 0.0, 9.0),
        ..Default::default()
    });

    // Sun
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 15000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(100.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Ambient light
    ambient_light.brightness = 0.2;
}

fn spawn_info_text(mut commands: Commands, asset_server: Res<AssetServer>) {

    // Camera for the UI layer
    commands.spawn_bundle(UiCameraBundle::default());

    // Show text that presents the controls
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            "WASD + Mouse movement\nSpace Up, LShift Down\nESC toggle input grab\nX toggle wireframes",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                font_size: 15.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Left
            }
        ),
        ..Default::default()
    });
}

fn toggle_wireframe_system(
    keys: Res<Input<KeyCode>>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    if keys.just_pressed(KeyCode::X) {
        wireframe_config.global = !wireframe_config.global;
    }
}

// The camera plugin places the camera with a strange transform. We center it at the origin looking along the Z axis.
fn fix_initial_camera_position(mut query: Query<&mut Transform, With<FlyCam>>) {
    let new_xform = Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y);
    for mut xform in query.iter_mut() {
        xform.rotation = new_xform.rotation;
        xform.translation = new_xform.translation;
        xform.scale = new_xform.scale;
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WgpuOptions {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_flycam::PlayerPlugin)
        .add_plugin(WireframePlugin)
        .add_startup_system(spawn_shapes)
        .add_startup_system(spawn_info_text)
        .add_startup_system_to_stage(StartupStage::PostStartup, fix_initial_camera_position)
        .add_system(toggle_wireframe_system)
        .run();
}
