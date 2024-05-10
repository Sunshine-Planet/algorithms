fn compute_trends(array: &[i32]) -> Vec<i32> {
    // let mut result = Vec::with_capacity(array.len() - 1);
    // for i in 0..array.len() - 1 {
    //     let delta = array[i+1] - array[i];
    //     result.push(delta);
    // }
    // result

    // array.windows(2) // 用 `windows` 创建滑动窗口
    // .map(|w| w[1] - w[0]) // 计算差值
    // .collect() // 转换为 `Vec`

    array
        .iter()
        .zip(array.iter().skip(1)) // zip() 返回的是元组，闭包参数记得加括号
        .map(|(a, b)| b - a)
        .collect()
}

pub fn find_max(array: &[i32]) -> (usize, usize, i32) {
    let len = array.len();
    let mut max_diff = 0;
    let mut start_index = 0;
    let mut end_index = 0;
    for i in 0..len {
        let min_val = array[i];
        for j in i..len {
            let max_val = array[j];
            let current_diff = max_val - min_val;
            if max_diff < current_diff {
                max_diff = current_diff;
                start_index = i;
                end_index = j
            }
        }
    }

    (start_index, end_index, max_diff)

    // let mut min_val = array[0];
    // let mut max_diff = 0;
    // let mut min_index = 0;
    // let mut start_index = 0;
    // let mut end_index = 0;

    // for i in 1..array.len() {
    //     if array[i] - min_val > max_diff {
    //         max_diff = array[i] - min_val;
    //         start_index = min_index;
    //         end_index = i;
    //     }
    //     // 在一次循环下，如果当前start下标的差值不超过接下来min的插值，即没有超过更高的max的话，就不用更新start和max
    //     if array[i] < min_val {
    //         min_val = array[i];
    //         min_index = i;
    //     }
    // }

    // (start_index, end_index, max_diff)

    // let (max_diff, _, _, start_index, end_index) = array.iter().enumerate().fold(
    //     (0, array[0], 0, 0, 0), // (最大差值, 最小值, 最小值索引, 起始索引, 结束索引)
    //     |(max_diff, min_val, min_index, start_index, end_index), (i, &val)| {
    //         let new_min_val = min_val.min(val); // 更新最小值
    //         let new_min_index = if val < min_val { i } else { min_index };

    //         let diff = val - new_min_val; // 计算差值
    //         if diff > max_diff {
    //             // 如果差值更大，更新最大差值
    //             (diff, new_min_val, new_min_index, new_min_index, i)
    //         } else {
    //             // 保持原有状态
    //             (max_diff, new_min_val, new_min_index, start_index, end_index)
    //         }
    //     },
    // );

    // // 最终返回所需的部分：起始索引、结束索引和最大差值
    // (start_index, end_index, max_diff)
}
/// 限制条件，求出的最大子数组必须跨越中点。
fn find_max_crossing_subarray(
    array: &[i32],
    low: usize,
    mid: usize,
    high: usize,
) -> (usize, usize, i32) {
    let mut left_sum: Option<i32> = None;
    let mut sum = 0;
    let mut max_left = 0;
    for i in (low..=mid).rev() {
        sum += array[i];
        // if left_sum.is_none() || sum > left_sum.unwrap() {
        //     left_sum = Some(sum);
        //     max_left = i;
        // }
        match left_sum {
            Some(current_sum) => {
                if sum > current_sum {
                    left_sum = Some(sum);
                    max_left = i;
                }
            }
            None => {
                left_sum = Some(sum);
                max_left = i;
            }
        }
    }

    let mut right_sum: Option<i32> = None;
    let mut sum = 0;
    let mut max_right = 0;
    for j in (mid + 1)..=high {
        sum += array[j];
        match right_sum {
            Some(current_sum) => {
                if sum > current_sum {
                    right_sum = Some(sum);
                    max_right = j;
                }
            }
            None => {
                right_sum = Some(sum);
                max_right = j;
            }
        }
    }
    match (max_left, max_right, left_sum, right_sum) {
        (_, _, Some(left_sum), Some(right_sum)) => (max_left, max_right, right_sum + left_sum),
        _ => panic!(),
    }
    //(1, 1, 1)
}

pub fn find_maximun_subarray(array: &[i32], low: usize, high: usize) -> (usize, usize, i32) {
    if high == low {
        (low, high, array[low])
    } else {
        let mid = (low + high) / 2;
        let (left_low, left_high, left_sum) = find_maximun_subarray(array, low, mid);
        let (right_low, right_high, right_sum) = find_maximun_subarray(array, mid + 1, high);
        let (cross_low, cross_high, cross_sum) = find_max_crossing_subarray(array, low, mid, high);
        if left_sum >= right_sum && left_sum >= cross_sum {
            (left_low, left_high, left_sum)
        } else if right_sum >= left_sum && right_sum >= cross_sum {
            (right_low, right_high, right_sum)
        } else {
            (cross_low, cross_high, cross_sum)
        }
    }
}

#[cfg(test)]
mod test {

    use super::{find_maximun_subarray, *};

    #[test]
    fn test_compute_trends() {
        let query = [
            100, 113, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 97,
        ];
        let result: Vec<i32> = vec![
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];
        assert_eq!(result, compute_trends(&query));
    }

    #[test]
    fn test_find_max() {
        let query = [10, 11, 7, 10, 6];
        let result = (2usize, 3usize, 3i32);
        assert_eq!(result, find_max(&query));
        let query = [3, 10, 1, 2];
        let result = (0usize, 1usize, 7i32);
        assert_eq!(result, find_max(&query));
    }

    #[test]
    fn test_find_max_crossing_subarray() {
        let query = [
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];
        let except = (7, 10, 43);
        for mid in 7..=9 {
            //println!("{}", mid);
            let result = find_max_crossing_subarray(&query, 0, mid, 15);
            assert_eq!(except, result);
        }
    }

    #[test]
    fn test_find_maximun_subarray() {
        let query = [
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];
        let except = (7, 10, 43);
        let high = query.len() - 1;
        let result = find_maximun_subarray(&query, 0, high);
        assert_eq!(except, result);
    }
}
