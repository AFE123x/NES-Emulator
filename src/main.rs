use slog::{Drain, Logger};
mod nes;
use std::{cell::{Ref, RefCell}, rc::Rc};

fn main() {

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator)
        .build()
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = Logger::root(drain, slog::o!());
    slog::info!(log, "hello world");

}
