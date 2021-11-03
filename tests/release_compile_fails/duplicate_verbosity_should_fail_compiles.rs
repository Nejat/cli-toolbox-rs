use cli_toolbox::release;

fn main() {
    release! {
        @terse println!("terse message: {}", 42);
        @terse println!("terse message: {}", 42)
    }

    release! {
        @verbose println!("verbose message: {}", 42);
        @verbose println!("verbose message: {}", 42)
    }
}