fn main() {
    let message = "Hello Debug!";
    let a = add(2,3);
    println!("{} with a={}", message,a);
}

fn add(i:i32 ,j: i32) -> i32{
    i+j
}