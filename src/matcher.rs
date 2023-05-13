use std::{fmt, ops::Deref};
use std::path::Path;

use globset::{GlobSet, GlobSetBuilder};

use crate::{Pattern, Error};

#[derive(Default, Debug)]
pub struct PatternSet {
    patterns: Vec<Pattern>,
}

#[derive(Debug)]
pub struct Matcher {
    patterns: Vec<Pattern>,
    matcher: GlobSet,
}

impl PatternSet {
    pub fn new() -> Self {
        PatternSet::default()
    }

    pub fn add(&mut self, pattern: Pattern) -> &mut Self {
        self.patterns.push(pattern);
        self
    }

    pub fn extend<I: IntoIterator<Item = Pattern>>(&mut self, patterns: I) -> &mut Self {
        self.patterns.extend(patterns);
        self
    }

    pub fn into_iter(self) -> impl Iterator<Item = Pattern> {
        self.patterns.into_iter()
    }

    pub fn into_matcher(self) -> Result<Matcher, Error> {
        let mut globset = GlobSetBuilder::new();
        for pattern in &self.patterns {
            globset.add(pattern.glob.clone());
        }

        Ok(Matcher {
            patterns: self.patterns,
            matcher: globset.build()?,
        })
    }
}

impl Deref for PatternSet {
    type Target = [Pattern];

    fn deref(&self) -> &Self::Target {
        &self.patterns
    }
}

struct RawMatchResults<'a> {
    patterns: &'a [Pattern],
    matches: Vec<usize>,
    pattern_i: usize,
    match_i: usize,
}

struct RawMatchResult<'a> {
    pattern: &'a Pattern,
    matched: bool
}

impl<'a> Iterator for RawMatchResults<'a> {
    type Item = RawMatchResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pattern_i >= self.patterns.len() {
            return None;
        }

        let matched = self.matches.get(self.match_i) == Some(&self.pattern_i);
        if matched {
            self.match_i += 1;
        }

        let result = RawMatchResult {
            matched,
            pattern: &self.patterns[self.pattern_i],
        };

        self.pattern_i += 1;
        Some(result)
    }
}

impl Matcher {
    fn raw_match_results(&self, path: &Path) -> RawMatchResults<'_> {
        RawMatchResults {
            patterns: &self.patterns,
            matches: self.matcher.matches(path),
            pattern_i: 0,
            match_i: 0,
        }
    }

    // TODO: Return something that allows identifying the pattern that matched,
    // if any. Maybe a new method: `find_match() -> Option<Match>`:
    // enum Match {
    //     Pattern(&Pattern),
    //     Exception(&Pattern),
    // }
    //
    // We could even have a `find_matches()` that returns all rules that
    // matched. `find_matches() -> Vec<Match>` or `impl Iterator<Item = MatcH>`.
    pub fn matches<P: AsRef<Path>>(&self, path: P, is_dir: bool) -> bool {
        let path = path.as_ref();
        let mut matched = false;
        for result in self.raw_match_results(path) {
            if matched && !result.pattern.exception {
                continue;
            }

            let true_match = result.matched && (!result.pattern.dir_only || is_dir);
            if !true_match {
                continue;
            }

            matched = true_match && !result.pattern.exception;
        }

        matched
    }
}

// matched exception -> false
// matched !exception -> true
// !matched !exception -> true

impl<P: IntoIterator<Item = Pattern>> From<P> for PatternSet {
    #[inline]
    fn from(patterns: P) -> Self {
        patterns.into_iter().collect()
    }
}

impl FromIterator<Pattern> for PatternSet {
    fn from_iter<T: IntoIterator<Item = Pattern>>(patterns: T) -> Self {
        let mut set = PatternSet::new();
        for pattern in patterns {
            set.add(pattern);
        }

        set
    }
}

impl FromIterator<Pattern> for Result<Matcher, Error> {
    fn from_iter<T: IntoIterator<Item = Pattern>>(patterns: T) -> Self {
        let set: PatternSet = patterns.into_iter().collect();
        set.into_matcher()
    }
}

impl fmt::Display for Matcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        for (i, pattern) in self.patterns.iter().enumerate() {
            if i != 0 { write!(f, ", ")?; }
            pattern.fmt(f)?;
        }

        write!(f, "]")
    }
}
