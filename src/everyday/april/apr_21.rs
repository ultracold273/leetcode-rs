

fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut nodds = vec![0; n + 1];
    for (i, &num) in nums.iter().enumerate() {
        nodds[i + 1] = nodds[i] + if num % 2 == 1 { 1 } else { 0 };
    }

    let mut cnt = 0;
    let (mut ast, mut aed) = (0, 0);
    let (mut bst, mut bed) = (0, 0);
    while ast < n + 1 {
        while aed < n + 1 && nodds[ast] == nodds[aed] {
            aed += 1;
        }
        if aed == n + 1 { break; }
        while bst < n + 1 && nodds[bst] != nodds[ast] + k {
            bst += 1;
        }
        if bst == n + 1 { break; }
        bed = bst;
        while bed < n + 1 && nodds[bst] == nodds[bed] {
            bed += 1;
        }
        cnt += (aed - ast) * (bed - bst);
        ast = aed;
    }
    cnt as i32
}
