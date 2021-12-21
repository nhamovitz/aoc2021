#[derive(Default)]
struct Position {
    horiz: u32,
    depth: u32,
    aim: u32,
}

impl Position {
    fn forward(&mut self, dist: u32) {
        self.horiz += dist;
        self.depth += self.aim * dist; // Comment out for P1 behavior
    }

    fn up(&mut self, dist: u32) {
        self.aim -= dist;

        // For P1, replace with:
        // self.depth = self.depth - dist;
    }

    fn down(&mut self, dist: u32) {
        self.aim += dist;

        // For P1, replace with:
        // self.depth = self.depth + dist;
    }

    fn command(&mut self, comm: Command) {
        use Command::*;
        match comm {
            Forward(d) => self.forward(d),
            Up(d) => self.up(d),
            Down(d) => self.down(d),
        }
    }

    fn mul(&self) -> u32 {
        self.horiz * self.depth
    }
}

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Command {
    fn try_new(s: &str) -> Result<Self, String> {
        let parts: Vec<_> = s.split(' ').collect();
        if parts.len() != 2 {
            return Err(s.to_owned());
        }

        let comm = parts[0];
        let dist = parts[1];

        let d = match dist.parse() {
            Ok(d) => d,
            Err(e) => return Err(format!("{} : {}", dist, e)),
        };

        match comm {
            "forward" => Ok(Self::Forward(d)),
            "down" => Ok(Self::Down(d)),
            "up" => Ok(Self::Up(d)),
            _ => Err(comm.to_owned()),
        }
    }
}

fn get_input() -> Vec<Command> {
    let input = include_str!("input/2.txt");
    let input = input.split_terminator('\n');

    let input: Result<Vec<_>, _> = input.map(Command::try_new).collect();
    match input {
        Ok(v) => v,
        Err(e) => panic!("something malformed in input: {}", e),
    }
}

fn calc() -> u32 {
    let mut pos = Position::default();

    for command in get_input() {
        pos.command(command);
    }

    pos.mul()
}

pub fn pretty_print() {
    println!("final pos x final depth: {}", calc());
}

#[cfg(test)]
mod tests {
    use super::*;

    /// No longer holds true because Part 2 changed the behavior of `Position`, and I wasn't sure how to preserve both ways
    // #[test]
    // fn t_part1() {
    //     assert_eq!(part1(), 2120749);
    // }

    #[test]
    fn t_part2() {
        assert_eq!(calc(), 2138382217);
    }
}
