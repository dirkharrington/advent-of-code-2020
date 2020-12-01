fn main() {
    let items: Vec<usize> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|i| i.parse().unwrap())
        .collect();

    for a in &items {
        for b in &items {
            for c in &items {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                    return;
                }
            }
        }
    }
}
