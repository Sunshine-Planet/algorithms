use super::*;

pub fn max_heapify(array: &mut [i32], i: usize) {
    let len = array.len();
    let l = left(i);
    let r = right(i);
    let mut largest = i;
    if l < len && array[l] > array[largest] {
        largest = l;
    }
    // 这里不要使用else，因为既要判断左子树也要判断右字数，选择最大的
    if r < len && array[r] > array[largest] {
        largest = r;
    }

    if largest != i {
        array.swap(i, largest);
        max_heapify(array, largest);
    }
}

/// 叶节点可以理解为没有孩子的节点，而就不能有左孩子，所以
/// 2×i+1≥n，ceil((n−1)/2), 用编程语言描述就是 ((n - 1) // 2) + 1
/// 叶节点范围从 ceil((n−1)/2) 到 𝑛−1
/// 而应该遍历的是非叶节点范围
pub fn build_max_heapify(array: &[i32]) -> Vec<i32> {
    let mut array = array.to_vec();
    let len = array.len();
    for i in (0..=((len - 1) / 2)).rev() {
        //println!("{}",i);
        max_heapify(&mut array, i);
    }
    array
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_max_heapify() {
        let mut query = [16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
        let except = [16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
        max_heapify(&mut query, 1);
        assert_eq!(except, query);
    }

    #[test]
    fn test_build_max_heapify() {
        let query = [4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        let except = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
        let result = build_max_heapify(&query);
        assert_eq!(except, result);
    }
}
