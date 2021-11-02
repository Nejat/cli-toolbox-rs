use cli_toolbox::eval;

fn main() {
    eval! {
        @terse println!("terse message: {}", 42);
        @terse println!("terse message: {}", 42)
    }

    eval! {
        @verbose println!("verbose message: {}", 42);
        @verbose println!("verbose message: {}", 42)
    }
}