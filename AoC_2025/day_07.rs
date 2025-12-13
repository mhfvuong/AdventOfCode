use std::fs;
use std::collections::{HashSet,HashMap};

fn part_01(teleporter_path: &Vec<&str>) -> usize {
    let mut beams: HashSet<usize> = HashSet::new();
    let mut splits = 0;
    for p in (0..teleporter_path.len()).step_by(2) {
        // println!("{}",teleporter_path[p]);
        if p == 0{
            let mut pos = 0;
            for i in teleporter_path[p].chars().collect::<Vec<_>>(){
                if i == 'S' {
                    beams.insert(pos);
                    break;
                }
                pos += 1;
            }
        } else {
            let mut split: HashSet<usize> = HashSet::new();
            for i in beams.drain(){
                if teleporter_path[p].chars().collect::<Vec<_>>()[i] == '^' {
                    split.insert(i-1);
                    split.insert(i+1);
                    splits += 1;
                } else {
                    split.insert(i);
                }
            }
            // println!("We now have beams:");
            for i in split.drain(){
                // println!("{}", i);
                beams.insert(i);
            }
        }

    }
    return splits;
}

fn part_02(teleporter_path: &Vec<&str>) -> usize {
    let mut beams: HashMap<usize, usize> = HashMap::new();
    for p in (0..teleporter_path.len()).step_by(2) {
        // println!("{}",teleporter_path[p]);
        if p == 0{
            let mut pos = 0;
            for i in teleporter_path[p].chars().collect::<Vec<_>>(){
                if i == 'S' {
                    beams.insert(pos, 1);
                    break;
                }
                pos += 1;
            }
        } else {
            let mut split: HashMap<usize, usize> = HashMap::new();
            for (k, v) in beams.drain(){
                if teleporter_path[p].chars().collect::<Vec<_>>()[k] == '^' {
                    match split.get(&(k+1)) {
                        Some(x) => split.insert(k+1, v+x),
                        None => split.insert(k+1, v),
                    };
                    match split.get(&(k-1)) {
                        Some(x) => split.insert(k-1, v+x),
                        None => split.insert(k-1, v),
                    };
                } else {
                    match split.get(&k) {
                        Some(x) => split.insert(k, v+x),
                        None => split.insert(k, v),
                    };
                }
            }
            for (k,v) in split.drain(){
                // println!("{}", i);
                beams.insert(k, v);
            }
        }

    }
    return beams.values().sum();
}

fn main(){
    let data = fs::read_to_string("day_07.txt").expect("Unable to read file");
    let lines = data.lines();
    let mut teleporter_path: Vec<&str> = Vec::new();
    for line in lines{
        teleporter_path.push(line.trim());
    }
    println!("Part 1: {}", part_01(&teleporter_path));
    println!("Part 2: {}", part_02(&teleporter_path));
}