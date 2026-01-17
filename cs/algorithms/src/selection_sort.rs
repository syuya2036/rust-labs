pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let min = (i..arr.len()).min_by_key(|&j| &arr[j]).unwrap();
        arr.swap(i, min);
    }
}
