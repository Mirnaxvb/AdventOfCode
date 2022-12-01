fn main() {
    let data = include_str!("../input.txt")
        .lines();
    let mut biggest = 0;
    let mut t: i32 = 0;

    for line in data {
        if line.is_empty() {
            if t >= biggest {
                biggest = t
            }
            t = 0;
            continue;
        }

       t += line.parse::<i32>().unwrap();
       println!("Total: {}", t);  
    }

    println!("Biggest: {}", biggest); 
}
