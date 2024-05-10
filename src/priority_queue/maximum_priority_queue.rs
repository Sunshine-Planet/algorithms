use crate::heap::max_heap;

pub fn heap_maximum(array: &[i32]) -> i32 {
    array[0]
}

pub fn heap_extract_max(mut array: Box<[i32]>) -> (i32, Box<[i32]>) {
    if array.is_empty() {
        eprintln!("heap underflow");
        panic!();
    }
    let len = array.len();
    let max = array[0];
    array[0] = array[len - 1];
    let mut vec: Vec<i32> = array.into_vec();
    // 以后可以增加判断弹出后为空的情况
    vec.pop();
    max_heap::max_heapify(&mut vec, 0);
    //let new_array = vec.into_boxed_slice();
    let new_array = vec.into_boxed_slice();
    (max, new_array)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_heap_maximum() {
        let query = [16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
        let except = 16;
        assert_eq!(except, heap_maximum(&query));
    }

    #[test]
    fn test_heap_extract_max() {
        let query = [16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
        let query = Box::new(query);
        let except_vec = vec![14, 8, 10, 4, 7, 9, 3, 2, 1];
        let except = (16, Box::from(except_vec));
        let (max, new_array) = heap_extract_max(query);
        assert_eq!(except.0, max);
        assert_eq!(except.1, new_array);
        let new_array = new_array.into_vec();
        let except_vec = vec![14, 8, 10, 4, 7, 9, 3, 2, 1];
        assert_eq!(except_vec, new_array);
        //移除注释以在编译时检查所有权是否转移
        //query;
    }
}
