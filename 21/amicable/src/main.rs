fn main() {
    let mut sum = 0;
    for n in 1..10001 {
        if is_amicable(n) {
            println!("{} is amicable", n);
            println!("  Has factors {:?}", divisors(n));
            let m = d(n);
            println!("  With sum {}", m);
            println!("      Has factors {:?}", divisors(m));
            println!("      With sum {}", d(m));
            sum += n;
        }
    }
    println!("{}", sum);
}

fn is_amicable(n : u32) -> bool {
    let m = d(n);
    if n != m {
        n == d(m)
    } else {
        false
    }
}

fn d(n : u32) -> u32 {
    let mut sum = 1;
    for d in 2..(n/2+1) {
        if n % d == 0 {
            sum += d;
        }
    }
    sum
}

fn divisors(n : u32) -> Vec<u32> {
    let mut res : Vec<u32> = vec![];
    for d in 2..(n/2+1) {
        if n % d == 0 {
            res.push(d);
        }
    }
    res
}
