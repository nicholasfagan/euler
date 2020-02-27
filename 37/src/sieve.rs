

struct Seive {
    primes:std::vec::Vec<bool>,
    index:usize,
}
impl Seive {
    fn new(initbound:usize) -> Seive {
        let index :usize= 1;
        let prs = vec![false,false,true];
        Seive { primes:prs, index:index }
    }
}
impl Iterator for Seive {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.index += 1;
            if self.index < self.primes.length
                || self.primes[self.index] {
                return Some(self.index);
            } else {
                self.primes.push(true);
                for i in 2..self.index {
                    if self.index % i == 0 {
                        self.primes[self.index] = false;
                        break;
                    }
                }
            }
        }
    } 
}
