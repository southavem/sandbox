fn main() {
    let number_list = vec![10, 20, 30, 40, 50];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 50);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
