pub fn solve() -> (usize, usize) {
    let final_sub = include_str!("../resources/day02.txt")
        .split("\n")
        .filter(|l| !l.is_empty())
        .fold(Sub::default(), move_sub);
    let result1 = final_sub.x * final_sub.depth;

    let final_sub = include_str!("../resources/day02.txt")
        .split("\n")
        .filter(|l| !l.is_empty())
        .fold(Sub::default(), move_sub2);
    let result2 = final_sub.x * final_sub.depth;

    (result1, result2)
}

fn move_sub(mut sub: Sub, raw_line: &str) -> Sub {
    let (cmd, n) = parse_line(raw_line);
    match cmd {
        "forward" => sub.x += n,
        "down" => sub.depth += n,
        "up" => sub.depth -= n,
        _ => unreachable!(),
    }
    sub
}

fn move_sub2(mut sub: Sub, raw_line: &str) -> Sub {
    let (cmd, n) = parse_line(raw_line);
    match cmd {
        "down" => sub.aim += n,
        "up" => sub.aim -= n,
        "forward" => {
            sub.x += n;
            sub.depth += sub.aim * n;
        }
        _ => unreachable!(),
    }
    sub
}

fn parse_line(raw_line: &str) -> (&str, usize) {
    let mut words = raw_line.split(" ");
    let cmd = words.next().unwrap();
    let n = usize::from_str_radix(words.next().unwrap(), 10).unwrap();
    (cmd, n)
}

#[derive(Default, Debug)]
struct Sub {
    x: usize,
    depth: usize,
    aim: usize,
}
