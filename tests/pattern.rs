#[macro_use] mod macros;

#[test]
fn test_pattern_matches() {
    assert_matches!(""; "*.rs", "**.rs", "/**/*.rs", "**/*.rs" => [
        ".rs",
        "foo.rs",
        "bar.rs",
        "a/.rs",
        "a/foo.rs",
        "a/foo.rs/",
        "a/b/foo.rs",
        "/a/b/foo.rs",
        "/a///b/foo.rs",
    ], [
        ".html",
        "foo.html",
        "bar.json",
        "a/b/c",
        "cat/and/dogs/are/good",
        "foo.rs/a",
        "bar/baz",
        "foo.html",
        "rs",
        "",
        "foo.rs/bar",
    ]);

    assert_matches!(""; "/*.rs" => [
        ".rs",
        "foo.rs",
        "bar.rs",
        "bar.rs/",
        ".rs/",
    ], [
        "a/.rs",
        "a/foo.rs",
        "a/foo.rs/",
        "a/b/foo.rs",
        "/a/b/foo.rs",
        "/a///b/foo.rs",
    ]);

    assert_matches!(""; "*.rs/", "**/*.rs/", "/**/*.rs/" => [
        "foo.rs/",
        "bar.rs/",
        "a/b/baz.rs/",
    ], [
        "foo.rs",
        "bar.rs",
        "a/b/baz.rs",
        "foo.rs/bar.rs",
        "bar.rs/baz",
        "a/b/c/",
        "a.rs",
    ]);

    assert_matches!(""; "/*.rs/" => [
        "foo.rs/",
        "bar.rs/",
        ".rs/",
    ], [
        "foo.rs",
        "foo/bar.rs/",
        "/bar.rs/",
        ".rs",
    ]);

    assert_matches!("/bar"; "*.rs", "**.rs", "/**/*.rs", "**/*.rs" => [
        "/bar/foo.rs",
        "/bar/foo.rs/",
        "/bar/baz/foo.rs",
        "/bar/baz/.rs",
        "/bar/baz/cat.rs",
    ], [
        "bar/foo.rs",
        "other/bar.rs",
        "other/bar/.rs",
        "other/bar/foo.rs",
        ".rs",
        "foo.rs",
        "bar.rs",
        "a/.rs",
        "a/foo.rs",
        "a/foo.rs/",
        "a/b/foo.rs",
        "/a/b/foo.rs",
        "/a///b/foo.rs",
        ".html",
        "foo.html",
        "bar.json",
        "a/b/c",
        "cat/and/dogs/are/good",
        "foo.rs/a",
        "bar/baz",
        "foo.html",
        "rs",
        "",
        "foo.rs/bar",
    ]);

    assert_matches!("/"; "*.rs", "**.rs", "/**/*.rs", "**/*.rs" => [
        "/foo.rs",
        "/foo.rs/",
        "/baz/foo.rs",
        "/baz/.rs",
        "/baz/cat.rs",
        "/a/b/c/d/../../.rs",
        "/a/b/foo.rs",
        "/a///b/foo.rs",
    ], [
        "bar/foo.rs",
        "other/bar.rs",
        "other/bar/.rs",
        "other/bar/foo.rs",
        ".rs",
        "foo.rs",
        "bar.rs",
        "a/.rs",
        "a/foo.rs",
        "a/foo.rs/",
        ".html",
        "foo.html",
        "bar.json",
        "a/b/c",
        "cat/and/dogs/are/good",
        "foo.rs/a",
        "bar/baz",
        "foo.html",
        "rs",
        "",
        "foo.rs/bar",
    ]);

    assert_matches!("/bar"; "/foo" => [
        "/bar/foo",
        "/bar/foo/",
    ], [
        "/bar/foo.rs",
        "/bar/foo.rs/",
        "/bar/baz/foo.rs",
        "/bar/baz/.rs",
        "/bar/baz/cat.rs",
        "bar/foo",
        "bar/foo.rs",
        "other/bar.rs",
        "other/bar/.rs",
        "other/bar/foo.rs",
        ".rs",
        "foo.rs",
        "bar.rs",
        "a/.rs",
        "a/foo.rs",
        "a/foo.rs/",
        "a/b/foo.rs",
        "/a/b/foo.rs",
        "/a///b/foo.rs",
        ".html",
        "foo.html",
        "bar.json",
        "a/b/c",
        "cat/and/dogs/are/good",
        "foo.rs/a",
        "bar/baz",
        "foo.html",
        "rs",
        "",
        "foo.rs/bar",
    ]);

    assert_matches!(""; "foo//" => [ "foo//" ], [ "foo/" ]);

    assert_matches!(""; "foo/" => [
        "foo/",
        "a/foo/",
        "a/b/foo/",
        "b/foo/",
        "/b/c/foo/",
    ], [
        "foo/bar",
        "foo/foo",
        "foo",
        "/foo",
    ]);

    assert_matches!("/bar"; "/foo/" => [
        "/bar/foo/",
    ], [
        "/bar/foo.rs",
        "/bar/baz/foo",
        "/bar/baz/foo/",
        "foo.rs/a",
        "bar/baz",
        "foo.html",
        "rs",
        "foo.rs/bar",
        "/baz/foo/",
        "/bar/foo",
        "bar/foo/",
    ]);
}
