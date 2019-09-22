extern crate lalrpop;

fn main() {
    lalrpop::process_root().expect("lalrpop::process_root");
}
