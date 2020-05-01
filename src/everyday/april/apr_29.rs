// This problem does not provide a rust grading system
// I post the cpp code below


// This is the MountainArray's API interface.
// You should not implement it, or speculate about its implementation
// class MountainArray {
// public:
//     int get(int index);
//     int length();
// };

// class Solution {
// public:
//     int findPeak(MountainArray &mountainArr, int start, int end) {
//         if (end - start < 3) { 
//             int max = -1; int index = -1;
//             for (int i = start; i <= end; i++) { 
//                 int value = mountainArr.get(i);
//                 if (value > max) {max = value; index = i;}
//             }
//             return index;
//         } else {
//             int mid = (start + end) / 2;
//             int t1 = (start + mid) / 2; int t2 = (end + mid) / 2;
//             int vmid = mountainArr.get(mid);
//             int v1 = mountainArr.get(t1); int v2 = mountainArr.get(t2);
//             if (v1 > vmid && v1 > v2) {
//                 return findPeak(mountainArr, start, mid);
//             } else if (vmid >= v1 && vmid >= v2) {
//                 return findPeak(mountainArr, t1, t2);
//             } else {
//                 return findPeak(mountainArr, mid, end);
//             }
//         }
//     }
    
//     int bsearch(MountainArray &mountainArr, int start, int end, bool increase, int target) {
//         int vs = mountainArr.get(start);
//         int ve = mountainArr.get(end);
//         while (start + 1 < end) {
//             int mid = (start + end) / 2;
//             int vmid = mountainArr.get(mid);
        
//             if (increase) {
//                 if (vmid > target) { end = mid; }
//                 else { start = mid; }
//             } else {
//                 if (vmid >= target) { start = mid; }
//                 else { end = mid; }
//             }
//         }
//         if (mountainArr.get(start) == target) { return start; }
//         else if (mountainArr.get(end) == target) { return end; }
//         else { return -1; }
//     }
    
//     int findInMountainArray(int target, MountainArray &mountainArr) {
//         int n = mountainArr.length();
//         int peak = findPeak(mountainArr, 0, n - 1);
//         int position = bsearch(mountainArr, 0, peak, true, target);
//         if (position != -1) return position;
//         return bsearch(mountainArr, peak, n - 1, false, target);
//     }
// };

struct MountainArray {
    array: Vec<i32>,
}

impl MountainArray {
    fn new() -> Self {
        Self { array: Vec::new(), }
    }
    fn length(&self) -> usize {
        self.array.len()
    }
    fn get(&self, index: usize) -> i32 {
        self.array[index]
    }
}

fn find_peak(mountain: &MountainArray, start: usize, end: usize) -> usize {
    if end < start + 3 {
        (start..=end).map(|i| (mountain.get(i), i)).max_by(
            |a, b| a.0.cmp(&b.0)
        ).unwrap().1
    } else {
        let mut indices = vec![0; 3];
        indices[1] = (start + end) / 2; indices[0] = (start + indices[1]) / 2; indices[2] = (indices[1] + end) / 2;
        let values = indices.iter().map(|&i| mountain.get(i)).collect::<Vec<_>>();
        if values[0] > values[1] && values[0] > values[2] {
            find_peak(mountain, start, indices[1])
        } else if values[1] >= values[0] && values[1] >= values[2] {
            find_peak(mountain, indices[0], indices[2])
        } else {
            find_peak(mountain, indices[1], end)
        }
    }
}

fn bsearch(mountain: &MountainArray, start: usize, end: usize, increase: bool, target: i32) -> i32 {
    let vs = mountain.get(start);
    let ve = mountain.get(end);
    let mut lo = start;
    let mut hi = end;
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        let vm = mountain.get(mid);

        if increase {
            if vm > target { hi = mid; }
            else { lo = mid; }
        } else {
            if vm >= target { lo = mid; }
            else { hi = mid; }
        }
    }
    if mountain.get(lo) == target { lo as i32 }
    else if mountain.get(hi) == target { hi as i32 }
    else { -1 }
}

fn find_in_mountain_array(target: i32, mountain: &MountainArray) -> i32 {
    let n = mountain.length();
    let peak = find_peak(mountain, 0, n - 1);
    let p_increase = bsearch(mountain, 0, peak, true, target);
    if p_increase != -1 { p_increase } else {
        bsearch(mountain, peak, n - 1, false, target)
    }
}