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

/// Player Component for the moviment
#[derive(Component)]
pub struct Player;

/// Animation Player
#[derive(Resource)]
pub struct PlayerAnimationGraph {
    graph: Handle<AnimationGraph>,
    node: AnimationNodeIndex,
}

/// Part of Player Moviments
/// In the case for Player Camera Fps style
#[derive(Component)]
pub struct CameraPitch {
    #[allow(unused)]
    pitch: f32,
}

/// Sounds for Player
#[derive(Component)]
pub struct FootstepsTimer(Timer);

#[derive(Component)]
/// All the into the memory
pub struct PlayerCharacter;

impl PlayerCharacter {
    /// Spawn Player Character with Physics(Dynamic)
    pub fn player_character(
        commands: &mut Commands,
        asset_server: Res<AssetServer>,
        mut graphs: ResMut<Assets<AnimationGraph>>,
    ) {
        // Setup Animations from Player Character
        let clip: Handle<AnimationClip> = asset_server.load(
            /* Here Going por example: "models/helena/helena.glb#Animation0" */
            "models/helena/helena.glb#Animation0"
        );
        let (graph, node) = AnimationGraph::from_clip(clip);
        let graph_handle = graphs.add(graph);
        commands.insert_resource(PlayerAnimationGraph {
            graph: graph_handle,
            node,
        });

        // Spawn Player Character with Physics(complete)
        commands
            .spawn((
                SceneRoot(
                    asset_server.load(
                        GltfAssetLabel::Scene(0).from_asset(
                            /* Here Going for example: "models/helena/helena.glb#Scene0" */
                            "models/helena/helena.glb#Scene0"
                        ),
                    ),
                ),
                RigidBody::Dynamic,
                Collider::capsule_y(0.9, 0.4),
                Velocity::default(),
                ActiveEvents::COLLISION_EVENTS,
                LockedAxes::ROTATION_LOCKED,
                AdditionalMassProperties::Mass(70.0),
                /* ------ LEAVE COMMENTED YET ------ */
                // Transform::from_xyz(0.0, 3.0, 5.0),
                // GlobalTransform::default(),
                // Transforms Global universe
            ))
            // ------ CHECK: Player Character is broken in the scene, and not attach ------
            // o Maybe I can do this in the other forms ;)
            // ------
            // TODO:
            // o Player Attach;
            // o Angle: 0.0 (FOR TOUCH IN THE GROUND);
            .insert((Transform::from_xyz(0.0, 3.0, 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y)
                .with_scale(Vec3 {
                    x: 0.01,
                    y: 0.01,
                    z: 0.01,
                }),));
    }

    /// Spawn Player with seguiments of commands.spawn. It uses in the function 'setup'
    pub fn spawn_player_camera(commands: &mut Commands) {
        commands
            .spawn((
                Player,
                /* ------ LEAVE COMMENTED YET ------ */
                // RigidBody::Dynamic,
                // Collider::capsule_y(0.9, 0.4),
                // Velocity::default(),
                // engine::network::components::Velocity { x: 0.0, y: 0.0, z: 0.0 },
                // ActiveEvents::COLLISION_EVENTS,
                // LockedAxes::ROTATION_LOCKED,
                // AdditionalMassProperties::Mass(70.0),
                // Transform::from_xyz(0.0, 3.0, 5.0),
                Transform::default(),
                GlobalTransform::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                FootstepsTimer(Timer::from_seconds(0.45, TimerMode::Repeating)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Camera3d::default(),
                    /* ------ Example for add post-processing graphics ------ */
                    // MotionBlur {
                    //     shutter_angle: 0.5,
                    //     samples: 0,
                    // },
                    Transform::from_xyz(0.0, 0.6, 0.0),
                    CameraPitch { pitch: 0.0 },
                ));
            });
    }

    /// Player damage for collision
    pub fn player_collision_damage(
        mut collision_events: MessageReader<CollisionEvent>,
        velocities: Query<&Velocity>,
    ) {
        for event in collision_events.read() {
            if let CollisionEvent::Started(e1, e2, _) = event {
                if let (Ok(v1), Ok(v2)) = (velocities.get(*e1), velocities.get(*e2)) {
                    let impact = (v1.linvel - v2.linvel).length();
                    if impact >= 8.0 {
                        // Take Hit On Damage here
                        // Damage for Impact
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

        if keyboard.pressed(KeyCode::ShiftLeft) {
            transform.translation +=
                direction.normalize_or_zero() * speed_shift * time.delta_secs();
        } else if !keyboard.pressed(KeyCode::ShiftLeft) {
            transform.translation += direction.normalize_or_zero() * speed * time.delta_secs();
        }
    }
}
