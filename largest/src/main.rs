use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![32, 44, 424, 424, 128, 42];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

}
