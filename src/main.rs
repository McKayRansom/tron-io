
mod snake;


#[macroquad::main("Snake")]
async fn main() {
    snake::main().await;
}
