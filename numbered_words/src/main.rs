use std::collections::HashMap;

fn main() {

    let text = "hello world wonderful world";

    // let mut map = HashMap::new();
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    let split= text.split_ascii_whitespace().collect::<Vec<_>>();
    let map: HashMap<_, _> = split.iter().zip(1..split.len()).collect();

    println!("{:?}", map);
}
