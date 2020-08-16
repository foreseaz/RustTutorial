/*fib 0 = 1
fib 1 = 1
fib n = fib (n-1) + fib (n-2)
*/

fn fibo(n: i64) -> i64 {
    if 0 == n {
        return 1;
    } else if 1 == n {
        return 1;
    } else {
        let ret = fibo(n - 1) + fibo(n - 2);
        return ret;
    }
}
fn main() {
    println!("Hello, world!");
    let start= std::time::Instant::now();
    let sum= fibo(200);
    println!("{}", sum);
    println!("{:?}", start.elapsed());
}
