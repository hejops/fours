use core::panic;

use anyhow::Result;
use clap::Parser;
use fours::thread::Post;
use fours::thread::Thread;
use serde::Deserialize;

// https://github.com/arijit79/minus/blob/main/examples/less-rs.rs

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

#[derive(Debug, Deserialize)]
struct Page {
    threads: Vec<Post>,
}

#[derive(Debug, Deserialize)]
// https://serde.rs/container-attrs.html#transparent
#[serde(transparent)]
/// Array of pages.
struct Catalog {
    pages: Vec<Page>,
}

impl Catalog {
    fn find_thread(
        &self,
        subject: &str,
        board: &str,
    ) -> Option<Thread> {
        for page in &self.pages {
            if let Some(post) = page
                .threads
                .iter()
                .find(|post| post.sub.as_ref().unwrap().contains(subject))
            {
                return Thread::new(board.to_string(), post.no).ok();
            }
        }
        None
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let thread = {
        if let Some(id) = args.id {
            Thread::new(args.board, id).ok()
        } else if let Some(subject) = args.subject {
            let url = format!("https://a.4cdn.org/{}/catalog.json", args.board);
            let resp = reqwest::blocking::get(url)?.text()?;
            let cat: Catalog = serde_json::from_str(&resp)?;
            cat.find_thread(&subject, &args.board)
        } else {
            panic!()
        }
    };
    if let Some(t) = thread {
        t.write()?;
    }
    Ok(())
}
