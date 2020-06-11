
// Include the `items` module, which is generated from items.proto.
pub mod types {
    include!(concat!(env!("OUT_DIR"), "/types.rs"));
}

fn main() {
    println!("Hello, world!");
}
