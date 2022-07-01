const UPPER_BOUND: u32 = 100000000;

fn main() {
    let mut primes: Vec<u32> = vec![2];

    for p in (3..UPPER_BOUND).step_by(2) {
        for q in &primes {
            if q * q > p {
                primes.push(p);
                break;
            }

            if p % q == 0 {
                break;
            }
        }
    }

    println!(
        "Found {count} primes below {UPPER_BOUND}",
        count = primes.len()
    );
}
