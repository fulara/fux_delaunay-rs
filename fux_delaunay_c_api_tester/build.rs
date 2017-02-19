extern crate gcc;

fn main() {
    gcc::compile_library("libc_api_tester.a", &["src/api.c"]);
}