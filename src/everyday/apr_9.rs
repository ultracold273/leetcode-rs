
fn gp_dfs(lp: i32, rp: i32, stack: &mut Vec<char>, res: &mut Vec<String>) {
    if lp == 0 && rp == 0 {
        res.push(stack.iter().collect());
        return ;
    }
    if lp > 0 {
        stack.push('(');
        gp_dfs(lp-1, rp, stack, res);
        stack.pop();
    }
    if rp > 0 && rp > lp {
        stack.push(')');
        gp_dfs(lp, rp-1, stack, res);
        stack.pop();
    }
}

fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = Vec::new();
    gp_dfs(n, n, &mut Vec::new(), &mut res);
    res
}