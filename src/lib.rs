use std::thread;
fn factors_rec(n: u64, i: u64, acc: Vec<u64>) -> Vec<u64> {
    if i > n {
        return acc;
    }
    if !is_prime(i) {
        println!("{i} is not prime");
        return factors_rec(n, i + 1, acc);
    }
    let (new_n, elements_to_push) = divide_until_not_divisible(n, i, vec![]);
    // let new_n = if gcd(n, i) != 1 { n / i } else { n };
    // let new_acc = functional_push_left(acc, i);
    let new_acc = acc
        .into_iter()
        .chain(elements_to_push.into_iter())
        .collect();
    factors_rec(new_n, i + 1, new_acc)
}

fn factors_start_rec(n: Box<u64>) -> Vec<u64> {
    factors_rec(*n, 2, vec![])
}

pub fn factors(n: u64) -> Vec<u64> {
    let n = Box::new(n);
    // we construct a thread to have larger stack size for our recursion :)
    const STACK_SIZE: usize = 4000 * 1024 * 1024;
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(|| factors_start_rec(n))
        .unwrap();
    let child_res = child.join().unwrap();
    child_res
}

fn functional_push_left<T>(acc: Vec<T>, x: T) -> Vec<T> {
    [x].into_iter().chain(acc.into_iter()).collect()
}

fn functional_push_right<T>(acc: Vec<T>, x: T) -> Vec<T> {
    acc.into_iter().chain([x].into_iter()).collect()
}

fn divide_until_not_divisible(n: u64, x: u64, acc: Vec<u64>) -> (u64, Vec<u64>) {
    if gcd(n, x) == 1 {
        return (n, acc);
    }
    let new_acc = functional_push_right(acc, x);
    divide_until_not_divisible(n / x, x, new_acc)
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn is_prime(n: u64) -> bool {
    for i in 2..n {
        let gcd_res = gcd(n, i);
        if gcd_res != 1 {
            return false;
        }
    }
    true
}
