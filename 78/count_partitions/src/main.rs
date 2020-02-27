fn main() {
    println!("{:?}", pmod(1000, 1000, 1000000));
    println!("{:?}", pmod2(1000, 1000, 1000000));
}

/// partition n elemnts into k subsets, return first one that is div by m
fn pmod(n:usize, k:usize, m:usize) -> Option<(usize,usize)> {
    let mut prev : Vec<usize> = vec![0 ; k + 1];
    let mut next : Vec<usize> = vec![0 ; k + 1];
    for i in 1..(n+1) {
        for j in 1..(i+1) {
            if j == 1 || i == j {
                next[j] = 1;
            } else {
                next[j] = (  ((j * prev[j])%m)  + prev[j-1] )%m;
                if next[j] == 0 {
                    return Some((i,j));
                }
            }
        }
        let tmp = prev;
        prev = next;
        next = tmp;
    }
    None
}

fn pmod2(n:usize, k:usize, m:usize) -> Option<(usize,usize)> {
    // Table to store results of subproblems
    let mut dp : Vec<Vec<usize>> = vec![vec![0;k+1];n+1];

    // Fill rest of the entries in dp[][]
    // in bottom up manner
    for i in 1..(n+1) {
        for j in 1..(i+1) {
            if j == 1 || i == j {
                dp[i][j] = 1;
            } else {
                dp[i][j] = ((j * dp[i - 1][j])%m  + dp[i - 1][j - 1])%m;
                if dp[i][j] == 0 {
                    return Some((i,j));
                }
            }
        }
    }
    None
}
