use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy_rapier3d::prelude::*;
use std::f32::consts::{FRAC_PI_2, TAU};

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::srgb(1.0, 1.0, 1.0),
            brightness: 0.4,
            affects_lightmapped_meshes: true,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(
            Startup,
            (
                setup_camera,
                spawn_environment,
                disable_gravity,
                spawn_tunnel,
                spawn_probe,
            ),
        )
        .add_systems(Update, camera_controller)
        .add_systems(FixedUpdate, probe_thrust.before(PhysicsSet::SyncBackend))
        .run();
}

fn setup_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(-6.0, 4.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);
    let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

    commands.spawn((
        Camera3d::default(),
        transform,
        Flycam {
            yaw,
            pitch,
            speed: 5.0,
            mouse_sensitivity: 0.0025,
        },
    ));
}

fn spawn_environment(
    mut commands: Commands,
) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 15_000.0,
            ..default()
        },
        Transform::from_xyz(5.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn disable_gravity(mut configs: Query<&mut RapierConfiguration, With<DefaultRapierContext>>) {
    for mut config in &mut configs {
        config.gravity = Vec3::new(0.0, -0.5, 0.0);
    }
}

#[derive(Component)]
struct Flycam {
    yaw: f32,
    pitch: f32,
    speed: f32,
    mouse_sensitivity: f32,
}

#[derive(Component)]
struct CapsuleProbe;

fn camera_controller(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut Flycam)>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut cam) in &mut query {
        if mouse_buttons.pressed(MouseButton::Right) {
            let mut delta = Vec2::ZERO;
            for ev in mouse_motion.read() {
                delta += ev.delta;
            }
            cam.yaw -= delta.x * cam.mouse_sensitivity;
            cam.pitch -= delta.y * cam.mouse_sensitivity;
            cam.pitch = cam.pitch.clamp(-1.54, 1.54);
        } else {
            // Clear any accumulated motion if mouse not held.
            mouse_motion.clear();
        }

        let yaw = cam.yaw;
        let pitch = cam.pitch;
        let forward = Vec3::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();
        let right = forward.cross(Vec3::Y).normalize();
        let up = Vec3::Y;

        let mut velocity = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            velocity += forward;
        }
        if keys.pressed(KeyCode::KeyS) {
            velocity -= forward;
        }
        if keys.pressed(KeyCode::KeyD) {
            velocity += right;
        }
        if keys.pressed(KeyCode::KeyA) {
            velocity -= right;
        }
        if keys.pressed(KeyCode::Space) {
            velocity += up;
        }
        if keys.pressed(KeyCode::ShiftLeft) {
            velocity -= up;
        }

        if velocity.length_squared() > 0.0 {
            transform.translation += velocity.normalize() * cam.speed * dt;
        }

        transform.rotation = Quat::from_euler(EulerRot::YXZ, cam.yaw, cam.pitch, 0.0);
    }
}

fn spawn_tunnel(mut commands: Commands) {
    // Cylindrical-ish tunnel built from ring segments to avoid the "+" look.
    let inner_radius = 0.82; // tighter clearance over probe collider radius (~0.8)
    let wall_thickness = 0.05;
    let length = 12.0;
    let segments = 16;

    let half_length = length * 0.5;
    let wall_half = wall_thickness * 0.5;
    let ring_radius = inner_radius + wall_half;
    let angle_step = TAU / segments as f32;
    let tangent_half = inner_radius * (angle_step * 0.5).tan() + wall_half;

    commands
        .spawn((
            Name::new("Tunnel"),
            RigidBody::Fixed,
            Transform::default(),
            GlobalTransform::default(),
        ))
        .with_children(|child| {
            for i in 0..segments {
                let angle = i as f32 * angle_step;
                let dir = Vec2::new(angle.cos(), angle.sin());
                let center = Vec3::new(dir.x * ring_radius, dir.y * ring_radius, 0.0);
                let rot = Quat::from_rotation_z(angle);

                child.spawn((
                    Name::new(format!("Tunnel Segment {i}")),
                    Collider::cuboid(wall_half, tangent_half, half_length),
                    Friction {
                        coefficient: 1.2,
                        combine_rule: CoefficientCombineRule::Average,
                        ..default()
                    },
                    Transform::from_translation(center).with_rotation(rot),
                    GlobalTransform::default(),
                ));
            }
        });
}

fn spawn_probe(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Capsule dimensions stretched longer than the tunnel (radius ~0.8, length ~16.0).
    let collider_radius = 0.8;
    let collider_half_height = 16.0 * 0.5 - collider_radius;

    let mesh_handle = meshes.add(Mesh::from(Capsule3d::new(
        collider_radius,
        collider_half_height * 2.0,
    )));
    let material_handle = materials.add(Color::srgb(0.8, 0.2, 0.2));

    let half_length = 6.0;
    let tip_offset = collider_half_height + collider_radius;
    let tip_clearance = 0.05;
    // Place the forward tip right at the tunnel entrance (slightly inside).
    let start_z = -half_length + tip_offset - tip_clearance;

    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(material_handle),
        Transform {
            translation: Vec3::new(0.0, 0.0, start_z),
            rotation: Quat::from_rotation_x(-FRAC_PI_2),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::capsule_y(collider_half_height, collider_radius),
        Friction {
            coefficient: 1.2,
            combine_rule: CoefficientCombineRule::Average,
            ..default()
        },
        Velocity::default(),
        Sleeping::disabled(),
        ExternalImpulse::default(),
        ExternalForce::default(),
        Damping {
            linear_damping: 0.1,
            angular_damping: 0.3,
        },
        CapsuleProbe,
    ));
}

fn probe_thrust(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &mut ExternalForce, &mut ExternalImpulse, &mut Velocity), With<CapsuleProbe>>,
) {
    let thrust = 25.0;
    let impulse_strength = 1.5;
    let target_speed = 0.8;

    for (_transform, mut force, mut impulse, mut velocity) in &mut query {
        let forward_pressed = keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyI);
        let backward_pressed = keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyK);

        let dir = Vec3::Z;

        if forward_pressed {
            force.force = dir * thrust;
            impulse.impulse = dir * impulse_strength;
            velocity.linvel = dir * target_speed;
        } else if backward_pressed {
            let back = -dir;
            force.force = back * thrust;
            impulse.impulse = back * impulse_strength;
            velocity.linvel = back * target_speed;
        } else {
            force.force = Vec3::ZERO;
            impulse.impulse = Vec3::ZERO;
            // Leave current velocity; damping will bleed it off.
        }
    }
}
