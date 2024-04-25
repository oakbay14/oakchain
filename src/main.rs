use std::fmt::Display;

fn main() {
    let mut oak: [i32; 7]=[1,13,15,7,11,20,33];
    bubble_sort(&mut oak);
    println!("the sort result is {:?}",oak);
}
fn bubble_sort(oak: &mut [i32]) {
    let mut swapped = true;
    while swapped {
    swapped = false;
    for i in 1..oak.len() {
    if oak[i - 1] > oak[i] {
    oak.swap(i - 1, i);
    swapped = true;
    }
    }
    }
    }
    