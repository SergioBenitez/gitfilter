use std::fmt;
use std::borrow::Cow;
use std::ops::Deref;
use std::path::Path;

use crate::{Error, PatternSet, PathExt, Matcher};

use globset::{GlobBuilder, Glob};

#[derive(Debug, Clone)]
pub struct Pattern {
    pub(crate) glob: Glob,
    pub(crate) root: String,
    pub(crate) exception: bool,
    pub(crate) dir_only: bool,
    pub(crate) rooted: bool,
}

struct RawPattern(str);

enum Prefix {
    Root,
    Negative,
    NegativeRoot,
    None,
}

impl RawPattern {
    fn new(string: &str) -> &RawPattern {
        unsafe { &*(string as *const str as *const RawPattern) }
    }

    fn prefix(&self) -> Prefix {
        let bytes = self.0.as_bytes();
        match (bytes.get(0), bytes.get(1)) {
            (Some(b'/'), _) => Prefix::Root,
            (Some(b'!'), Some(b'/')) => Prefix::NegativeRoot,
            (Some(b'!'), _) => Prefix::Negative,
            _ => Prefix::None,
        }
    }

    pub fn path(&self) -> &str {
        let prefix = match self.prefix() {
            Prefix::Root | Prefix::Negative => &self.0[1..],
            Prefix::NegativeRoot => &self.0[2..],
            Prefix::None => &self.0,
        };

        prefix.strip_suffix('/').unwrap_or(prefix)
    }

    fn rooted(&self) -> bool {
        match self.prefix() {
            Prefix::Root | Prefix::NegativeRoot => true,
            _ => self.path().contains('/')
        }
    }

    fn negative(&self) -> bool {
        matches!(self.prefix(), Prefix::Negative | Prefix::NegativeRoot)
    }

    fn dir_only(&self) -> bool {
        self.ends_with('/')
    }
}

impl Pattern {
    pub fn new<P: AsRef<Path>>(pattern: &str, root: P) -> Result<Self, Error> {
        let pattern = RawPattern::new(pattern);
        let mut glob: Cow<'_, str> = (pattern.rooted() || pattern.path().starts_with("**/"))
            .then(|| pattern.path().into())
            .unwrap_or_else(|| format!("**/{}", pattern.path()).into());

        let root = root.to_normalized_string_lossy();
        if !root.is_empty() {
            if !glob.is_empty() {
                if root.ends_with('/') {
                    glob = format!("{}{}", root, glob).into();
                } else {
                    glob = format!("{}/{}", root, glob).into();
                }
            } else {
                glob = root.clone();
            }
        }

        Ok(Pattern {
            glob: GlobBuilder::new(&glob).literal_separator(true).build()?,
            root: root.into(),
            exception: pattern.negative(),
            dir_only: pattern.dir_only(),
            rooted: pattern.rooted(),
        })
    }

    // Will go _below_ the root.
    pub fn rootful_dedotted(self) -> Self {
        let dedotted = self.glob.glob().dedot();
        let glob = dedotted.to_normalized_string_lossy();
        Pattern {
            glob: GlobBuilder::new(&glob).literal_separator(true).build().expect("valid => valid"),
            ..self
        }
    }

    pub fn dedotted(self) -> Self {
        let glob = self.glob.glob();
        let glob = glob.strip_prefix(&self.root).unwrap_or(glob).dedot();
        let glob = glob.to_normalized_string_lossy();
        let pattern = Pattern::new(&glob, &self.root).expect("valid => valid");
        Pattern {
            glob: pattern.glob,
            ..self
        }
    }

    pub fn with_root<P: AsRef<Path>>(&self, root: P) -> Result<Self, Error> {
        Pattern::new(&self.to_string(), root.as_ref())
    }

    pub fn invert(mut self) -> Pattern {
        self.exception = !self.exception;
        self
    }

    pub fn into_set(self) -> PatternSet {
        [self].into_iter().collect()
    }

    pub fn into_matcher(self) -> Result<Matcher, Error> {
        self.into_set().into_matcher()
    }
}

impl Deref for RawPattern {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Pattern::new(s, "")
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use fmt::Write;

        if self.exception { f.write_char('!')?; }
        if self.rooted && !self.root.starts_with('/') { f.write_char('/')?; }
        f.write_str(self.glob.glob())?;
        if self.dir_only { f.write_char('/')?; }

        Ok(())
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::Pattern;
    use serde::{ser, de};

    impl ser::Serialize for Pattern {
        fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.to_string().serialize(serializer)
        }
    }

    impl<'de> de::Deserialize<'de> for Pattern {
        fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            struct Visitor;

            impl<'de> de::Visitor<'de> for Visitor {
                type Value = Pattern;

                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "a gitignore pattern")
                }

                fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                    v.parse().map_err(E::custom)
                }
            }

            deserializer.deserialize_string(Visitor)
        }
    }
}
