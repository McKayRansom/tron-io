use nanoserde::DeBin;
use quad_net::quad_socket::server::SocketHandle;
use tron_io::world::server::connection::ServerConnectionState;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tron_io::grid::msg::{ClientMsg, ServerMsg, WorldState};
use tron_io::world::server::WorldServer;

#[derive(Default)]
struct ClientState {
    connection: Option<ServerConnectionState>,
    world: Option<Arc<Mutex<WorldServer>>>,
}

impl ClientState {
    fn on_msg(
        &mut self,
        msg: ClientMsg,
        out: &mut SocketHandle,
        waiting_queue: Arc<Mutex<WaitingQueue>>,
    ) {
        if self.world.is_none() {
            let mut waiting_queue = waiting_queue.lock().unwrap();

            if let Some(world_arc) = &waiting_queue.other_waiting {
                // if there is another waiting player, send them both into the same world
                let mut world = world_arc.lock().unwrap();
                if world.world_state == WorldState::Waiting {
                    world.world_state = WorldState::Playing;
                    println!("Adding player to world");

                    self.world = Some(world_arc.clone());
                    self.connection = Some(ServerConnectionState::new());
                }
            }
            if self.world.is_none() {
                // if there is no other waiting player, add this one to the queue
                let world = WorldServer::new();

                println!("Starting new world");

                let world = Arc::new(Mutex::new(world));
                self.world = Some(world.clone());
                self.connection = Some(ServerConnectionState::new());
                waiting_queue.other_waiting = Some(world);
            }
        }
        if let Some(world) = &self.world {
            let mut world = world.lock().unwrap();
            if let Some(connection) = &mut self.connection {
                if let Some(response) = connection.on_msg(&msg, &mut world) {
                    // send the response to the client
                    Self::send_msg(out, &response).unwrap();
                }
            }

        }
    }

    fn send_msg(out: &mut SocketHandle, msg: &ServerMsg) -> Result<(), ()> {
        {
            let this = &mut *out;
            let data = nanoserde::SerBin::serialize_bin(msg);
            // println!("Sending msg {:?}", data);
            this.send(&data)
        }
    }

    fn on_timer(&self, out: &mut SocketHandle) {
        if let Some(world) = &self.world {
            let mut world = world.lock().unwrap();
            if let Some(connection) = &self.connection {
                if let Some(response) = connection.update(&mut world) {
                    Self::send_msg(out, &response).unwrap();
                }
            }
            // I don't love that any thread can update the world, but it works for now
            world.update();
        }
    }

    fn on_disconnect(&self) {
        println!("Client disconnected");
    }
}

struct WaitingQueue {
    other_waiting: Option<Arc<Mutex<WorldServer>>>,
}

pub fn main() -> std::io::Result<()> {
    // let world = Arc::new(Mutex::new(World::new()));
    let waiting_queue = Arc::new(Mutex::new(WaitingQueue {
        other_waiting: None,
    }));

    quad_net::quad_socket::server::listen(
        "0.0.0.0:8090",
        "0.0.0.0:8091",
        quad_net::quad_socket::server::Settings {
            on_message: {
                let waiting_queue = waiting_queue.clone();
                move |out, state: &mut ClientState, msg| {
                    let msg: ClientMsg = DeBin::deserialize_bin(&msg).unwrap();
                    state.on_msg(msg, out, waiting_queue.clone());
                }
            },
            on_timer: move |out, state| {
                state.on_timer(out);
            },
            on_disconnect: |state| {
                state.on_disconnect();
            },
            timer: Some(Duration::from_millis(10)),
            _marker: std::marker::PhantomData,
        },
    );
    Ok(())
}
