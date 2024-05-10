pub fn insertion_sort(array: &[i32]) -> Vec<i32> {
    //let mut result = [0; N];
    // for i in 0..N { result[i] = (i + 1) as i32; }
    let mut array = array.to_vec(); // 转换为向量
                                    //let len = array.len();
    for j in 1..array.len() {
        let key = array[j];
        let mut i = j;
        while i > 0 && array[i - 1] > key {
            //这里换成小于号就是降序
            //key 不能换成array[j]，所有权
            array[i] = array[i - 1];
            i -= 1;
        }
        array[i] = key;
    }
    array
}

pub fn insertion_sort_asc(array: &[i32]) -> Vec<i32> {
    let mut array = array.to_vec();
    let len = array.len();
    for j in (0..(len - 1)).rev() {
        //println!("{}", j);
        let key = array[j];
        let mut i = j;
        while i < (len - 1) && key < array[i + 1] {
            //key 不能换成array[j]，所有权
            array[i] = array[i + 1];
            i += 1;
        }
        array[i] = key;
    }
    array
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let query = [5, 2, 4, 6, 1, 3];
        let expected = vec![1, 2, 3, 4, 5, 6];

        assert_eq!(expected, insertion_sort(&query));
    }

    #[test]
    fn test_insertion_sort_asc() {
        let query = [5, 2, 4, 6, 1, 3];
        let expected = vec![6, 5, 4, 3, 2, 1];

        assert_eq!(expected, insertion_sort_asc(&query));
    }
}
