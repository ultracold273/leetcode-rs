
fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut mins_in_row = vec![(0, 1000001); m];
    let mut maxs_in_col = vec![(0, 0); n];
    let mut res = Vec::new();
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] < mins_in_row[i].1 {
                mins_in_row[i] = (j, matrix[i][j]);
            }
            if matrix[i][j] > maxs_in_col[j].1 {
                maxs_in_col[j] = (i, matrix[i][j]);
            }
        }
    }

    // println!("{:?}", mins_in_row);
    // println!("{:?}", maxs_in_col);

    for i in 0..m {
        if maxs_in_col[mins_in_row[i].0].0 == i {
            res.push(mins_in_row[i].1)
        }
    }
    res
}

#[test]
fn lucky_numbers_tests() {
    let test_cases = vec![
        (vec![vec![3,7,8],vec![9,11,13],vec![15,16,17]], vec![15]),
        (vec![vec![1,10,4,2],vec![9,3,8,7],vec![15,16,17,12]], vec![12]),
        (vec![vec![7,8],vec![1,2]], vec![7]),
    ];
    for case in test_cases {
        assert_eq!(lucky_numbers(case.0), case.1);
    }
}