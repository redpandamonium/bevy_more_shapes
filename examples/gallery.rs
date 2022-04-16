use std::fs;
use bevy::prelude::*;
use bevy::app::App;
use bevy::asset::{AssetServer, Assets};
use bevy::input::Input;
use bevy::math::{DVec2, Rect, Vec3};
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::pbr::{AmbientLight, DirectionalLight, PbrBundle, StandardMaterial};
// use bevy::render::settings::{ WgpuSettings, WgpuFeatures }; bevy 0.7
use bevy::render::options::{ WgpuFeatures, WgpuOptions }; // bevy 0.6
use bevy::text::{Text, Text2dBundle, TextAlignment, TextStyle};
use bevy::ui::{AlignSelf, PositionType, Style, Val};
use bevy::DefaultPlugins;
use bevy_more_shapes::{Cone, Cylinder};
use bevy::render::mesh::shape::Icosphere;
use bevy::render::render_resource::Texture;
use bevy::render::texture::ImageType;
use bevy::window::WindowFocused;
use smooth_bevy_cameras::controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin};

// Spawns the actual gallery of shapes. Spawns a row for each type in z+ direction.
fn spawn_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut ambient_light: ResMut<AmbientLight>,
    asset_server: Res<AssetServer>
) {
    let checkerboard_texture = asset_server.load("textures/checker-map_tho.png");

    // Start out without wireframes, but you can toggle them.
    wireframe_config.global = false;

    // Comparison: Builtin sphere
    let mut sphere = Icosphere::default();
    sphere.radius = 0.5;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(sphere)),
        material: materials.add(StandardMaterial::from(Color::BISQUE)),
        transform: Transform::from_xyz(-2.0, 0.0, 5.0),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(sphere)),
        material: materials.add(StandardMaterial::from(checkerboard_texture.clone())),
        transform: Transform::from_xyz(-2.0, 0.0, 7.0),
        ..Default::default()
    });

    // Default cone
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Cone::default())),
        material: materials.add(StandardMaterial::from(Color::GOLD)),
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..Default::default()
    });

    // Textured cone
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Cone::default())),
        material: materials.add(StandardMaterial::from(checkerboard_texture.clone())),
        transform: Transform::from_xyz(0.0, 0.0, 7.0),
        ..Default::default()
    });

    // Textured cylinder
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Cylinder::default())),
        material: materials.add(StandardMaterial::from(checkerboard_texture.clone())),
        transform: Transform::from_xyz(2.0, 0.0, 13.0),
        ..Default::default()
    });

    // Tiny cylinder
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Cylinder {
            height: 1.0,
            radius_bottom: 0.5,
            radius_top: 0.5,
            subdivisions: 3,
        })),
        material: materials.add(StandardMaterial::from(Color::OLIVE)),
        transform: Transform::from_xyz(2.0, 0.0, 11.0),
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

// Spawn a UI layer with the controls and other useful info.
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

// Spawn and configure the camera.
fn spawn_camera(
    mut commands: Commands,
) {

    let mut controller = FpsCameraController::default();
    controller.enabled = false; // we have a system that takes care of this, so disable it to prevent first-frame weirdness
    controller.smoothing_weight = 0.5;
    controller.translate_sensitivity = 0.4;

    commands.spawn_bundle(FpsCameraBundle::new(
        controller,
        PerspectiveCameraBundle::default(),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0)
    ));
}

// Toggles global wireframe mode (all meshes) on a key press.
fn toggle_wireframe_system(
    keys: Res<Input<KeyCode>>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    if keys.just_pressed(KeyCode::X) {
        wireframe_config.global = !wireframe_config.global;
    }
}

pub struct MouseLockPlugin;

pub struct MouseLock {
    /// If the lock is engaged the input will be grabbed and the cursor hidden.
    pub lock: bool,
    /// The plugin comes with a default toggle system. If you implement your own logic when to lock and unlock, you need to override it.
    pub override_default_lock_system: bool,
    // Keep track of where the mouse was when it entered, so we can restore its position later.
    last_position: Option<Vec2>,
    // Keep track of what the last lock status was, so we can detect when we need to toggle.
    last_lock: bool,
}

impl MouseLock {
    pub fn new(initially_locked: bool, override_default_lock_system: bool) -> Self {
        Self {
            lock: initially_locked,
            override_default_lock_system,
            last_position: None,
            last_lock: false,
        }
    }
}

impl Default for MouseLock {
    fn default() -> Self {
        MouseLock {
            lock: false,
            override_default_lock_system: false,
            last_position: None,
            last_lock: false,
        }
    }
}

// Determines the correct lock state based on inputs. ESC to drop focus, click on the window to regain it.
fn automatic_lock_system(
    mut lock: ResMut<MouseLock>,
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    // Automatic locking overridden, do nothing.
    if lock.override_default_lock_system {
        return;
    }

    // Check for unlock
    if lock.last_lock {
        if keys.just_pressed(KeyCode::Escape) {
            lock.lock = false;
        }
    }
    else {
        // The current focus state is the last focus event.
        if mouse.just_pressed(MouseButton::Left) {
            lock.lock = true;
        }
    }
}

// Observed the MouseLock status and updates the actual window config according to the status.
fn update_lock(
    mut lock: ResMut<MouseLock>,
    mut windows: ResMut<Windows>,
) {
    // Change detected
    if lock.lock != lock.last_lock {

        let window = windows.get_primary_mut().unwrap();

        // Locking, save position
        if lock.lock {
            lock.last_position = window.cursor_position();
        }

        // Set display modes
        window.set_cursor_lock_mode(lock.lock);
        window.set_cursor_visibility(!lock.lock);

        // Unlocked, restore cursor position
        if !lock.lock {
            // Try to restore cursor position
            if let Some(pos) = lock.last_position {
                window.set_cursor_position(pos);
            }
        }

        // Update done
        lock.last_lock = lock.lock;
    }
}

impl Plugin for MouseLockPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add default config
            .insert_resource(MouseLock::default())
            .add_system(automatic_lock_system)
            .add_system_to_stage(CoreStage::PostUpdate, update_lock);
    }
}

fn lock_camera(
    mouse_lock: Res<MouseLock>,
    mut camera_controllers: Query<&mut FpsCameraController>,
) {
    // When the cursor is locked, we want the camera to be active. Otherwise keep it still.
    camera_controllers.for_each_mut(|mut cam| cam.enabled = mouse_lock.lock);
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        // Wireframes require line mode
        .insert_resource(WgpuOptions {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(smooth_bevy_cameras::LookTransformPlugin)
        .add_plugin(FpsCameraPlugin::default())
        .add_plugin(WireframePlugin)
        .add_plugin(MouseLockPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_shapes)
        .add_startup_system(spawn_info_text)
        .add_system(toggle_wireframe_system)
        .add_system(lock_camera)
        .run();
}
