
struct TreeAncestor {
    tree: Vec<Vec<i32>>,
}

impl TreeAncestor {

    fn new(n: i32, parent: Vec<i32>) -> Self {
        let n = n as usize;
        let mut tree = vec![vec![]; n];
        for i in 0..n {
            tree[i].push(parent[i]);
        }
        let mut allminusone = false;
        let mut j = 1;
        while !allminusone {
            allminusone = true;
            for i in 0..n {
                let prev = tree[i][j - 1];
                if prev != -1 {
                    allminusone = false;
                    let tmp = tree[prev as usize][j - 1];
                    tree[i].push(tmp);
                } else {
                    tree[i].push(-1);
                }
            }
            j += 1;
        }

        Self { tree: tree }
    }
    
    fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let ffs = |x: i32| { 
            let mut bn = x & -x;
            let mut count = 0;
            while bn != 0 {
                count += 1;
                bn = bn >> 1;
            }
            count - 1
        };
        if node == -1 || k == 0 {
            node
        } else {
            let node = node as usize;
            let pos = ffs(k) as usize;
            // println!("{} {}", k, pos);
            if pos < self.tree[node].len() {
                self.get_kth_ancestor(self.tree[node][pos], k - (1 << pos))
            } else {
                -1
            }
        }
    }
}

#[test]
fn tree_test() {
    let tree = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
    println!("{}", tree.get_kth_ancestor(3, 1));
    println!("{}", tree.get_kth_ancestor(5, 2));
    println!("{}", tree.get_kth_ancestor(6, 3));
}