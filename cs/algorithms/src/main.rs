use algorithms::selection_sort::selection_sort;
fn main() {
    let mut arr = vec![10, 2, 8, 2, 9, 3, 4];
    selection_sort(&mut arr);
    println!("{:?}", arr);
}
