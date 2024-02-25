pub mod constants;

pub use constants::{ADJECTIVES, ADVERBS, NOUNS, VERBS};

#[must_use]
pub fn random<'a>(vec: &'a [&'a str]) -> &'a str {
    let index = fastrand::usize(..vec.len());
    vec[index]
}

#[must_use]
pub fn longest<'a>(vec: &'a [&'a str]) -> &'a str {
    vec.iter().max_by_key(|s| s.len()).unwrap_or(&"")
}

#[must_use]
pub fn shortest<'a>(vec: &'a [&'a str]) -> &'a str {
    vec.iter().min_by_key(|s| s.len()).unwrap_or(&"")
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    chars.next().map_or_else(String::new, |c| {
        c.to_uppercase().collect::<String>() + chars.as_str()
    })
}

#[derive(Debug)]
pub struct Options<'a> {
    pub separator: Option<&'a str>,
    pub capitalize: bool,
    pub add_adverb: bool,
    pub adjective_count: usize,
}

impl Default for Options<'_> {
    fn default() -> Self {
        Self {
            separator: None,
            capitalize: true,
            add_adverb: false,
            adjective_count: 1,
        }
    }
}

impl Options<'_> {
    #[must_use]
    pub fn builder<'a>() -> OptionsBuilder<'a> {
        OptionsBuilder::default()
    }
}

#[derive(Debug)]
pub struct OptionsBuilder<'a> {
    separator: Option<&'a str>,
    capitalize: bool,
    add_adverb: bool,
    adjective_count: usize,
}

impl Default for OptionsBuilder<'_> {
    fn default() -> Self {
        Self {
            separator: None,
            capitalize: true,
            add_adverb: false,
            adjective_count: 1,
        }
    }
}

impl<'a> OptionsBuilder<'a> {
    #[must_use]
    pub const fn separator(mut self, separator: &'a str) -> Self {
        self.separator = Some(separator);
        self
    }

    #[must_use]
    pub const fn capitalize(mut self, capitalize: bool) -> Self {
        self.capitalize = capitalize;
        self
    }

    #[must_use]
    pub const fn add_adverb(mut self, add_adverb: bool) -> Self {
        self.add_adverb = add_adverb;
        self
    }

    #[must_use]
    pub const fn adjective_count(mut self, adjective_count: usize) -> Self {
        self.adjective_count = adjective_count;
        self
    }

    #[must_use]
    pub const fn build(self) -> Options<'a> {
        Options {
            separator: self.separator,
            capitalize: self.capitalize,
            add_adverb: self.add_adverb,
            adjective_count: self.adjective_count,
        }
    }
}

pub fn human_id(options: Option<Options>) -> String {
    let options = options.unwrap_or_default();

    let mut words = Vec::new();

    for _ in 0..options.adjective_count {
        words.push(random(&ADJECTIVES));
    }

    words.push(random(&NOUNS));
    words.push(random(&VERBS));

    if options.add_adverb {
        words.push(random(&ADVERBS));
    }

    let words: Vec<_> = if options.capitalize {
        words.iter().map(|s| capitalize(s)).collect()
    } else {
        words.into_iter().map(String::from).collect()
    };

    options
        .separator
        .map_or_else(|| words.join(""), |sep| words.join(sep))
}
