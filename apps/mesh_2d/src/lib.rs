mod utils;
pub use utils::random_color;

mod systems;
use systems::{system_listeners::keyboard_event_system, update_colours_system};

mod model;
mod state;
mod prelude {
    pub use crate::model::{tags, Board, Entity, Float64Value};
    pub use crate::state::{State, StateTrack};
    pub use crate::utils::random_color;
    pub use std::io::Result;
}

use crate::model::tags::BoardItem;
pub use crate::model::{Board, Entity, Float64Value};
pub use std::io::{ErrorKind, Result};

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::os::unix::net::UnixListener;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

const SOCKET_FILE: &str = "/tmp/remote-field.sock";

fn set_custom_panic_hook() {
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        // Do some reaction for panicked application just here,
        // HTTP Requests, etc. (https://stackoverflow.com/a/45623133/6493531)
        error!("Application panicked!");
        default_panic(info);
    }));
}

fn start_unix_socket_handler() -> Receiver<String> {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        match std::fs::remove_file(SOCKET_FILE) {
            Ok(_) => {}
            Err(_) => {}
        }

        let unix_listener = UnixListener::bind(SOCKET_FILE).unwrap();

        loop {
            let (mut unix_stream, _socket_address) = unix_listener.accept().unwrap();
            let mut message = String::new();
            unix_stream.read_to_string(&mut message).unwrap();
            tx.send(message).unwrap();
        }
    });

    rx
}

fn start_web_server() -> Receiver<String> {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    thread::spawn(move || {
        let http_listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in http_listener.incoming() {
            match stream {
                Ok(stream) => {
                    let command = handle_connection(stream);
                    tx.send(command).unwrap();
                }
                Err(_) => {}
            }
        }
    });

    rx
}

#[derive(Resource)]
struct SocketReceiver(Mutex<Receiver<String>>);

#[derive(Resource)]
struct HttpReceiver(Mutex<Receiver<String>>);

pub fn main() {
    set_custom_panic_hook();
    let rx = start_unix_socket_handler();
    let rx2 = start_web_server();
    App::new()
        .insert_resource(SocketReceiver(Mutex::new(rx)))
        .insert_resource(HttpReceiver(Mutex::new(rx2)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup.pipe(error_handler))
        .add_systems(FixedFirst, process_unix_socket_data.pipe(error_handler))
        .add_systems(FixedFirst, process_http_data.pipe(error_handler))
        .add_systems(FixedPreUpdate, keyboard_event_system.pipe(error_handler))
        .add_systems(FixedUpdate, update_colours_system.pipe(error_handler))
        .add_systems(FixedPostUpdate, panic_system_test.pipe(error_handler))
        .run();
}

fn panic_system_test() -> Result<()> {
    // Err(Error::new(ErrorKind::Other, "oh no"))
    Ok(())
}

fn error_handler(In(result): In<Result<()>>, mut commands: Commands) {
    match result {
        Ok(_) => {}
        Err(err) => error!("System failed: {:?}", err),
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result<()> {
    commands.spawn(Camera2dBundle::default());

    let app_state = state::State::default();

    for i in 0..3 {
        for j in 0..3 {
            let material = materials.add(random_color());

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Rectangle::default()).into(),
                    transform: Transform {
                        translation: Vec3::new((i * 10) as f32, (j * 10) as f32, 1.),
                        scale: Vec3::splat(10.),
                        ..default()
                    },
                    material,
                    ..default()
                },
                BoardItem,
            ));
        }
    }

    commands.insert_resource(app_state);

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(Color::PURPLE),
        ..default()
    });

    Ok(())
}

fn process_unix_socket_data(
    mut state: ResMut<crate::state::State>,
    rx: Res<SocketReceiver>,
) -> Result<()> {
    if let Ok(data) = rx.0.lock().unwrap().try_recv() {
        println!("unix socket received: {}", data);
    }

    Ok(())
}

fn process_http_data(mut state: ResMut<crate::state::State>, rx: Res<HttpReceiver>) -> Result<()> {
    if let Ok(data) = rx.0.lock().unwrap().try_recv() {
        println!("http received: {}", data);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> String {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let (status_line, filename) = if http_request[0] == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else if http_request[0] == "GET /style_guide.html HTTP/1.1" {
        ("HTTP/1.1 200 OK", "style_guide.html")
    } else if http_request[0] == "GET /assets/css/font.css HTTP/1.1" {
        ("HTTP/1.1 200 OK", "assets/css/font.css")
    } else if http_request[0] == "GET /assets/css/reset.css HTTP/1.1" {
        ("HTTP/1.1 200 OK", "assets/css/reset.css")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = std::fs::read_to_string(format!("{}{}", "./apps/mesh_2d/web_admin/dist/", filename)).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    "WIP".to_string()
}
