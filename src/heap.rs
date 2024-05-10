pub mod max_heap;

pub fn parent(i: usize) -> usize {
    match i {
        0 => 0,
        _ => (i - 1) / 2,
    }
}

pub fn left(i: usize) -> usize {
    2 * i + 1
}

pub fn right(i: usize) -> usize {
    2 * i + 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parent() {
        let query = [0, 1, 2, 3, 4, 5];
        let except = [0, 0, 0, 1, 1, 2];
        for (i, (q, e)) in query
            .into_iter()
            .map(parent)
            .zip(except.into_iter())
            .enumerate()
        {
            //print!("index:{}, ", i);
            assert!(
                q == e,
                "Mismatch at index {}: expected {}, but found {}",
                i,
                e,
                q
            );
        }
        //assert_eq!(except, parent(query));
    }

    #[test]
    fn test_left() {
        let query = [0, 1, 2, 3, 4, 5];
        let except = [1, 3, 5, 7, 9, 11];
        for (i, (q, e)) in query
            .into_iter()
            .map(left)
            .zip(except.into_iter())
            .enumerate()
        {
            //print!("index:{}, ", i);
            assert!(
                q == e,
                "Mismatch at index {}: expected {}, but found {}",
                i,
                e,
                q
            );
        }
        //assert_eq!(except, parent(query));
    }

    #[test]
    fn test_right() {
        let query = [0, 1, 2, 3, 4, 5];
        let except = [2, 4, 6, 8, 10, 12];
        for (i, (q, e)) in query
            .into_iter()
            .map(right)
            .zip(except.into_iter())
            .enumerate()
        {
            //print!("index:{}, ", i);
            assert!(
                q == e,
                "Mismatch at index {}: expected {}, but found {}",
                i,
                e,
                q
            );
        }
        //assert_eq!(except, parent(query));
    }
}
