use alloc::string::String;

pub async fn echo(args: String) {
    crate::println!("{}", args);
}
