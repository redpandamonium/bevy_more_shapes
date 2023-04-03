use bevy::app::App;
use bevy::asset::{AssetServer, Assets};
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::pbr::{AmbientLight, DirectionalLight, NotShadowCaster, PbrBundle, StandardMaterial};
use bevy::prelude::*;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::text::{Text, TextAlignment, TextStyle};
use bevy::ui::{AlignSelf, PositionType, Style, Val};
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::DefaultPlugins;
use bevy::render::RenderPlugin;
use bevy_normal_material::prelude::{NormalMaterial, NormalMaterialPlugin};
use bevy_more_shapes::torus::Torus;
use bevy_more_shapes::{Cone, Cylinder, Grid, Polygon};
use smooth_bevy_cameras::controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin};
use bevy_more_shapes::tube::{CurveFunction, Curve};

struct WaveFunction;

impl CurveFunction for WaveFunction {
    fn eval_at(&self, t: f32) -> Vec3 {
        Vec3::new(
            -f32::sin(t * std::f32::consts::PI * 2.0) * 0.2,
            t,
            f32::sin(t * std::f32::consts::PI * 2.0) * 0.2
        )
    }
}

struct Knot {
    rotation_winds: u32,
    circle_winds: u32,
}

impl CurveFunction for Knot {
    fn eval_at(&self, mut t: f32) -> Vec3 {

        t *= std::f32::consts::TAU * 2.0;
        let cu = f32::cos(t);
        let su = f32::sin(t);
        let quop = self.circle_winds as f32 / self.rotation_winds as f32 * t;
        let cs = f32::cos(quop);

        Vec3::new(
            (2.0 + cs) * 0.5 * cu,
            (2.0 + cs) * su * 0.5,
            f32::sin(quop) * 0.5,
        )
    }
}

