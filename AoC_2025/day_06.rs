use std::fs;
// use std::str;

fn part_01(problems: &Vec<&str>) -> usize {
    let problem_amount = problems[0].split_whitespace().collect::<Vec<_>>().len();
    // println!("{:?}", problems[0].split_whitespace().collect::<Vec<_>>());
    let mut answers = vec![0; problem_amount];
    let addition_multiplication: Vec<&str> = problems[problems.len() - 1].split_whitespace().collect();
    for p in 0..problems.len() - 1{
        for num in 0..problem_amount{
            if p == 0 {
                answers[num] = problems[0].split_whitespace().collect::<Vec<_>>()[num].parse::<usize>().unwrap();
            } else {
                if addition_multiplication[num] == "+"{
                    answers[num] += problems[p].split_whitespace().collect::<Vec<_>>()[num].parse::<usize>().unwrap();
                } else if addition_multiplication[num] == "*" {
                    answers[num] *= problems[p].split_whitespace().collect::<Vec<_>>()[num].parse::<usize>().unwrap();
                }
            }
        }
    }
    return answers.iter().sum::<usize>();
}

fn main(){
    let data = fs::read_to_string("day_06.txt").expect("Unable to read");
    let lines = data.lines();
    let mut problems: Vec<&str> = Vec::new();
    for line in lines {
        problems.push(line.trim());
    }
    // println!("{:?}", problems);
    println!("Part 1: {}", part_01(&problems));
}