fn return_inex(data: &Vec<i32>) -> Option<i32> {
    if data.len() <= 2 {
        None
    } else {
        Some(data[3])
    }
}

fn main() {
    println!("testing");
    let my_vec = vec![4; 1];
    println!("value :   {:?}", my_vec);

    match return_inex(&my_vec) {
        Some(value) => println!("value = {}", value),
        None => println!("not found in index"),
    }

    println!("Programs end");
}
