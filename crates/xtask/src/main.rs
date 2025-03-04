use dump::dump;
use parse::parse_modules;

mod dirs;
mod dump;
mod parse;

fn main() {
    let modules = parse_modules();
    dump(modules).unwrap();
}
