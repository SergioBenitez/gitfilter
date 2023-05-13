#![allow(unused_macros)]

macro_rules! is_match {
    ($matcher:expr => $path:expr) => ({
        use gitfilter::PathExt;

        let is_dir = $path.has_trailing_slash();
        let path = $path.strip_suffix('/').unwrap_or($path);
        $matcher.matches(path, is_dir)
    })
}

macro_rules! assert_match {
    ($matcher:expr => $path:expr) => {
        if !is_match!($matcher => $path) {
            panic!("{} failed to match {:?}", $matcher, $path);
        }
    }
}

macro_rules! assert_no_match {
    ($matcher:expr => $path:expr) => {
        if is_match!($matcher => $path) {
            panic!("{} unexpectedly matched {:?}", $matcher, $path);
        }
    }
}

macro_rules! assert_matches {
    ($root:expr; $($pattern:expr),+ => [ $($good:expr),* $(,)? ], [ $($bad:expr),* $(,)? ]) => (
        let patterns = &[$($pattern),+];
        for string in patterns {
            let matcher = gitfilter::Pattern::new(string, $root)
                .unwrap()
                .into_matcher()
                .unwrap();

            $(assert_match!(matcher => $good);)*
            $(assert_no_match!(matcher => $bad);)*
        }
    )
}

macro_rules! assert_set_matches {
    (
        $({ $root:expr ; $($pattern:expr),* $(,)? }),+  $(,)? ,
        [ $($good:expr),* $(,)? ], [ $($bad:expr),* $(,)? ]
    ) => (

        let patterns: Vec<gitfilter::Pattern> = vec![$(
            $(gitfilter::Pattern::new($pattern, $root).unwrap()),*
        ),+];

        let matcher = gitfilter::PatternSet::from(patterns).into_matcher().unwrap();
        $(assert_match!(matcher => $good);)*
        $(assert_no_match!(matcher => $bad);)*
    )
}
