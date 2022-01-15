fn main() {
    loop {
        let scan = std::io::stdin();
        let mut line = String::new();
        let _ = scan.read_line(&mut line);
        let vec: Vec<&str> = line.split_whitespace().collect();

        let n: usize = vec[0].parse().unwrap();
        let x: usize = vec[1].parse().unwrap();
        if n == 0 && x == 0 {
            break;
        }

        let mut count = 0;
        for i in 1..n - 1 {
            for j in i + 1..n {
                for k in j + 1..n + 1 {
                    if i + j + k == x {
                        count += 1;
                    }
                }
            }
        }

        println!("{}", count);
    }
}
