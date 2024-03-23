use std::fmt::Display;
use std::fs::File;
use std::io::Write;

use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;

use crate::string;

/// Shared between Catalog and Thread.
#[derive(Debug, Deserialize)]
pub struct Post {
    pub no: usize,
    com: Option<String>,
    tim: Option<usize>,

    // Only present in Catalog (?)
    pub sub: Option<String>,
}

impl Post {
    /// Sanitise all HTML line breaks, decode HTML entities, convert to plain
    /// text.
    pub fn decode(&self) -> Option<String> {
        let frag = self
            .com
            .as_ref()?
            .replace("<wbr>", "")
            .replace("<br>", "\n");
        Some(
            scraper::Html::parse_fragment(&html_escape::decode_html_entities(&frag))
                .tree
                .into_iter()
                .filter(|f| f.is_text())
                .map(|f| f.as_text().unwrap().to_string())
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}

pub struct Thread {
    board: String,
    /// Equivalent to OP id
    thread: usize,
    posts: Vec<Post>,
}
impl Thread {
    pub fn new(
        board: String,
        thread: usize,
    ) -> Result<Self> {
        let url = format!("https://a.4cdn.org/{}/thread/{}.json", board, thread);
        let resp = reqwest::blocking::get(url)?.text()?;
        let json: Value = serde_json::from_str(&resp)?;

        // the raw json only contains the field 'posts'. ideally we want to deserialise
        // the raw json directly into Vec<Post>, but serde only lets us deserialise into
        // a struct (with field 'posts'). however, such a struct does not capture any
        // state whatsoever, so we need to augment the raw json while/after
        // deserialising.
        //
        // custom deserialise? https://serde.rs/deserialize-struct.html
        let posts: Vec<Post> = json
            .get("posts")
            .context("no posts field")?
            .as_array()
            .context("could not cast as array")?
            .iter()
            .map(|p| {
                let post: Post = serde_json::from_value(p.clone()).unwrap();
                post
            })
            .collect();

        Ok(Self {
            board,
            thread,
            posts,
        })
    }

    /// Write to file
    pub fn write(&self) -> Result<()> {
        let fname = format!(
            "/tmp/{}-{}",
            self.board,
            self.posts
                .first()
                .context("no posts in thread")?
                .sub
                .as_ref()
                .context("could not cast subject as str")?,
        )
        .to_lowercase();
        let mut f = File::create(fname)?;
        write!(f, "{}", self)?;
        Ok(())
    }

    // https://github.com/arijit79/minus/blob/main/examples/less-rs.rs
    pub fn page(&self) -> Result<()> {
        let output = minus::Pager::new();
        let changes = || {
            output.push_str(self.to_string())?;
            // i have no idea what this syntax is
            Result::<()>::Ok(())
        };

        let pager = output.clone();
        let result = std::thread::spawn(|| minus::dynamic_paging(pager));
        changes()?;
        result.join().unwrap()?;
        Ok(())
    }

    // TODO: update (fetch new posts)
}

impl Display for Thread {
    /// Order: post id (leftpadded), image, comment
    ///
    /// Either image and comment will be present, sometimes both.
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let url = format!(
            "https://boards.4chan.org/{}/thread/{}",
            self.board, self.thread
        );

        writeln!(f, "{}", string::leftpad(&url))?;

        for post in self.posts.iter() {
            writeln!(f, "{}", string::leftpad(&post.no.to_string()))?;

            if let Some(tim) = &post.tim {
                writeln!(f, "https://i.4cdn.org/{}/{}.jpg", self.board, tim)?;
            }

            if post.com.is_some() {
                writeln!(
                    f,
                    "{}",
                    string::selective_wrap(&post.decode().ok_or(std::fmt::Error)?),
                )?;
            }
        }

        writeln!(f, "{}", string::leftpad(&url))?;

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct Page {
    threads: Vec<Post>,
}

#[derive(Debug, Deserialize)]
// https://serde.rs/container-attrs.html#transparent
#[serde(transparent)]
/// Array of pages.
// TODO: flatten pages? i.e. single unnested Vec<Post>
pub struct Catalog {
    pages: Vec<Page>,
}

impl Catalog {
    pub fn find_thread(
        subject: &str,
        board: &str,
    ) -> Option<Thread> {
        let url = format!("https://a.4cdn.org/{}/catalog.json", board);
        let resp = reqwest::blocking::get(url).ok()?.text().ok()?;
        let cat: Catalog = serde_json::from_str(&resp).ok()?;

        for page in cat.pages {
            if let Some(post) = page
                .threads
                .iter()
                // meh
                .find(|post| post.sub.is_some() && post.sub.as_ref().unwrap().contains(subject))
            {
                return Thread::new(board.to_string(), post.no).ok();
            }
        }
        None
    }
}
