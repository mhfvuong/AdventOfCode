use std::fs;

fn day_02(numbers: &Vec<&str>) -> i64 {
    let mut sum = 0;
    let mut iter = 0;
    while iter < numbers.len() {
        let first: i64 = numbers[iter].parse().unwrap();
        let second: i64 = numbers[iter + 1].parse().unwrap();
        let mut pointer: i64 = first;
        // let mut pointer_as_char: &str = String::new().as_str();
        
        while pointer <= second {
            // println!("Pointer: {}", pointer);
            let pointer_as_char = pointer.to_string();
            let mut char = String::new();
            if pointer_as_char.len() % 2 != 0 {
                pointer += 1;
                continue;
            }
            for c in pointer_as_char.chars() {
                char.push(c);
                if char.len() > pointer_as_char.len() / 2 {
                    break;
                }
                let pat = char.clone();
                
                if pointer_as_char.matches(pat.as_str()).count()*char.len() == pointer_as_char.len() && pointer_as_char.matches(pat.as_str()).count() == 2{
                    // println!("Found repeating pattern: {}", pointer);
                    sum += pointer;
                    break;
                }
                
            }
            pointer += 1;
            
        }
        iter += 2;
    }
    // Placeholder for part 1 logic
    return sum;
}

fn day_02_2(numbers: &Vec<&str>) -> i64 {
    let mut sum = 0;
    let mut iter = 0;
    while iter < numbers.len() {
        let first: i64 = numbers[iter].parse().unwrap();
        let second: i64 = numbers[iter + 1].parse().unwrap();
        let mut pointer: i64 = first;
        // let mut pointer_as_char: &str = String::new().as_str();
        
        while pointer <= second {
            // println!("Pointer: {}", pointer);
            let pointer_as_char = pointer.to_string();
            let mut char = String::new();
            // if pointer_as_char.len() % 2 != 0 {
            //     pointer += 1;
            //     continue;
            // }
            for c in pointer_as_char.chars() {
                char.push(c);
                if char.len() > pointer_as_char.len() / 2 {
                    break;
                }
                let pat = char.clone();
                
                if pointer_as_char.matches(pat.as_str()).count()*char.len() == pointer_as_char.len(){// && pointer_as_char.matches(pat.as_str()).count() == 2{
                    // println!("Found repeating pattern: {}", pointer);
                    sum += pointer;
                    break;
                }
                
            }
            pointer += 1;
            
        }
        iter += 2;
    }
    // Placeholder for part 1 logic
    return sum;
}

fn main() {
    // println!("Hello, world!");
    let data = fs::read_to_string("day_02.txt").expect("Unable to read file");
    // println!("File contents:\n{}", data);

    let collections: Vec<&str> = data.trim().split(",").collect();
    let mut numbers: Vec<&str> = Vec::new();
    for c in collections{
        // println!("Collection item: {}", c);
        numbers.append(&mut c.split("-").collect());
    }
    // println!("Collections: {:?}", collections);
    
    // println!("Numbers: {:?}", numbers);
    println!("Part 1 Result: {}", day_02(&numbers));
    println!("Part 2 Result: {}", day_02_2(&numbers));
}
