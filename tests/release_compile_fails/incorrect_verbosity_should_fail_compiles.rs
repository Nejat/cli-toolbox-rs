use cli_toolbox::release;

fn main() {
    release! { @err-terse println!("this should fail") }

    release! { @err-verbose println!("this should fail") }

    release! { @foo println!("this should fail") }
}