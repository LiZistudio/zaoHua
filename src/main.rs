use std::fs;

fn main() {
    let engine_title:String = String::from("造化钟神秀，阴阳割昏晓。Welcome to zaoHua game engine.");
    println!("{:?}",engine_title);
    //println!("In file {}", "zaoHua.txt");
    let contents = fs::read_to_string("zaoHua.txt")
        .expect("Something went wrong reading the file");
    println!("{}", contents);
}
