use std::borrow::Cow;
use std::path::{Component, Path, PathBuf};

#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStrExt;

pub trait PathExt: AsRef<Path> {
    /// Normalizes a path to use `/` as a separator everywhere, even on platforms
    /// that recognize other characters as separators.
    ///
    /// On Unix, this always succeeds, is zero cost and always returns a slice.
    /// On non-Unix systems, this does a UTF-8 check. If the given OS string
    /// slice is not valid UTF-8, then it is lossily decoded into valid UTF-8
    /// (with invalid bytes replaced by the Unicode replacement codepoint).
    fn to_normalized_string_lossy(&self) -> Cow<'_, str>;

    /// Returns `true` if `self` has a trailing slash.
    ///
    /// On Windows, this is either `\` or `/`. On Unix, this is `/`.
    fn has_trailing_slash(&self) -> bool;

    /// Returns `true` if `self` is equal to the empty path `Path::new("")`.
    fn is_empty(&self) -> bool {
        self.as_ref() == Path::new("")
    }

    /// Remove any dots from the path by popping as needed.
    fn dedot(&self) -> PathBuf {
        dedot_components(self.as_ref().components())
    }

    /// Remove any dots from `self` considered relative to `base` by popping as
    /// needed. That is, this is `base.join(self).dedot()` but more efficient.
    fn dedot_from<B: AsRef<Path>>(&self, base: B) -> PathBuf {
        let (base, path) = (base.as_ref(), self.as_ref());
        let components = path.is_absolute()
            .then(|| path.components().chain(Path::new("").components()))
            .unwrap_or_else(|| base.components().chain(path.components()));

        dedot_components(components)
    }

    /// Strip `prefix` from the `path` and return the remainder.
    ///
    /// If `path` doesn't have a prefix `prefix`, then return `None`.
    fn strip_prefix<B: AsRef<Path>>(&self, prefix: B) -> Option<&Path>;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<P: AsRef<Path>> PathExt for P {
    fn to_normalized_string_lossy(&self) -> Cow<'_, str> {
        // UNIX only uses /, so we're good.
        self.as_ref().to_string_lossy()
    }

    fn has_trailing_slash(&self) -> bool {
        self.as_ref().as_os_str().as_bytes().last() == Some(&b'/')
    }

    fn strip_prefix<B: AsRef<Path>>(&self, prefix: B) -> Option<&Path> {
        use std::ffi::OsStr;

        let prefix = prefix.as_ref().as_os_str().as_bytes();
        let path = self.as_ref().as_os_str().as_bytes();
        if prefix.len() > path.len() || prefix != &path[..prefix.len()] {
            None
        } else {
            Some(&Path::new(OsStr::from_bytes(&path[prefix.len()..])))
        }
    }
}

#[cfg(windows)]
impl<P: AsRef<Path>> PathExt for P {
    fn to_normalized_string_lossy(&self) -> Cow<'_, str> {
        use std::path::is_separator;

        let mut path = self.as_ref().to_string_lossy();
        for i in 0..path.len() {
            let byte = path.as_bytes()[i];
            if byte == b'/' || !byte.is_ascii() || !is_separator(byte as char) {
                continue;
            }

            // SAFETY: We've just checked that `byte` at index `i` is ASCII.
            // Thus, we can safely replace it with any other valid ASCII
            // character. '/' is valid ASCII. Thus, this is safe and correct.
            unsafe { path.to_mut().as_bytes_mut()[i] = b'/'; };
        }

        path
    }

    fn has_trailing_slash(&self) -> bool {
        let last = self.as_ref().as_os_str().encode_wide().last();
        last == Some(b'\\' as u16) || last == Some(b'/' as u16)
    }

    fn strip_prefix<B: AsRef<Path>>(&self, prefix: B) -> Option<&Path> {
        self.as_ref().strip_prefix(prefix.as_ref()).ok()
    }
}

#[cfg(not(any(unix, windows, target_os = "wasi")))]
impl<P: AsRef<Path>> PathExt for P {
    fn to_normalized_string_lossy(&self) -> Cow<'_, str> {
        // UNIX only uses /, so we're good.
        self.as_ref().to_string_lossy()
    }

    fn has_trailing_slash(&self) -> bool {
        self.as_ref().to_str().map_or(false, |s| s.ends_with('/'))
    }

    fn strip_prefix<B: AsRef<Path>>(&self, prefix: B) -> Option<&Path> {
        self.as_ref().strip_prefix(prefix.as_ref()).ok()
    }
}

fn dedot_components<'c>(components: impl Iterator<Item = Component<'c>>) -> PathBuf {
    use std::path::Component::*;

    let mut comps = vec![];
    for component in components {
        match component {
            p@Prefix(_) => comps = vec![p],
            r@RootDir if comps.iter().all(|c| matches!(c, Prefix(_))) => comps.push(r),
            r@RootDir => comps = vec![r],
            CurDir => { },
            ParentDir if comps.iter().all(|c| matches!(c, Prefix(_) | RootDir)) => { },
            ParentDir => { comps.pop(); },
            c@Normal(_) => comps.push(c),
        }
    }

    comps.iter().map(|c| c.as_os_str()).collect()
}

// /// Lossily create a new byte string from a path.
// ///
// /// On Unix, this always succeeds, is zero cost and always returns a slice.
// /// On non-Unix systems, this does a UTF-8 check. If the given OS string
// /// slice is not valid UTF-8, then it is lossily decoded into valid UTF-8
// /// (with invalid bytes replaced by the Unicode replacement codepoint).
// fn to_bytes_lossy(&self) -> Cow<'_, [u8]>;

// fn to_bytes_lossy(&self) -> Cow<'_, [u8]> {
//     match self.as_ref().as_os_str().to_string_lossy() {
//         Cow::Borrowed(x) => Cow::Borrowed(x.as_bytes()),
//         Cow::Owned(x) => Cow::Owned(Vec::from(x)),
//     }
// }

// fn to_bytes_lossy(&self) -> Cow<'_, [u8]> {
//     Cow::Borrowed(self.as_ref().as_os_str().as_bytes())
// }
