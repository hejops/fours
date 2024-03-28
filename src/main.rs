use anyhow::Result;
use clap::Parser;
use fours::thread::Catalog;
use fours::thread::Thread;
use fours::tui::Menu;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
// either subject or id must be provided
// TODO: remove this requirement; i.e. interactive catalog browser (ratatui table or inquire)
// https://github.com/clap-rs/clap/discussions/3899#discussioncomment-3096743
// https://docs.rs/clap/latest/clap/struct.ArgGroup.html
#[clap(group(
    clap::ArgGroup::new("foo")
        // .required(true)
        .args(&["subject", "id"]),
))]
struct Args {
    #[arg(short, long)]
    board: String,

    #[arg(short, long)]
    subject: Option<String>,

    #[arg(short, long)]
    id: Option<usize>,

    #[arg(short, long, default_value = "false")]
    pager: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let thread = {
        if args.subject.is_some() {
            Catalog::new(&args.board)?.find_thread(&args.subject.unwrap())
        } else if args.subject.is_some() {
            Thread::new(args.board, args.id.unwrap()).ok()
        } else {
            Catalog::new(&args.board)?.menu()?;
            unimplemented!();
        }
    };
    if let Some(t) = thread {
        match args.pager {
            true => t.page()?,
            false => t.write()?,
        }
        // TODO: on exiting pager, return to 'menu' of threads
    }
    Ok(())
}
