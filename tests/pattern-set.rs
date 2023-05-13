#[macro_use] mod macros;

#[test]
fn test_pattern_set_matches() {
    assert_set_matches!(
        {
            ""; "*.rs", "!foo.rs",
        },
        [
            "bar.rs",
            "baz.rs",
            "/bar.rs",
            "/fo.rs",
            "oo.rs",
            "a/foo/.rs",
            "a/.rs",
            "any/foo.rs/other.rs",
        ],
        [
            "foo.rs",
            "a/foo.rs",
            "foo/foo.rs",
            "/foo/foo.rs",
            "/foo.rs",
            "/foo.rs/",
        ]
    );

    assert_set_matches!(
        {
            "/root/scratch"; "foo*", "!*.html",
        },
        [
            "/root/scratch/foo",
            "/root/scratch/foobar",
            "/root/scratch/foo",
            "/root/scratch/foo.rs",
        ],
        [
            "foo",
            "foo.html",
            "/root/foobar",
            "/root/foo.html",
            "/root/scratch/foo.html",
            "/root/scratch/foobar.html",
            "/root/scratch/candy/foobar.html",
        ]
    );

    assert_set_matches!(
        {
            ""; "a/b/c/**", "!important",
        },
        [
            "a/b/c/d",
            "a/b/c/hi",
            "a/b/c/foo/bar/baz",
            "a/b/c/notimportant/hi",
            "a/b/c/important/sorry",
        ],
        [
            "a",
            "a/",
            "a/b",
            "a/b/",
            "a/b/c",
            "a/b/c/",
            "a/b/c/important",
            "a/b/c/important/",
            "a/b/c/foobar/important",
        ]
    );

    assert_set_matches!(
        {
            "/root/a"; "foo*", "!foobar",
        },
        {
            "/root/b"; "bar*", "!barbaz",
        },
        [
            "/root/a/foo",
            "/root/a/foo/",
            "/root/a/foobarbaz",

            "/root/b/bar",
            "/root/b/bar/",
            "/root/b/barbarbaz",
        ],
        [
            "/root/foo",
            "/root/foobarbaz",
            "root/foo",
            "root/foobar",
            "root/foobarbaz",
            "foo",
            "foobar",
            "foobarbaz",
            "/root/bar",
            "/root/barbaz",

            "/root/a/foo/bar",
            "/root/a/foo//",
            "/root/a/foobar",
            "/root/a/foobar/",
            "/root/a/b/foobar",
            "/root/a/b/foobar/",

            "/root/a/bar",
            "/root/a/barbaz",

            "root/b/bar",
            "root/b/bar/",
            "root/b/barbarbaz",
            "bar",
            "bar/",
            "barbarbaz",

            "/root/b/foo",
            "/root/b/foobar",

            "/root/hi/foo",
            "/root/hi/fobar",
            "/root/hi/bar",
            "/root/hi/barbaz",
        ]
    );

    // Test later rules taking precedence.
    assert_set_matches!(
        { "/root/html"; "foo*" },
        { "/root"; "!foo", },
        [
            "/root/html/foobar",
            "/root/html/bar/foobar",
            "/root/html/foocice",
            "/root/html/hi/there/foocice",
            "/root/html/hi/there/food",
        ],
        [
            "/root/foo",
            "/root/html/foo/bar",
            "/root/html/bar/foobar/foo",
            "/root/html/hi/there/foo",
            "/root/html/foo",
        ]
    );

    // Test later rules taking precedence.
    assert_set_matches!(
        { "/root"; "foo*" },
        { "/root/html"; "!foo*", "!*bar" },
        [
            "/root/foo",
            "/root/foobar",
        ],
        [
            "/root/catbar",
            "/root/html/foo",
            "/root/html/foobar",
            "/root/html/catbar",
            "/root/html/hi/foobar",
            "/root/html/hi/catbar",
        ]
    );
}
