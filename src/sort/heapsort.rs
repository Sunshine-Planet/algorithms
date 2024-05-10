use crate::heap::max_heap;

pub fn heapsort(array: &[i32]) -> Vec<i32> {
    let mut array = max_heap::build_max_heapify(array);
    //let mut len = array.len();
    let len = array.len();
    for i in (1..len).rev() {
        //println!("{}", i);
        array.swap(0, i);
        //print!("{:?}    ", array);
        //let array = &mut array[..len - 1];
        //let len = array.len();
        //max_heap::max_heapify(&mut array[..len - 1], 0);
        max_heap::max_heapify(&mut array[..i], 0);
        //println!("{:?}", array);
        //len -= 1;
    }
    array
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_heapsort() {
        let query = [16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
        let except = vec![1, 2, 3, 4, 7, 8, 9, 10, 14, 16];
        assert_eq!(except, heapsort(&query));
    }
}
