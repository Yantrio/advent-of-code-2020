fn main() {
    println!("Part 1: {}", solve(8335663, 8614349));
}

fn solve(pk1: usize, pk2: usize) -> usize {
    let mut loop_size = 0;
    let mut result = 1;

    while result != pk2 {
        loop_size += 1;
        result *= 7;
        result %= 20201227;
    }
    get_ek(pk1, loop_size)
}

fn get_ek(pk: usize, loops: usize) -> usize {
    let mut res = 1;
    for _ in 0..loops {
        res *= pk;
        res %= 20201227;
    }
    res
}
