fn main() {
    let mut sum = 0;
    let mut count = 0;
    let mut s = Seive::new();
    while let Some(n) = s.next() {
        if count >= 11 {
            break;
        }
        if (n > 10) {//}&& s.is_prime(n) {
            let t = Trunc::new(n);
            let mut p = true;
            for x in t {
                if ! s.is_prime(x) {
                    p = false;
                    break;
                }
            }
            if p {
                println!("All Prime! found a match {}\n",n);
                sum += n;
                count += 1;
                println!("Found {}/11 for a sum of {}",count,sum);
            } 
        }
    }
}

struct Trunc {
    digits: std::vec::Vec<u8>,
    cur:i32,
}

impl Trunc {
    fn num_digits(n: usize) -> usize {
        (format!("{}",n).split("").filter(|x| x.len() != 0).collect::<Vec<_>>()).len()
    }
    fn new(num:usize) -> Trunc {
        let digits :Vec<u8> = format!("{}",num)
            .split("")
            .filter(|x| x.len() != 0)
            .map(|n| n.parse().unwrap())
            .collect();
        let start = 1 - (digits.len() as i32);
        Trunc { digits:digits,cur:start }
    }
    fn to_num(mut digits: &[u8]) -> usize {
        let mut sum:usize = 0;
        for d in digits.iter().map(|x| *x) {
            sum *= 10;
            sum += d as usize;
        }
        sum
    }
}
impl Iterator for Trunc {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let mut res = None;
        if self.cur < 0 
            && ((-self.cur) as usize) < self.digits.len() {
            res = Some(Trunc::to_num(&self.digits[self.digits.len() - (-self.cur) as usize ..]))
        } else if self.cur == 0 {
            res= Some(Trunc::to_num(&self.digits))
        } else if self.cur < self.digits.len() as i32 {
            res = Some(Trunc::to_num(&self.digits[..self.cur as usize]))
        }
        self.cur +=1;
        res
    }
}

struct Seive {
    primes:std::vec::Vec<bool>,
    index:usize,
}
impl Seive {
    fn new() -> Seive {
        let index :usize= 2;
        let prs = vec![false,false,true];
        Seive { primes:prs, index:index }
    }
    fn is_prime(&mut self, num :usize) -> bool {
        let oldi = self.index;
        while num >= self.primes.len() {
            self.next().unwrap();
        }
        let res = self.primes[num];
        self.index = oldi;
        return res;
    }
}
impl Iterator for Seive {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.index < self.primes.len() && self.primes[self.index] {
            let res = self.index;
            self.index +=1;
            return Some(res);
        } else {
            while self.index < self.primes.len() && ! self.primes[self.index] {
                self.index+= 1; //find next one
            }
            if self.index < self.primes.len() {
                //oh we found another
                let res = self.index;
                self.index += 1;
                return Some(res);
            }

            loop {
            //reached end of list, need to calc another
                self.index = self.primes.len();
                for i in 2..self.primes.len()  {
                    if self.index % i == 0 {
                        self.primes.push(false);
                        break;
                    }
                }
                if self.primes.len() == self.index {
                    //diddnt add a false, must be true!
                    self.primes.push(true);
                    return self.next();
                } else {
                    //we added a false in the loop. just keep trying.
                }
            }
        }
    } 
}
