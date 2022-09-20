use std::{env, error, iter, result};

use powerpack::Item;
use url::Url;

type Error = Box<dyn error::Error>;
type Result<T> = result::Result<T, Error>;

fn main() {
    // Alfred passes in a single argument for the user query.
    let query = env::args().nth(1);
    let item = if let Some(query) = query {
        run(&query)
    } else {
        Item::new("Provide an Amazon URL").valid(false)
    };

    // Output the item to Alfred!
    if let Err(e) = powerpack::output(iter::once(item)) {
        eprintln!("{}", e);
    }
}

fn run(query: &str) -> Item {
    match clean_link(query) {
        Ok(Clean::Cleaned(url)) => Item::new(&url)
            .arg(url)
            .subtitle(&format!("Cleaned from {}", query))
            .valid(true),
        Ok(Clean::Original(url)) => Item::new(&url)
            .arg(url)
            .subtitle("Kept in its original form; could not find a product page")
            .valid(true),
        Err(e) => Item::new(e.to_string()).valid(false),
    }
}

fn clean_link(url: &str) -> Result<Clean<String>> {
    let url = parse_url(url)?;
    let url = clean_url(url)?;
    Ok(match url {
        Clean::Cleaned(url) => Clean::Cleaned(url.to_string()),
        Clean::Original(url) => Clean::Original(url.to_string()),
    })
}

fn parse_url(url: &str) -> Result<Url> {
    Ok(if url.starts_with("http") {
        Url::parse(url)?
    } else {
        Url::parse(&format!("https://{}", url))?
    })
}

fn clean_url(mut url: Url) -> Result<Clean<Url>> {
    let path = url.path_segments().ok_or("url cannot be base")?;
    let mut cleaned = Cleaner::new(path);
    let new_path = cleaned.by_ref().collect::<Vec<_>>();
    Ok(if matches!(cleaned.state, State::Done) {
        url.set_path(&new_path.join("/"));
        url.set_query(None);
        url.set_fragment(None);
        Clean::Cleaned(url)
    } else {
        Clean::Original(url)
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Clean<T> {
    Cleaned(T),
    Original(T),
}

#[derive(Clone, Debug)]
struct Cleaner<I> {
    iter: I,
    state: State,
}

impl<I> Cleaner<I> {
    fn new(iter: I) -> Self {
        Self {
            iter,
            state: State::Start,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum State {
    Start,
    Dp,
    Id,
    Done,
}

impl<'a, I> Iterator for Cleaner<I>
where
    I: Iterator<Item = &'a str> + 'a,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::Start => {
                let mut previous = None;
                for segment in self.iter.by_ref() {
                    if segment == "dp" {
                        self.state = State::Dp;
                        return previous.or_else(|| self.next());
                    }
                    previous = Some(segment);
                }
                None
            }
            State::Dp => {
                self.state = State::Id;
                Some("dp")
            }
            State::Id => {
                self.state = State::Done;
                self.iter.next()
            }
            State::Done => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn assert_cleaned(input: &str, expected: &str) {
        assert_eq!(
            clean_link(input).unwrap(),
            Clean::Cleaned(String::from(expected))
        );
    }

    #[track_caller]
    fn assert_original(input: &str, expected: &str) {
        assert_eq!(
            clean_link(input).unwrap(),
            Clean::Original(String::from(expected))
        );
    }

    #[test]
    fn full_url() {
        assert_cleaned(
            "https://www.amazon.de/-/en/Jon-Gjengset/dp/1718501854?keywords=rust+for+rustaceans&qid=1663667521&sprefix=rust+for%2Caps%2C88&sr=8-1",
            "https://www.amazon.de/Jon-Gjengset/dp/1718501854"
        );
    }

    #[test]
    fn full_path_with_question_mark() {
        assert_cleaned(
            "https://www.amazon.de/-/en/Jon-Gjengset/dp/1718501854?",
            "https://www.amazon.de/Jon-Gjengset/dp/1718501854",
        );
    }

    #[test]
    fn full_path() {
        assert_cleaned(
            "https://www.amazon.de/-/en/Jon-Gjengset/dp/1718501854",
            "https://www.amazon.de/Jon-Gjengset/dp/1718501854",
        );
    }

    #[test]
    fn almost_clean() {
        assert_cleaned(
            "https://www.amazon.de/-/en/dp/1718501854",
            "https://www.amazon.de/en/dp/1718501854",
        );
    }

    #[test]
    fn remove_query() {
        assert_cleaned(
            "https://www.amazon.de/dp/1718501854#qid=1663667521",
            "https://www.amazon.de/dp/1718501854",
        );
    }

    #[test]
    fn prepend_https() {
        assert_cleaned(
            "www.amazon.de/dp/1718501854",
            "https://www.amazon.de/dp/1718501854",
        );
    }

    #[test]
    fn keep_original() {
        assert_original(
            "https://www.amazon.de/something/unexpected/1718501854?keywords=rust+for+rustaceans&qid=1663667521&sprefix=rust+for%2Caps%2C88&sr=8-1",
            "https://www.amazon.de/something/unexpected/1718501854?keywords=rust+for+rustaceans&qid=1663667521&sprefix=rust+for%2Caps%2C88&sr=8-1"
        );
    }

    #[test]
    fn match_dp_case_sensitive() {
        assert_original(
            "https://www.amazon.de/-/en/Jon-Gjengset/DP/1718501854?",
            "https://www.amazon.de/-/en/Jon-Gjengset/DP/1718501854?",
        );
    }

    #[test]
    fn prepend_https_in_original() {
        assert_original("www.amazon.de", "https://www.amazon.de/");
    }
}
