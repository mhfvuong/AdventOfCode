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
    let mut ids = 0;
    let mut inventory_ranges: Vec<Vec<usize>> = vec![vec![inventory_low[0],inventory_high[0]]];
    let mut ranges = 0;
    for idx in 1..inventory_low.len() {
        if inventory_low[idx] <= inventory_ranges[ranges][1]{
            if inventory_high[idx] > inventory_ranges[ranges][1]{
                inventory_ranges[ranges][1] = inventory_high[idx];
            }
        } else if inventory_low[idx] + 1 == inventory_ranges[ranges][1] {
            inventory_ranges[ranges][1] = inventory_high[idx];
        } else {
            inventory_ranges.push(vec![inventory_low[idx],inventory_high[idx]]);
            ranges += 1;
        }
    }
    for range in inventory_ranges{
        ids += range[1] - range[0] + 1;
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
    println!("Part 2: {}", part_2(&inventory_low, &inventory_high));
}