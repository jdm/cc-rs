extern crate cc;
extern crate tempdir;

use std::env;
use std::path::Path;
use std::ffi::OsString;

mod support;
use support::Test;

#[test]
fn main() {
    ccache();
    distcc();
    ccache_spaces();
    ccache_env_flags();
    leading_spaces();
    extra_flags();
    path_to_ccache();
}

fn ccache() {
    let test = Test::gnu();

    env::set_var("CC", "ccache cc");
    let compiler = test.gcc().file("foo.c").get_compiler();

    assert_eq!(compiler.path(), Path::new("cc"));
}

fn ccache_spaces() {
    let test = Test::gnu();
    test.shim("ccache");

    env::set_var("CC", "ccache        cc");
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), Path::new("cc"));
}

fn distcc() {
    let test = Test::gnu();
    test.shim("distcc");

    env::set_var("CC", "distcc cc");
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), Path::new("cc"));
}

fn ccache_env_flags() {
    let test = Test::gnu();
    test.shim("ccache");

    env::set_var("CC", "ccache lol-this-is-not-a-compiler");
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), Path::new("lol-this-is-not-a-compiler"));
    assert_eq!(
        compiler.cc_env(),
        OsString::from("ccache lol-this-is-not-a-compiler")
    );
    assert!(
        compiler
            .cflags_env()
            .into_string()
            .unwrap()
            .contains("ccache") == false
    );
    assert!(
        compiler
            .cflags_env()
            .into_string()
            .unwrap()
            .contains(" lol-this-is-not-a-compiler") == false
    );

    env::set_var("CC", "");
}

fn leading_spaces() {
    let test = Test::gnu();
    test.shim("ccache");

    env::set_var("CC", " test ");
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), Path::new("test"));

    env::set_var("CC", "");
}

fn extra_flags() {
    let test = Test::gnu();
    test.shim("ccache");

    env::set_var("CC", "ccache cc -m32");
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), Path::new("cc"));
}

fn path_to_ccache() {
    let test = Test::gnu();
    test.shim("ccache");

    env::set_var("CC", "/path/to/ccache.exe cc -m32");
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), Path::new("cc"));
    assert_eq!(
        compiler.cc_env(),
        OsString::from("/path/to/ccache.exe cc -m32"),
    );
}
