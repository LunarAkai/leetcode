fn main() {
    let nums_1 = vec![-1,0,3,5,9,12];
    let target_1 = 9;

    let index_1 = search(nums_1, target_1);
    println!("Found value at: {}", index_1);
}

// outputs the index of target or -1
fn search(nums: Vec<i32>, target: i32) -> i32 {    
    let mut left = 0;
    let mut right = nums.len() - 1;

    println!("len: {}", right);

    while left <= right {

        let mid = (left + right)/2;

        if target == nums[mid] {
            return mid as i32;
        }
        if target > nums[mid] {
            left = mid + 1;
        } else {
            if mid.overflowing_add_signed(-1) == (usize::MAX, true)  {
                return -1;
            } else {
                right = mid - 1;
            }
        }
    }
    -1
} 

 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums_1 = vec![-1,0,3,5,9,12];
        let target_1 = 9;

        assert_eq!(search(nums_1, target_1), 4);
    }

    #[test]
    fn test_example_2() {
        let nums_2 = vec![-1,0,3,5,9,12];
        let target_2 = 2;

        assert_eq!(search(nums_2, target_2), -1);
    }

    #[test]
    fn test_example_3() {
        let nums_2 = vec![5];
        let target_2 = -5;

        assert_eq!(search(nums_2, target_2), -1);
    }
}