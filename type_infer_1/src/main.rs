
fn main() {

    let vec: Vec<i32> = vec![1,2,3];
    let coll = vec.iter().collect();
    let inferred_vec = to_vec(coll);

    println!("{:?}", inferred_vec);
}

fn to_vec<T>(vec : Vec<T>) -> Vec<T> {
    vec
}