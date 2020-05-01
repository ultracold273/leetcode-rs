
fn merge_sort_with_count(nums: &mut Vec<(i32, i32)>, start: usize, end: usize) {
    let mut temp = Vec::new();
    if start + 1 < end { 
        let mid = (start + end) / 2;
        merge_sort_with_count(nums, start, mid);    
        merge_sort_with_count(nums, mid, end);
        let mut pn1 = start;
        let mut pn2 = mid;
        while pn1 < mid && pn2 < end {
            if nums[pn1].0 <= nums[pn2].0 {
                temp.push(nums[pn1]);
                pn1 += 1;
            } else {
                nums[pn2].1 += (mid - pn1) as i32;
                temp.push(nums[pn2]);
                pn2 += 1;
            }
        }
        if pn1 == mid { (pn2..end).for_each(|i| temp.push(nums[i])) }
        if pn2 == end { (pn1..mid).for_each(|i| temp.push(nums[i])) }
        (start..end).for_each(|i| nums[i] = temp[i-start])
    }
}

fn reverse_pairs(nums: Vec<i32>) -> i32 {
    let mut array_with_count = nums.iter().map(|&x| (x, 0i32)).collect::<Vec<_>>();
    merge_sort_with_count(&mut array_with_count, 0, nums.len());
    array_with_count.iter().map(|a| a.1).sum()
}

#[test]
fn reverse_pairs_test() {
    assert_eq!(reverse_pairs(vec![7, 5, 6, 4]), 5);
}