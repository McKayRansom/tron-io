use nanoserde::DeBin;
use quad_net::quad_socket::server::SocketHandle;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tron_io_world::server::WorldServer;
use tron_io_world::server::connection::ServerConnectionState;
use tron_io_world::{ClientMsg, ServerMsg, WorldState};

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
        log::debug!("Received msg {:?}", msg);
        if self.world.is_none() {
            let mut waiting_queue = waiting_queue.lock().unwrap();

            log::info!("Client connected");

            if let Some(world_arc) = &waiting_queue.other_waiting {
                // if there is another waiting player, send them both into the same world
                let mut world = world_arc.lock().unwrap();
                if world.world_state == WorldState::Waiting {
                    world.world_state = WorldState::Playing;
                    log::info!("Adding player to world");

                    self.world = Some(world_arc.clone());
                    self.connection = Some(ServerConnectionState::new());
                }
            }
            if self.world.is_none() {
                // if there is no other waiting player, add this one to the queue
                let world = WorldServer::new();

                log::info!("Starting new world");

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
                    Self::send_msg(out, &response);
                }
            }
        }
    }

    fn send_msg(out: &mut SocketHandle, msg: &ServerMsg) {
        {
            let this = &mut *out;
            let data = nanoserde::SerBin::serialize_bin(msg);
            log::debug!("Sending msg {:?}", msg);
            if let Err(err) = this.send(&data) {
                log::error!("Failed to send message: {:?}", err);
            }
        }
    }
    fn get_time() -> f64 {
        use std::time::SystemTime;

        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|e| panic!("{}", e));
        time.as_secs_f64()
    }

    fn on_timer(&self, out: &mut SocketHandle) {
        if let Some(world) = &self.world {
            let mut world = world.lock().unwrap();
            if let Some(connection) = &self.connection {
                if let Some(response) = connection.update(&mut world) {
                    Self::send_msg(out, &response);
                }
            }
            // I don't love that any thread can update the world, but it works for now
            world.update(Self::get_time());

        }
    }

    fn on_disconnect(&self) {
        log::info!("Client disconnected");
    }
}

struct WaitingQueue {
    other_waiting: Option<Arc<Mutex<WorldServer>>>,
}

pub fn main() -> std::io::Result<()> {
    // Initialize logging, and log the "info" level for this crate only, unless
    // the environment contains `RUST_LOG`.
    let env = env_logger::Env::new().default_filter_or("info");
    env_logger::Builder::from_env(env)
        .format_module_path(false)
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .init();

    let tcp_addr = "0.0.0.0:8090";
    let ws_addr = "0.0.0.0:8091";

    log::info!("listening on tcp://{} and ws://{}", tcp_addr, ws_addr);

    // let world = Arc::new(Mutex::new(World::new()));
    let waiting_queue = Arc::new(Mutex::new(WaitingQueue {
        other_waiting: None,
    }));

    quad_net::quad_socket::server::listen(
        tcp_addr,
        ws_addr,
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
