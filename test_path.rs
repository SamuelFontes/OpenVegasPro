use std::path::Path;

fn main() {
    let p = Path::new("test.mp4");
    println!("{:?}", p.extension());
}
