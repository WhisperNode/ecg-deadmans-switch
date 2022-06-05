use std::time::Duration;

use clap::Parser;

mod checkable;
mod cli;
mod error;
mod heart;
mod lcd;

fn main() {
    let args = cli::Args::parse();
    let agent = http_agent();
    let checkables: Vec<Box<dyn checkable::Checkable>> = vec![Box::new(lcd::block::Block::new())];
    let mut heart = heart::Heart::new(agent, args.beat_interval, checkables);
    heart.start()
}

fn http_agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(2))
        .timeout_write(Duration::from_secs(2))
        .build()
}
