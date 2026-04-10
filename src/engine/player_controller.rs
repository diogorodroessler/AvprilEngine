use bevy::{
    animation::graph::AnimationNodeIndex,
    animation::{AnimationClip, AnimationPlayer},
    camera::prelude::*,
    ecs::query::Added,
    input::mouse::MouseMotion,
    // post_process::motion_blur::MotionBlur,
    prelude::*,
};
use bevy_rapier3d::prelude::{
    ActiveEvents, AdditionalMassProperties, Collider, CollisionEvent, LockedAxes, RigidBody,
    Velocity,
};
use std::time::Duration;

/// Player Component
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CameraShake;

/// Animation Player
#[derive(Resource)]
pub struct PlayerAnimationGraph {
    graph: Handle<AnimationGraph>,
    node: AnimationNodeIndex,
}

/// Player Camera
#[derive(Component)]
pub struct CameraPitch {
    #[allow(unused)]
    pitch: f32,
}

/// Sounds for Player
#[derive(Component)]
pub struct FootstepsTimer(Timer);

/// Player Character Controller
#[derive(Component)]
pub struct PlayerCharacter;

impl PlayerCharacter {
    /// Spawn Player Character with Physics(Dynamic)
    /// Spawn Player with seguiments of commands.spawn. It uses in the function 'setup'
    pub fn spawn_player_camera(
        commands: &mut Commands,
        asset_server: Res<AssetServer>,
        mut graphs: ResMut<Assets<AnimationGraph>>,
    ) {
        // Setup Animations from Player Character
        let clip: Handle<AnimationClip> = asset_server.load(
            /* Here Going por example: "models/helena/helena.glb#Animation0" */
            "",
        );
        let (graph, node) = AnimationGraph::from_clip(clip);
        let graph_handle = graphs.add(graph);
        commands.insert_resource(PlayerAnimationGraph {
            graph: graph_handle,
            node,
        });

        // ----------------------- Player Character Old -----------------------
        // commands
        //     .spawn((
        //         // Player Character
        //         Player,
        //         Transform::default(),
        //         GlobalTransform::default(),
        //         FootstepsTimer(Timer::from_seconds(0.45, TimerMode::Repeating)),
        //     ))
        //     .insert((
        //         // Physics
        //         RigidBody::Dynamic,
        //         Collider::capsule_y(0.0, 0.2),
        //         Velocity::default(),
        //         ActiveEvents::COLLISION_EVENTS,
        //         LockedAxes::ROTATION_LOCKED,
        //         AdditionalMassProperties::Mass(70.0),
        //     ))
        //     .insert((
        //         // Camera
        //         Camera3d::default(),
        //         Visibility::default(),
        //         InheritedVisibility::default(),
        //         // Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
        //         CameraPitch { pitch: 0.0 },
        //         CameraShake,
        //     ))
        //     // .insert((
        //     //     // Player Objects
        //     //     SceneRoot(asset_server.load("models/raquel/raquel.glb#Scene0")),
        //     //     Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
        //     // ));
        //     .with_children(|f| {
        //         f.spawn((
        //             SceneRoot(asset_server.load("models/raquel/raquel.glb#Scene0")),
        //             Transform::from_scale(Vec3::new(1.0, 1.0, 1.0))
        //                 .with_rotation(Quat::from_axis_angle(Vec3::new(60.0, 0.0, 0.0), 60.0)),
        //         ));
        //     });
        // ----------------------- End Player Character Old -----------------------

        let player = commands
            .spawn((
                Player,
                Transform::default(),
                GlobalTransform::default(),
                FootstepsTimer(Timer::from_seconds(0.45, TimerMode::Repeating)),

                // Física
                RigidBody::Dynamic,
                Collider::capsule_y(0.9, 0.3),
                Velocity::default(),
                LockedAxes::ROTATION_LOCKED,
            ))
            .id();

        // Camera pivot (pitch)
        let camera_pivot = commands
            .spawn((
                Transform::from_translation(Vec3::new(0.0, 1.0, -2.0)),
                GlobalTransform::default(),
                CameraPitch { pitch: 0.0 },
            ))
            .id();

        // Camera
        let camera = commands
            .spawn((
                Camera3d::default(),
                Transform::default(),
            ))
            .id();

        // Modelo do personagem
        let model = commands
            .spawn((
                SceneRoot(asset_server.load("models/raquel/raquel.glb#Scene0")),
                Transform::from_rotation(
                    Quat::from_rotation_y(90.0) // virar de costas std::f32::consts::PI
                ),
            ))
            .id();

        // Hierarquia
        commands.entity(player).add_child(camera_pivot);
        commands.entity(camera_pivot).add_child(camera);
        commands.entity(player).add_child(model);
    }

