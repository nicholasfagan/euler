use std::env;

//calculates the Mth permutation of [1..N]
fn main() {
    let args :Vec<_>= env::args().collect();
    if args.len() != 3 {
        panic!("usage: {} <n> <m>",args[0]);
    } else {
        let n :u32= args[1].parse().unwrap();
        let m :u32= args[2].parse().unwrap();
        let lst :Vec<_>=(1..(n+1)).collect(); 
        println!("{:?}",permute(lst,m as u64));
    }
}



fn permute(lst :Vec<u32>, m :u64) -> Vec<u32> {
    if m >= factorial(lst.len() as u64) {
        panic!("Invalid permutation");
    } else {
        permute_helper(lst,Vec::new(),m)
    }
} 
fn permute_helper(mut lst :Vec<u32>,mut acc :Vec<u32>, m :u64) -> Vec<u32> {
    if lst.len() == 0 {
        acc
    } else if m == 0 {
        let mut lst = lst;
        let mut acc = acc;
        acc.push(lst.remove(0));
        permute_helper(lst,acc,0)
    } else {
        let f = factorial((lst.len()-1) as u64); 
        let el = lst[(m/f) as usize];
        lst.remove((m/f) as usize);
        acc.push(el);
        permute_helper(lst,acc,m%f) 
    }

} 


fn factorial(n : u64) -> u64 {
    let mut a = 1;
    let mut n = n;
    while n != 0 {
        a *= n;
        n-=1;
    }
    a
}