// Spawns the actual gallery of shapes. Spawns a row for each type in z+ direction.
fn spawn_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut normal_materials: ResMut<Assets<NormalMaterial>>,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut ambient_light: ResMut<AmbientLight>,
    asset_server: Res<AssetServer>,
) {
    let checkerboard_texture = asset_server.load("textures/checkerboard_1024x1024.png");

    // Start out without wireframes, but you can toggle them.
    wireframe_config.global = false;

    /*
    // Comparison: Builtin sphere
    let mut sphere = Icosphere::default();
    sphere.radius = 0.5;
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::try_from(sphere).unwrap()),
        material: materials.add(StandardMaterial::from(Color::BISQUE)),
        transform: Transform::from_xyz(-2.0, 0.0, 5.0),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::try_from(sphere).unwrap()),
        material: materials.add(StandardMaterial::from(checkerboard_texture.clone())),
        transform: Transform::from_xyz(-2.0, 0.0, 7.0),
        ..Default::default()
    });

     */

    // Default cone
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(Cone::default())),
        material: normal_materials.add(NormalMaterial::default()),
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..Default::default()
    });

    // Big cone
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cone {
            radius: 0.8,
            height: 2.0,
            segments: 32,
        })),
        material: materials.add(StandardMaterial::from(Color::YELLOW_GREEN)),
        transform: Transform::from_xyz(0.0, 0.0, 7.0),
        ..Default::default()
    });

    // Small cone
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cone {
            radius: 0.8,
            height: 0.3,
            segments: 32,
        })),
        material: materials.add(StandardMaterial::from(Color::DARK_GRAY)),
        transform: Transform::from_xyz(0.0, 0.0, 9.0),
        ..Default::default()
    });

    // Textured cone
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cone::default())),
        material: materials.add(StandardMaterial::from(checkerboard_texture.clone())),
        transform: Transform::from_xyz(0.0, 0.0, 11.0),
        ..Default::default()
    });

    // Textured cylinder
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cylinder::default())),
        material: materials.add(StandardMaterial::from(checkerboard_texture.clone())),
        transform: Transform::from_xyz(2.0, 0.0, 13.0),
        ..Default::default()
    });

    // Tiny cylinder
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cylinder {
            height: 1.0,
            radius_bottom: 0.5,
            radius_top: 0.5,
            radial_segments: 3,
            height_segments: 1,
        })),
        material: materials.add(StandardMaterial::from(Color::OLIVE)),
        transform: Transform::from_xyz(2.0, 0.0, 11.0),
        ..Default::default()
    });

    // Default cylinder
    {
        let mut mat = StandardMaterial::from(Color::CRIMSON);
        mat.cull_mode = None;
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cylinder::default())),
            material: materials.add(mat),
            transform: Transform::from_xyz(2.0, 0.0, 5.0),
            ..Default::default()
        });
    }

    // Taller regular cylinder
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cylinder::new_regular(2.2, 0.5, 16))),
        material: materials.add(StandardMaterial::from(Color::FUCHSIA)),
        transform: Transform::from_xyz(2.0, 0.0, 7.0),
        ..Default::default()
    });

    // Irregular cylinder
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(Cylinder {
            height: 1.0,
            radius_bottom: 0.6,
            radius_top: 0.2,
            radial_segments: 64,
            height_segments: 1,
        })),
        material: normal_materials.add(NormalMaterial::default()),
        transform: Transform::from_xyz(2.0, 0.0, 9.0),
        ..Default::default()
    });

    // Height segmented cylinder
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cylinder {
            height: 1.0,
            radius_bottom: 0.3,
            radius_top: 0.5,
            radial_segments: 32,
            height_segments: 5,
        })),
        material: materials.add(StandardMaterial::from(Color::SEA_GREEN)),
        transform: Transform::from_xyz(2.0, 0.0, 15.0),
        ..Default::default()
    });

    // Single-segment grid
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Grid::default())),
        material: materials.add(StandardMaterial::from(Color::SALMON)),
        transform: Transform::from_xyz(4.0, 0.0, 5.0),
        ..Default::default()
    });

    // Multi-segment grid
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Grid {
            width: 1.0,
            height: 0.6,
            width_segments: 10,
            height_segments: 6,
        })),
        material: materials.add(StandardMaterial::from(Color::TEAL)),
        transform: Transform::from_xyz(4.0, 0.0, 7.0),
        ..Default::default()
    });

    // Single-segment grid textured
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Grid::default())),
        material: materials.add(StandardMaterial::from(checkerboard_texture.clone())),
        transform: Transform::from_xyz(4.0, 0.0, 9.0),
        ..Default::default()
    });

    // Multi-segment grid textured
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Grid::new_square(1.0, 12))),
        material: materials.add(StandardMaterial::from(checkerboard_texture.clone())),
        transform: Transform::from_xyz(4.0, 0.0, 11.0),
        ..Default::default()
    });

    // Triangle polygon
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::try_from(Polygon::new_triangle(0.7)).unwrap()),
        material: materials.add(StandardMaterial::from(Color::GREEN)),
        transform: Transform::from_xyz(6.0, 0.0, 5.0),
        ..Default::default()
    });

    // Octagon polygon
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::try_from(Polygon::new_octagon(0.7)).unwrap()),
        material: materials.add(StandardMaterial::from(Color::SEA_GREEN)),
        transform: Transform::from_xyz(6.0, 0.0, 7.0),
        ..Default::default()
    });

    // Many-cornered polygon
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::try_from(Polygon::new_regular_ngon(0.7, 32)).unwrap()),
        material: materials.add(StandardMaterial::from(Color::YELLOW)),
        transform: Transform::from_xyz(6.0, 0.0, 9.0),
        ..Default::default()
    });

    // Star
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::try_from(Polygon {
            points: generate_star_shape(7, 0.7, 0.4),
        }).unwrap()),
        material: materials.add(StandardMaterial::from(checkerboard_texture.clone())),
        transform: Transform::from_xyz(6.0, 0.0, 11.0),
        ..Default::default()
    });

    // Simple torus
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Torus::default())),
        material: materials.add(StandardMaterial::from(Color::ALICE_BLUE)),
        transform: Transform::from_xyz(8.0, 0.0, 5.0),
        ..Default::default()
    });

    // Low poly torus
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Torus {
            radius: 0.8,
            tube_radius: 0.2,
            radial_segments: 8,
            tube_segments: 5,
            ..Default::default()
        })),
        material: materials.add(StandardMaterial::from(Color::PINK)),
        transform: Transform::from_xyz(8.0, 0.0, 7.0),
        ..Default::default()
    });

    // Thick torus
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Torus {
            radius: 0.5,
            tube_radius: 0.3,
            ..Default::default()
        })),
        material: materials.add(StandardMaterial::from(Color::NAVY)),
        transform: Transform::from_xyz(8.0, 0.0, 9.0),
        ..Default::default()
    });

    // Textured torus
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Torus::default())),
        material: materials.add(StandardMaterial::from(checkerboard_texture.clone())),
        transform: Transform::from_xyz(8.0, 0.0, 11.0),
        ..Default::default()
    });

    // Half torus
    {
        let mut mat = StandardMaterial::from(Color::CRIMSON);
        mat.cull_mode = None;
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Torus {
                radial_circumference: std::f32::consts::PI,
                tube_circumference: std::f32::consts::TAU,
                ..Default::default()
            })),
            material: materials.add(mat),
            transform: Transform::from_xyz(10.0, 0.0, 5.0),
            ..Default::default()
        });
    }

    // Half torus (horizontal cut)
    {
        let mut mat = StandardMaterial::from(Color::ORANGE_RED);
        mat.cull_mode = None;
        let mut flipped_transform = Transform::from_xyz(10.0, 0.0, 7.0);
        flipped_transform.rotation = Quat::from_rotation_x(std::f32::consts::PI);
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Torus {
                radial_circumference: std::f32::consts::TAU,
                tube_circumference: std::f32::consts::PI,
                tube_offset: std::f32::consts::PI * 1.5,
                ..Default::default()
            })),
            material: materials.add(mat),
            transform: flipped_transform,
            ..Default::default()
        });
    }

    // 2/3 torus with texture
    {
        let mut mat = StandardMaterial::from(checkerboard_texture.clone());
        mat.cull_mode = None;
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Mesh::from(Torus {
                radial_circumference: std::f32::consts::PI * 4.0/3.0,
                tube_circumference: std::f32::consts::TAU,
                ..Default::default()
            }))),
            material: materials.add(mat),
            transform: Transform::from_xyz(10.0, 0.0, 9.0),
            ..Default::default()
        });
    }

    // Simple tube
    {
        let mut mat = StandardMaterial::from(Color::WHITE);
        mat.cull_mode = None;

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Curve {
                curve: Box::new(WaveFunction),
                ..Default::default()
            })),
            material: materials.add(mat),
            transform: Transform::from_xyz(12.0, 0.0, 5.0),
            ..Default::default()
        });
    }

    // Knot
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Curve {
            curve: Box::new(Knot {
                rotation_winds: 2,
                circle_winds: 3,
            }),
            radius: 0.1,
            length_segments: 128,
            ..Default::default()
        })),
        material: materials.add(StandardMaterial::from(checkerboard_texture.clone())),
        transform: Transform::from_xyz(12.0, 0.0, 7.0),
        ..Default::default()
    });

    // Line tube
    {
        let mut mat = StandardMaterial::from(Color::WHITE);
        mat.cull_mode = None;

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Curve {
                curve: Box::new(Knot {
                    rotation_winds: 2,
                    circle_winds: 3,
                }),
                radius: 0.0,
                length_segments: 128,
                ..Default::default()
            })),
            material: materials.add(mat),
            transform: Transform::from_xyz(12.0, 0.0, 9.0),
            ..Default::default()
        });
    }

    // Flat wave
    {
        let mut mat = StandardMaterial::from(checkerboard_texture.clone());
        mat.cull_mode = None;

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Curve {
                radius: 0.2,
                radial_segments: 1,
                curve: Box::new(WaveFunction),
                ..Default::default()
            })),
            material: materials.add(mat),
            transform: Transform::from_xyz(12.0, 0.0, 11.0),
            ..Default::default()
        });
    }

    // Sun
    commands.spawn(DirectionalLightBundle {
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

    // Sky
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::default())),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("111111").unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(50.0)),
            ..default()
        },
        NotShadowCaster,
    ));
}

