use std::fs;

fn part_1(inventory_low: &Vec<usize>, inventory_high: &Vec<usize>, numbers: &Vec<usize>) -> usize {
    let mut fresh = 0;
    for n in numbers{
        let mut in_range = false;
        for idx in 0..inventory_low.len() {
            if inventory_low[idx] > *n || in_range{
                continue;
            }
            if *n >= inventory_low[idx] && *n <= inventory_high[idx] {
                fresh += 1;
                // println!("inventory range {}-{}\n item: {}", inventory_low[idx], inventory_high[idx], n);
                in_range = true;
            }
        }
    }
    return fresh;
}

fn part_2(inventory_low: &Vec<usize>, inventory_high: &Vec<usize>) -> usize {
    let mut last_id = 0;
    let mut ids = 0;
    for idx in 0..inventory_low.len() {
        // println!("checking range {}-{}", inventory_low[idx], inventory_high[idx]);
        // ids += inventory_high[idx] - inventory_low[idx] + 1;
        if last_id > inventory_low[idx]{
            // let diff = last_id - inventory_low[idx] + 1;
            // println!("overlapping with the last range: {}", last_id);
            // ids -= diff;
            if inventory_high[idx] > last_id{
                // println!("adding: {}", inventory_high[idx] - last_id + 1);
                ids += inventory_high[idx] - last_id + 1; // count range
            }
        } else {
            // println!("adding: {}", inventory_high[idx] - inventory_low[idx] + 1);
            ids += inventory_high[idx] - inventory_low[idx] + 1; // count range
        }
        if inventory_high[idx] > last_id {
            last_id = inventory_high[idx]; // remember highest value of range
        }
    }
    return ids;
}

fn main(){
    let data = fs::read_to_string("day_05.txt").expect("Unable to read file");
    // println!("File contents:\n{}", data);
    let lines = data.lines();
    let mut numbers: Vec<usize> = Vec::new();
    let mut inventory_low: Vec<usize> = Vec::new();
    let mut inventory_high: Vec<usize> = Vec::new();
    let mut get_ranges = true;
    for line in lines {
        if line == ""{
            get_ranges = false;
            continue;
        }
        if get_ranges {
            let ranges: Vec<&str> = line.trim().split("-").collect();
            inventory_low.push(ranges[0].parse::<usize>().unwrap());
            inventory_high.push(ranges[1].parse::<usize>().unwrap());
        } else {
            numbers.push(line.trim().parse::<usize>().unwrap());
        }
    }
    let mut pairs: Vec<(usize, usize)> = inventory_low
    .into_iter()
    .zip(inventory_high.into_iter())
    .collect();

    pairs.sort_by_key(|(low, _)| *low);

    let (inventory_low, inventory_high): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();

    println!("Part 1: {}", part_1(&inventory_low, &inventory_high, &numbers));
    println!("Part 2: \n{}", part_2(&inventory_low, &inventory_high));
    println!("42792414823769 is too low and\n319927450805369 is too low\n338693411431517 is too high");
}