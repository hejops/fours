use clap::Parser;
use fours::thread::Catalog;
use fours::thread::Thread;
// use fours::tui::Menu;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
// board must always be provided. in addition, either subject or id must be provided
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
    board: String, // required

    #[arg(short, long)]
    subject: Option<String>,

    #[arg(short, long)]
    id: Option<usize>,

    #[arg(short, long, default_value = "false")]
    pager: bool,
}

fn main() -> anyhow::Result<()> {
    // let t = Thread::new("hr".to_owned(), 4916609).expect("could not fetch
    // thread"); t.render();
    // return Ok(());

    let args = Args::parse();

    // if args.pager {
    //     // TODO: on exiting pager, return to 'menu' of threads
    //     loop {
    //         Catalog::new(&args.board)?.menu()?;
    //     }
    //     // true => t.page()?,
    // }

    let thread = {
        if args.subject.is_some() {
            Catalog::new(&args.board)?.find_thread(&args.subject.unwrap())
        } else if args.id.is_some() {
            Thread::new(&args.board, args.id.unwrap()).ok()
        } else {
            unimplemented!();
        }
    };

    if let Some(t) = thread {
        t.write().unwrap();
    }

    Ok(())
}