fn generate_star_shape(n: usize, radius_big: f32, radius_small: f32) -> Vec<Vec2> {
    let mut positions = Vec::new();
    let angle_step = 2.0 * std::f32::consts::PI / (n * 2) as f32;
    for i in 0..2 * n {
        let theta = angle_step * i as f32;
        if i % 2 == 0 {
            positions.push(Vec2::new(
                radius_big * f32::cos(theta),
                radius_big * f32::sin(theta),
            ));
        } else {
            positions.push(Vec2::new(
                radius_small * f32::cos(theta),
                radius_small * f32::sin(theta),
            ));
        }
    }

    positions
}

// Spawn a UI layer with the controls and other useful info.
fn spawn_info_text(mut commands: Commands, asset_server: Res<AssetServer>) {

    // Show text that presents the controls
    commands.spawn(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::from_section(
            "WASD + Mouse movement\nSpace Up, LShift Down\nESC toggle input grab\nX toggle wireframes",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                font_size: 15.0,
                color: Color::WHITE,
            },
        ).with_alignment(TextAlignment::Left),
        ..Default::default()
    });
}

// Spawn and configure the camera.
fn spawn_camera(mut commands: Commands) {
    let mut controller = FpsCameraController::default();
    controller.enabled = false; // we have a system that takes care of this, so disable it to prevent first-frame weirdness

    commands
        .spawn(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            controller,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 1.0, 0.0),
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

#[derive(Resource)]
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

    pub fn grab_mode(&self) -> CursorGrabMode {
        if self.lock {
            CursorGrabMode::Locked
        } else {
            CursorGrabMode::None
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
    } else {
        // The current focus state is the last focus event.
        if mouse.just_pressed(MouseButton::Left) {
            lock.lock = true;
        }
    }
}

// Observed the MouseLock status and updates the actual window config according to the status.
fn update_lock(mut lock: ResMut<MouseLock>, mut primary_query: Query<&mut Window, With<PrimaryWindow>>) {

    // Change detected
    if lock.lock != lock.last_lock {

        let mut window = primary_query.get_single_mut().unwrap();

        // Locking, save position
        if lock.lock {
            lock.last_position = window.cursor_position();
        }

        // Set display modes
        window.cursor.grab_mode = lock.grab_mode();
        window.cursor.visible = !lock.lock;

        // Unlocked, restore cursor position
        if !lock.lock {
            // Try to restore cursor position
            if let Some(pos) = lock.last_position {
                window.set_cursor_position(Some(pos));
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
            .add_system(update_lock.in_base_set(CoreSet::PostUpdate));
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
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                // Wireframes require line mode
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            },
        }))
        .add_plugin(smooth_bevy_cameras::LookTransformPlugin)
        .add_plugin(FpsCameraPlugin::default())
        .add_plugin(WireframePlugin)
        .add_plugin(MouseLockPlugin)
        .add_plugin(NormalMaterialPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_shapes)
        .add_startup_system(spawn_info_text)
        .add_system(toggle_wireframe_system)
        .add_system(lock_camera)
        .run();
}
