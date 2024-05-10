fn merge_without_none(array: &mut [i32], p: usize, q: usize, r: usize) {
    let n1 = q - p + 1;
    let n2 = r - q;

    let mut left: Vec<i32> = vec![0; n1];
    let mut right: Vec<i32> = vec![0; n2];
    left.copy_from_slice(&array[p..=q]);
    right.copy_from_slice(&array[q + 1..=r]);
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }
    if i < left.len() {
        merged.extend_from_slice(&left[i..]);
    } else {
        merged.extend_from_slice(&right[j..]);
    }

    array[p..=r].copy_from_slice(&merged);
}

fn merge(array: &mut [i32], p: usize, q: usize, r: usize) {
    let n1 = q - p + 1;
    let n2 = r - q;
    // let mut left: Vec<Option<i32>> = vec![None; n1 + 1];
    // let mut right: Vec<Option<i32>> = vec![None; n2 + 1];
    // for i in 0..n1 {
    //     left[i] = array.get(p + i).copied();
    // }
    // for j in 0..n2 {
    //     right[j] = array.get(q + j + 1).copied();
    // }

    let mut left: Vec<Option<i32>> = (0..n1).map(|i| array.get(p + i).copied()).collect();
    let mut right: Vec<Option<i32>> = (0..n2).map(|j| array.get(q + 1 + j).copied()).collect();
    left.push(None);
    right.push(None);
    let (mut i, mut j) = (0, 0);
    for k in p..=r {
        if (left[i] <= right[j] && left[i].is_some()) || right[j].is_none() {
            array[k] = left[i].unwrap();
            i += 1;
        } else {
            array[k] = right[j].unwrap();
            j += 1;
        }
    }
    //let mut merged: Vec<i32> = Vec::with_capacity(n1 + n2);
    // while i < n1 || j < n2 {
    //     if (right[j].is_none()) || (left[i].is_some() && left[i] <= right[j]) {
    //         merged.push(left[i].unwrap());
    //         i += 1;
    //     } else {
    //         merged.push(right[j].unwrap());
    //         j += 1;
    //     }
    // }

    //array[p..=r].copy_from_slice(&merged);
}

pub fn merge_sort(array: &mut [i32], p: usize, r: usize) {
    if p < r {
        let q = (p + r) / 2;
        merge_sort(array, p, q);
        merge_sort(array, q + 1, r);
        merge(array, p, q, r);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge() {
        let mut query = [2, 1];
        let except = [1, 2];
        merge(&mut query, 0, 0, 1);
        assert_eq!(except, query);
    }

    #[test]
    fn test_merge_without_none() {
        let mut query = [2, 1];
        let except = [1, 2];
        merge_without_none(&mut query, 0, 0, 1);
        assert_eq!(except, query);
    }

    #[test]
    fn test_merge_sort() {
        let mut query = [2, 4, 5, 7, 1, 2, 3, 6];
        let start = 0;
        let end = query.len() - 1;
        let result = [1, 2, 2, 3, 4, 5, 6, 7];
        merge_sort(&mut query, start, end);
        assert_eq!(result, query);
    }
}
