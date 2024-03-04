use std::io;

fn solution(input: &Vec<(u16, u16)>, population: u16) -> u16 {
    if input.len() == 1 {
        trivial_solution_2(input[0].0, input[0].1, population)
    } else if input.len() == 2 {
        trivial_solution_4(input[0].0, input[0].1, input[1].0, input[1].1, population)
    } else {
        solution_by_wall(0, input, population)
            .min(solution_by_wall(1, input, population))
            .min(solution_by_wall(2, input, population))
            .min(solution_by_wall(3, input, population))
    }
}

fn trivial_solution_2(n00: u16, n01: u16, population: u16) -> u16 {
    if n00 + n01 <= population {
        1
    } else {
        2
    }
}

fn trivial_solution_3(n00: u16, n01: u16, n10: u16, population: u16) -> u16 {
    if n00 + n01 <= population || n00 + n10 <= population {
        2
    } else {
        3
    }
}

fn trivial_solution_4(n00: u16, n01: u16, n10: u16, n11: u16, population: u16) -> u16 {
    if n00 + n01 <= population && n10 + n11 <= population
        || n00 + n10 <= population && n01 + n11 <= population
    {
        2
    } else if n00 + n01 <= population
        || n10 + n11 <= population
        || n00 + n10 <= population
        || n01 + n11 <= population
    {
        3
    } else {
        4
    }
}

fn solution_by_wall(wall: u8, input: &Vec<(u16, u16)>, population: u16) -> u16 {
    let mut status0;
    let mut status1;
    let mut status2;
    let mut status3;

    match wall {
        0 => {
            status0 = trivial_solution_2(input[0].0, input[0].1, population);
            status1 = 1;
            status2 = 1;
            status3 = 0
        }
        1 => {
            status0 =
                trivial_solution_3(input[0].0, input[0].1, input.last().unwrap().0, population);
            status1 = 2;
            status2 = trivial_solution_2(input.last().unwrap().0, input[0].0, population);
            status3 = 1
        }
        2 => {
            status0 =
                trivial_solution_3(input[0].1, input[0].0, input.last().unwrap().1, population);
            status1 = trivial_solution_2(input.last().unwrap().1, input[0].1, population);
            status2 = 2;
            status3 = 1
        }
        3 => {
            status0 = trivial_solution_4(
                input[0].0,
                input[0].1,
                input.last().unwrap().0,
                input.last().unwrap().1,
                population,
            );
            status1 = trivial_solution_3(
                input.last().unwrap().1,
                input.last().unwrap().0,
                input[0].1,
                population,
            );
            status2 = trivial_solution_3(
                input.last().unwrap().0,
                input.last().unwrap().1,
                input[0].0,
                population,
            );
            status3 =
                trivial_solution_2(input.last().unwrap().0, input.last().unwrap().1, population)
        }
        _ => panic!(),
    }

    for i in 1..input.len() {
        let n00 = status0 + trivial_solution_2(input[i].0, input[i].1, population);
        let n01 = status1 + trivial_solution_3(input[i].0, input[i].1, input[i - 1].0, population);
        let n02 = status2 + trivial_solution_3(input[i].1, input[i].0, input[i - 1].1, population);
        let n03 = status3
            + trivial_solution_4(
                input[i].0,
                input[i].1,
                input[i - 1].0,
                input[i - 1].1,
                population,
            );

        let n10 = status0 + 1;
        let n11 = status1 + 2;
        let n12 = status2 + trivial_solution_2(input[i - 1].1, input[i].1, population);
        let n13 =
            status3 + trivial_solution_3(input[i - 1].1, input[i - 1].0, input[i].1, population);

        let n20 = status0 + 1;
        let n21 = status1 + trivial_solution_2(input[i - 1].0, input[i].0, population);
        let n22 = status2 + 2;
        let n23 =
            status3 + trivial_solution_3(input[i - 1].0, input[i - 1].1, input[i].0, population);

        let n30 = status0;
        let n31 = status1 + 1;
        let n32 = status2 + 1;
        let n33 = status3 + trivial_solution_2(input[i - 1].0, input[i - 1].1, population);

        status0 = n00.min(n01).min(n02).min(n03);
        status1 = n10.min(n11).min(n12).min(n13);
        status2 = n20.min(n21).min(n22).min(n23);
        status3 = n30.min(n31).min(n32).min(n33);
    }

    match wall {
        0 => status0,
        1 => status1,
        2 => status2,
        3 => status3,
        _ => panic!(),
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let test_case_count = line.trim().parse::<usize>().unwrap();
    for _ in 0..test_case_count {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.trim().split(' ').flat_map(&str::parse::<u16>);
        iter.next();
        let population = iter.next().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line1 = line.trim().split(' ').flat_map(&str::parse::<u16>);
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let input = line
            .trim()
            .split(' ')
            .flat_map(&str::parse::<u16>)
            .zip(line1)
            .collect::<Vec<_>>();

        println!("{}", solution(&input, population));
    }
}
