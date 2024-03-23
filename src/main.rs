use core::panic;

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
        if let Some(id) = args.id {
            Thread::new(args.board, id).ok()
        } else if let Some(subject) = args.subject {
            Catalog::find_thread(&subject, &args.board)
        } else {
            panic!()
        }
    };
    if let Some(t) = thread {
        t.write()?;
    }
    Ok(())
}
