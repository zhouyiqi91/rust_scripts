fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m; m = n; n = t;
        }
        m = m % n;
    }
    n
}

struct StrVecIter {
    v: Vec<&'static str>,
    i: usize
}

impl Iterator for StrVecIter {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item>{
        if self.i >= self.v.len() {
            return None
        } 
        self.i += 1;
        return Some(self.v[self.i - 1]);
    }
}

fn triangle(n: i32) -> i32 {
    (0..n+1).fold(0, |sum, i| sum + i)
}

fn main() {
    let (m, n): (u64, u64) = (30, 20);
    let number = gcd(m, n);
    println!("m: {} n: {} number:{}", m,n, number);

    let myvec = StrVecIter{v: vec!["I", "am", "tony"], i:0};
    for s in myvec {
        println!("{}", s);
    }

    let s = triangle(5);
    println!("{}", s);

    let thread1 = std::thread::spawn(|| {
        println!("Alphonse");
        return 137;
    });
    let thread2 = std::thread::spawn(|| {
        println!("Gaston");
        return 139;
    });
    assert_eq!((thread1.join().unwrap()), 137);
    assert_eq!((thread2.join().unwrap()), 139);


}
