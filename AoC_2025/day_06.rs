use std::fs;

fn part_01(problems: &Vec<&str>) -> usize {
    let problem_amount = problems[0].split_whitespace().collect::<Vec<_>>().len();
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

fn part_02(problems: &Vec<&str>) -> usize {
    let mut addition_multiplication: Vec<&str> = problems[problems.len() - 1].split_whitespace().collect();
    let problem_amount = problems[0].split_whitespace().collect::<Vec<_>>().len();
    let mut answers = vec![0; problem_amount];
    let mut answers_idx = 0;
    addition_multiplication.reverse();
    for char in (0..problems[0].chars().collect::<Vec<_>>().len()).rev(){
        let mut white_spaces = true;
        let mut num_text = String::from("");

        for p in 0..problems.len() - 1 {
            let line: Vec<_> = problems[p].chars().collect();
            if line[char] != ' '{
                white_spaces = false;
                num_text.push_str(&line[char].to_string());
            }
        }
        if white_spaces == true{
            answers_idx += 1;
        } else {
            let number = num_text.parse::<usize>().unwrap();
            if answers[answers_idx] == 0{
                answers[answers_idx] = number;
            } else {
                if addition_multiplication[answers_idx] == "+"{
                    answers[answers_idx] += number;
                } else if addition_multiplication[answers_idx] == "*" {
                    answers[answers_idx] *= number;
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
        problems.push(line);
    }
    println!("Part 1: {}", part_01(&problems));
    println!("Part 2: {}", part_02(&problems));
}