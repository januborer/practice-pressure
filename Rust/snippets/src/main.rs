mod closure;
mod generics;

fn main() {
    println!(
        "The largest num is :{}",
        generics::largest(&vec![
            23, 32, 4, 34, 45, 5, 6, 5, 34, 34, 34, 3434, 34, 343434
        ])
    )
}
