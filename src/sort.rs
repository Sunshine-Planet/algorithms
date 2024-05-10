pub mod sort_hello;
pub mod insertion_sort;
pub mod merge_sort;
pub mod bubblesort;
pub mod heapsort;

pub fn hello(){
    println!("sort 模块");
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test(){
        hello();
    }
}