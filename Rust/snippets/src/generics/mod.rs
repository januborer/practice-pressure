pub fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}
