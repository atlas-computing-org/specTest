

/* in the output list, given positions in in the list, i and j, if i is smaller than j, then the value 
   associate to i have to be smaller than j.
   
   the output list has to have the same values of the input list, just in different order. Both need to have 
   the same size. */
fn merge_sort<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    // Base case: a list of size 0 or 1 is already sorted
    if list.len() <= 1 {
        return list.to_vec();
    }

    // Split the list into two halves
    let mid = list.len() / 2;
    let left = merge_sort(&list[..mid]);
    let right = merge_sort(&list[mid..]);

    // Merge the two sorted halves
    merge(&left, &right)
}

/* the output list need to have the size equal the sum of the sizes of the input lists. */
fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i].clone());
            i += 1;
        } else {
            merged.push(right[j].clone());
            j += 1;
        }
    }

    // Append any remaining elements
    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);

    merged
}

fn main() {
    let nums = vec![38, 27, 43, 3, 9, 82, 10];
    let sorted = merge_sort(&nums);
    println!("Original: {:?}", nums);
    println!("Sorted:   {:?}", sorted);
}

