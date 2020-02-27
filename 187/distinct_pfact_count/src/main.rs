use std::io::Write;
use std::env::args;

fn main() {
    let mut argv = args();
    argv.next().unwrap();
    let max = argv.next().unwrap().parse::<usize>().unwrap();
    let mut count = 0;
    let ps = prime_seive(max);
    println!("Counting . . . ");
    for l in 0..(ps.len()) {
        for h in l..(ps.len()) {
            if ps[l] * ps[h] < max {
                count += 1;
            } else { break; }
        }
    }
    println!("There are {} Semiprimes below {}", count, max);
}

fn prime_seive(n :usize) -> Vec<usize> {
    print!("Allocating seive . . . ");
    std::io::stdout().flush().unwrap();
    let mut seive : Vec<_>= vec![true; n];
    println!("done.");
    progress("Sifting . . .", 0f32, 1f32);
    for i in 2..n {
        if seive[i] == true {
            for j in ((i*i)..n).step_by(i) {
                seive[j] = false;
            }
        }
        if 20000 * i / n % 100 == 0 {
            progress("Sifting . . .", i as f32, n as f32);
        }
    }
    progress("Sifting . . .", 1f32, 1f32);
    println!("");


    print!("Copying to vec . . . ");
    std::io::stdout().flush().unwrap();
    let mut res :Vec<usize> = std::vec::Vec::new();
    for i in 2..n {
        if seive[i] == true {
            res.push(i);
        }
    }
    println!("done.");
    res
}

fn progress<T : Into<f32>>(msg : &str, curr : T, total : T) {
    print!("\r{} {:2.2}%", msg, 100f32 * curr.into() as f32 / total.into() as f32);
    std::io::stdout().flush().unwrap();
}