    /// Player damage for collision
    pub fn player_collision_damage(
        mut collision_events: MessageReader<CollisionEvent>,
        velocities: Query<&Velocity>,
        mut camera: Query<&mut Transform, (With<Player>, With<CameraShake>)>,
        mut time: Res<Time>,
    ) {
        for event in collision_events.read() {
            if let CollisionEvent::Started(e1, e2, _) = event {
                if let (Ok(v1), Ok(v2)) = (velocities.get(*e1), velocities.get(*e2)) {
                    let impact = (v1.linvel - v2.linvel).length();
                    if impact >= 8.0 {
                        Self::camera_shake_damage(&mut camera, &mut time);
                        println!("You got the Hit for Impact");
                    }
                }
            }
        }
    }

    /// Player Animation abstractor
    pub fn player_animation(
        mut commands: Commands,
        mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
        graph: Res<PlayerAnimationGraph>,
    ) {
        for (entity, mut player) in &mut players {
            commands
                .entity(entity)
                .insert(AnimationGraphHandle(graph.graph.clone()));
            player.play(graph.node).repeat();
        }
    }

    /// Mouse Look with angle of 90 degree
    pub fn mouse_look(
        mut mouse_motion: MessageReader<MouseMotion>,
        mut query: Query<&mut Transform, With<Player>>,
    ) {
        let mut transform = query.single_mut().unwrap();
        let mut delta = Vec2::ZERO;

        for motion in mouse_motion.read() {
            delta += motion.delta;
        }

        let sensitivity = 0.002;
        let yaw = -delta.x * sensitivity;
        let pitch = -delta.y * sensitivity;
        let (mut yaw_rot, mut pitch_rot, _) = transform.rotation.to_euler(EulerRot::YXZ);

        yaw_rot += yaw;
        pitch_rot += pitch;

        // limit to 90 degree angle
        pitch_rot = pitch_rot.clamp(-1.54, 1.54);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw_rot, pitch_rot, 0.0);
    }

    /// Camera shake for Player Character Damage
    pub fn camera_shake_damage(
        camera: &mut Query<&mut Transform, (With<Player>, With<CameraShake>)>,
        time: &mut Res<Time>,
    ) {
        for mut transform in camera {
            let t = time.elapsed_secs();

            let shake_x = (t * 10.0).sin() * 0.1;
            let shake_y = (t * 12.0).sin() * 0.1;

            transform.translation.x = shake_x;
            transform.translation.y = shake_y;
        }
    }

    /// Player moviments with Keyboard Commands ('W,A,S,D')
    pub fn player_movement(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        keyboard: Res<ButtonInput<KeyCode>>,
        mut query: Query<&mut Transform, With<Player>>,
        mut query_sounds: Query<&mut FootstepsTimer, With<Player>>,
        time: Res<Time>,
    ) {
        let mut transform = query.single_mut().unwrap();
        let mut direction = Vec3::ZERO;

        let mut timer_tick = query_sounds.single_mut().unwrap();

        if keyboard.pressed(KeyCode::ShiftLeft) {
            timer_tick.0.set_duration(Duration::from_secs(0));
        } else if keyboard.just_released(KeyCode::ShiftLeft) {
            timer_tick.0.set_duration(Duration::from_secs(1));
        }

        if keyboard.pressed(KeyCode::KeyW) {
            direction += *transform.forward();

            timer_tick.0.tick(time.delta());
            if timer_tick.0.just_finished() {
                commands.spawn(AudioPlayer::new(
                    asset_server.load("sounds/steps/w_forward.ogg"),
                ));
            }
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction -= *transform.forward();

            timer_tick.0.tick(time.delta());
            if timer_tick.0.just_finished() {
                commands.spawn(AudioPlayer::new(
                    asset_server.load("sounds/steps/s_backward.ogg"),
                ));
            }
        }
        if keyboard.pressed(KeyCode::KeyA) {
            direction -= *transform.right();

            timer_tick.0.tick(time.delta());
            if timer_tick.0.just_finished() {
                commands.spawn(AudioPlayer::new(
                    asset_server.load("sounds/steps/a_left.ogg"),
                ));
            }
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction += *transform.right();

            timer_tick.0.tick(time.delta());
            if timer_tick.0.just_finished() {
                commands.spawn(AudioPlayer::new(
                    asset_server.load("sounds/steps/d_right.ogg"),
                ));
            }
        }

        direction.y = 0.0;

        let speed = 0.45;
        let speed_shift = 3.0;

        // Control Walking and Running Speed
        if keyboard.pressed(KeyCode::ShiftLeft) {
            transform.translation +=
                direction.normalize_or_zero() * speed_shift * time.delta_secs();
        } else if !keyboard.pressed(KeyCode::ShiftLeft) {
            transform.translation += direction.normalize_or_zero() * speed * time.delta_secs();
        }
    }
}
