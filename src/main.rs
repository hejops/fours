use core::panic;

use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use fours::thread::Catalog;
use fours::thread::Thread;

// TODO: pager https://github.com/arijit79/minus/blob/main/examples/less-rs.rs

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
// either subject or id must be provided
// https://github.com/clap-rs/clap/discussions/3899#discussioncomment-3096743
// https://docs.rs/clap/latest/clap/struct.ArgGroup.html
#[clap(group(
    clap::ArgGroup::new("foo")
        .required(true)
        .args(&["subject", "id"]),
))]
struct Args {
    #[arg(short, long)]
    board: String,

    #[arg(short, long)]
    subject: Option<String>,

    #[arg(short, long)]
    id: Option<usize>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let thread = {
        // there is probably a better way to use match with an enum (e.g.
        // Query::Subject)
        match args {
            Args {
                subject: Some(subject),
                ..
            } => Catalog::find_thread(&subject, &args.board),
            _ => Thread::new(args.board, args.id.context("no thread id")?).ok(),
        }
    };
    if let Some(t) = thread {
        t.write()?;
    }
    Ok(())
}
