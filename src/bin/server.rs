use nanoserde::DeBin;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tron_io::grid::msg::{BikeUpdate, GridUpdateMsg};

#[derive(Default)]
struct ClientState {
    id: Option<usize>,
    tick: u32,
}

struct World {
    // pos: (f32, f32),
    // last_edit_id: usize,
    // unique_id: usize,
    last_update_time: Instant,
    next_update: GridUpdateMsg,
    last_update: GridUpdateMsg,
}

impl World {
    fn new() -> Self {
        Self {
            // pos: (0.0, 0.0),
            // last_edit_id: 0,
            // unique_id: 0,
            last_update_time: Instant::now(),
            next_update: GridUpdateMsg {
                tick: 0,
                seed: 0,
                updates: vec![],
            },
            last_update: GridUpdateMsg {
                tick: 0,
                seed: 0,
                updates: vec![],
            },
        }
    }
}

pub fn main() -> std::io::Result<()> {
    let world = Arc::new(Mutex::new(World::new()));

    quad_net::quad_socket::server::listen(
        "0.0.0.0:8090",
        "0.0.0.0:8091",
        quad_net::quad_socket::server::Settings {
            on_message: {
                let world = world.clone();
                move |mut _out, state: &mut ClientState, msg| {
                    let mut msg: GridUpdateMsg = DeBin::deserialize_bin(&msg).unwrap();

                    let mut world = world.lock().unwrap();
                    println!("Received message: {:?}", msg);
                    world.next_update.updates.append(&mut msg.updates);
                    state.tick = msg.tick;
                    // if state.id.is_none() {
                    //     state.id = Some(world.lock().unwrap().unique_id);
                    //     world.lock().unwrap().unique_id += 1;
                    // }
                    // world.lock().unwrap().last_edit_id = state.id.unwrap();
                    // world.lock().unwrap().pos = msg;
                    // out.send_bin(&msg).unwrap();
                }
            },
            on_timer: move |out, state| {
                let mut world = world.lock().unwrap();
                if state.tick != world.last_update.tick {
                    out.send_bin(&world.last_update).unwrap();
                    // state.tick = world.update.tick;
                }
                // weirld but okay
                if world.last_update_time.elapsed() > Duration::from_millis(50) {
                    world.last_update_time = Instant::now();
                    world.last_update = world.next_update.clone();
                    world.next_update.updates.clear();
                    world.next_update.tick += 1;
                    // out.send_bin(&world.update).unwrap();
                }
                // out.send_bin(&(world.pos.0, world.pos.1, world.last_edit_id))
                //     .unwrap();
            },
            on_disconnect: |_| {},
            timer: Some(Duration::from_millis(10)),
            _marker: std::marker::PhantomData,
        },
    );
    Ok(())
}
