use slog::{Drain, Logger};
mod nes;

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator)
        .build()
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = Logger::root(drain, slog::o!());
    slog::info!(log, "hello world");

    let mut nes = nes::Nes::new();
    match nes.game_loop() {
        Ok(_) => println!("Game loop exited successfully."),
        Err(e) => eprintln!("Error in game loop: {}", e),
    }
}
