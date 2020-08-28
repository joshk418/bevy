use bevy::{
    input::{keyboard::KeyCode, Input},
    window::CursorMoved,
    prelude::*,
};

/// This example illustrates how to create parent->child relationships between entities how parent transforms
/// are propagated to their descendants
fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .init_resource::<State>()
        .add_startup_system(setup.system())
        .add_system(move_camera_system.system())
        .run();
}

/// set up a simple scene with a "parent" cube and a "child" cube
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let cube_material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.5, 0.4, 0.3),
        ..Default::default()
    });

    commands
        // parent cube
        .spawn(PbrComponents {
            mesh: cube_handle,
            material: cube_material_handle,
            translation: Translation::new(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .with_children(|parent| {
            // child cube
            parent.spawn(PbrComponents {
                mesh: cube_handle,
                material: cube_material_handle,
                translation: Translation::new(0.0, 0.0, 3.0),
                ..Default::default()
            });
        })
        // light
        .spawn(LightComponents {
            translation: Translation::new(4.0, 5.0, -4.0),
            ..Default::default()
        })

        // camera
        .spawn(Camera3dComponents {
            transform: Transform::new_sync_disabled(Mat4::face_toward(
                Vec3::new(5.0, 10.0, 10.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        })
        .with(Camera { 
            move_speed: 5.0,
            position: Vec3::new(0.0, 0.0, -1.0), 
            front: Vec3::new(0.0, 0.0, 1.0), 
            up: Vec3::new(0.0, 1.0, 0.0), 
            pitch: 0.0, 
            yaw: -90.0,
        })
        .with(MousePosition{
            position_x: 0.0, 
            position_y: 0.0,
            last_mouse_x: 400.0,
            last_mouse_y: 300.0,
            sensitivity: 0.1,
        });
}

struct Camera {
    move_speed: f32,
    position: Vec3,
    front: Vec3,
    up: Vec3,
    pitch: f32,
    yaw: f32
}

struct MousePosition {
    position_x: f32,
    position_y: f32,
    last_mouse_x: f32,
    last_mouse_y: f32,
    sensitivity: f32
}

#[derive(Default)]
struct State {
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

fn move_camera_system(
    time: Res<Time>, 
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut state: ResMut<State>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut query: Query<(&mut Camera, &mut MousePosition, &mut Transform)>
) {
    for (mut camera, mut mouse_position, mut transform) in &mut query.iter() {
        let move_speed = camera.move_speed;
        let front = camera.front;
        let up = camera.up;

        // Move forward/back and left/right
        if keyboard_input.pressed(KeyCode::W) {
            camera.position += time.delta_seconds * move_speed * front;
        }
        if keyboard_input.pressed(KeyCode::A) {
            camera.position -= time.delta_seconds * move_speed * Vec3::normalize(Vec3::cross(front, up));
        }
        if keyboard_input.pressed(KeyCode::S) {
            camera.position -= time.delta_seconds * move_speed * front;
        }
        if keyboard_input.pressed(KeyCode::D) {
            camera.position += time.delta_seconds * move_speed * Vec3::normalize(Vec3::cross(front, up));
        }

        // Modify camera move speed
        if keyboard_input.just_pressed(KeyCode::LShift) {
            camera.move_speed *= 2.0; 
        }
        if keyboard_input.just_released(KeyCode::LShift) {
            camera.move_speed /= 2.0; 
        }

        // Move camera's pitch and yaw when right click is pressed
        if mouse_button_input.pressed(MouseButton::Right) {
            for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
                mouse_position.position_x = event.position.x();
                mouse_position.position_y = event.position.y();
            }
        }

        let x_offset = mouse_position.position_x - mouse_position.last_mouse_x;
        let y_offset = mouse_position.last_mouse_y - mouse_position.position_y;
        mouse_position.last_mouse_x = mouse_position.position_x;
        mouse_position.last_mouse_y = mouse_position.position_y;

        camera.yaw += x_offset *  mouse_position.sensitivity;
        camera.pitch -= y_offset *  mouse_position.sensitivity;

        if camera.pitch > 179.0 {
            camera.pitch = 179.0;
        }
        if camera.pitch < -179.0 {
            camera.pitch = -179.0;
        }

        let mut dir = Vec3::zero();
        dir.set_x(f32::to_radians(camera.yaw).cos() * f32::to_radians(camera.pitch).cos());
        dir.set_y(f32::to_radians(camera.pitch).sin());
        dir.set_z(f32::to_radians(camera.yaw).sin() * f32::to_radians(camera.pitch).cos());
        camera.front = Vec3::normalize(dir);

        *transform = Transform::new_sync_disabled(Mat4::face_toward(camera.position, camera.position + camera.front, camera.up));
    }
}