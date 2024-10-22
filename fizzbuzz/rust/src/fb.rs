use std::convert::From;

pub type FBNum = u64;

#[derive(Clone, Debug)]
pub enum FB {
    Num(FBNum),
    Fizz,
    Buzz,
    FizzBuzz,
}

impl From<FBNum> for FB {
    fn from(n: FBNum) -> FB {
        match (n % 3, n % 5) {
            (0, 0) => FB::FizzBuzz,
            (0, _) => FB::Fizz,
            (_, 0) => FB::Buzz,
            _ => FB::Num(n),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FBStats {
    nums: usize,
    fizz: usize,
    buzz: usize,
    fizzbuzz: usize,
}

impl FBStats {
    pub fn new() -> Self {
        FBStats {
            nums: 0,
            fizz: 0,
            buzz: 0,
            fizzbuzz: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FBSet {
    pub nums: Vec<FBNum>,
    pub last: FBNum,
    pub stats: FBStats,
}

impl FBSet {
    pub fn new(count: FBNum) -> Self {
        let mut fbset = FBSet {
            last: 0,
            nums: Vec::new(),
            stats: FBStats::new(),
        };
        fbset.update(count);
        fbset
    }

    pub fn update(&mut self, count: FBNum) {
        for num in (self.last + 1)..=count {
            match FB::from(num) {
                FB::Fizz => self.stats.fizz += 1,
                FB::Buzz => self.stats.buzz += 1,
                FB::FizzBuzz => self.stats.fizzbuzz += 1,
                FB::Num(n) => {
                    self.stats.nums += 1;
                    self.nums.push(n);
                }
            }
        }
        self.last = count.max(self.last);
    }
}
