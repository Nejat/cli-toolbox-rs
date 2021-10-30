use cli_toolbox::eval;

fn main() {
    eval! { @err-terse println!("this should fail") }

    eval! { @err-verbose println!("this should fail") }

    eval! { @foo println!("this should fail") }
}