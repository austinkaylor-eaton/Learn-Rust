macro_rules! my_macro {
    () => {
        "Hello world!"
    };
    ($val:expr) => {
        concat!("Hello ", $val)
    };
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
