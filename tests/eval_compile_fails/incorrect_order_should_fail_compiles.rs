use cli_toolbox::eval;

fn main() {
    eval! {
        @verbose println!("verbose message: {}", 42);
        @terse println!("terse message: {}", 42)
    }
}