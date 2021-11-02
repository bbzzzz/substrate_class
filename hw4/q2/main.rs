fn main() {
    let overflow_nums: [u32; 5] = [1,2,3,4,u32::MAX];
    assert_eq!(sum(&overflow_nums), None);
   
    let nums: [u32; 5] = [1,2,3,4,5];
    assert_eq!(sum(&nums).unwrap(), 15);
}

fn sum(nums: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
   
    for num in nums {
        if sum.checked_add(*num).is_none() {
            return None;
        } else {
            sum += num
        }
    }
   
    Some(sum)
}
