pub fn bubblesort(array: &[i32]) -> Vec<i32> {
    let mut array = array.to_vec();
    let len = array.len();

    for i in 0..len {
        for j in 0..(len - i - 1) {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }  
        }
    }
    array
    //vec![1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bubblesort() {
        let query = [5, 2, 4, 6, 1, 3];
        let expected = vec![1, 2, 3, 4, 5, 6];

        assert_eq!(expected, bubblesort(&query));
    }
}
