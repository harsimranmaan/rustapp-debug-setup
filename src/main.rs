fn main() {
    let message = "Hello Debug!";
    let a = add(2,3);
    let kind_words = vec!["Nice job", "It works"];
    println!("{} with a={}.\n {:#?}", message,a, kind_words);
}

fn add(i:i32 ,j: i32) -> i32{
    i+j
}