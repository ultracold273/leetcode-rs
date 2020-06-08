
fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut arr = arr.clone();
    arr.sort();
    let mut res = Vec::new();
    let m = arr[(arr.len()-1)/2];
    let mut st = 0;
    let mut ed = arr.len() - 1;
    for _ in 0..k {
        if st == ed { 
            res.push(arr[st]); 
            break;
        } else if m - arr[st] > arr[ed] - m {
            res.push(arr[st]);
            st += 1;
        } else {
            res.push(arr[ed]);
            ed -= 1;
        }
    }
    res
}

#[test]
fn get_strongest_test() {
    let v = vec![1,2,3,4,5];
    println!("{:?}", get_strongest(v, 2));
}