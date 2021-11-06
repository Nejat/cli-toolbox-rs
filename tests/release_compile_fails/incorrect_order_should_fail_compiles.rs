use cli_toolbox::release;

fn main() {
    release! {
        @verbose println!("verbose message: {}", 42);
        @terse println!("terse message: {}", 42)
    }
}