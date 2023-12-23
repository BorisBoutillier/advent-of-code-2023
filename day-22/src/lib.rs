use aoc_commons::*;
#[derive(Clone, Debug, PartialEq, Eq)]
struct Brick {
    pos_1: Vec<isize>,
    pos_2: Vec<isize>,
    dir: usize,
    size: usize,
}
impl Brick {
    fn parse(line: &str) -> Brick {
        let (p1, p2) = line.split_once('~').unwrap();
        let mut pos_1 = p1
            .split(',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let mut pos_2 = p2
            .split(',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let dir = if pos_1[0] != pos_2[0] {
            0
        } else if pos_1[1] != pos_2[1] {
            1
        } else {
            2
        };
        if pos_2[dir] < pos_1[dir] {
            (pos_1, pos_2) = (pos_2, pos_1);
        }
        let size = (pos_2[dir] - pos_1[dir] + 1) as usize;
        Brick {
            pos_1,
            pos_2,
            dir,
            size,
        }
    }
    fn intersect(&self, pos: &[isize]) -> bool {
        (0..3).all(|d| {
            if d == self.dir {
                (self.pos_1[d]..=self.pos_2[d]).contains(&pos[d])
            } else {
                self.pos_1[d] == pos[d]
            }
        })
    }
    fn all_pos(&self) -> Vec<Vec<isize>> {
        let mut pos = (0..self.size)
            .map(|_| self.pos_1.clone())
            .collect::<Vec<_>>();
        for (i, v) in (self.pos_1[self.dir]..=self.pos_2[self.dir]).enumerate() {
            pos[i][self.dir] = v;
        }
        pos
    }
    fn fall_1(&mut self) {
        self.pos_1[2] -= 1;
        self.pos_2[2] -= 1;
    }
}
#[derive(Clone)]
struct Bricks {
    bricks: Vec<Brick>,
}
impl Bricks {
    fn fall(&mut self) {
        //println!("FALLING");
        //for (i, brick) in self.bricks.iter().enumerate() {
        //    println!("  {i}: {brick:?}");
        //}
        loop {
            let can_fall = self.get_bricks_that_can_fall(None);
            if can_fall.is_empty() {
                break;
            }
            //println!("Can Fall: {can_fall:?}");
            for i in can_fall {
                self.bricks[i].fall_1();
            }
            //println!("After Fall");
            //for (i, brick) in self.bricks.iter().enumerate() {
            //    println!("  {i}: {brick:?}");
            //}
        }
        //println!("DONE FALLING");
    }
    fn get_bricks_that_can_fall(&self, without: Option<usize>) -> Vec<usize> {
        //println!("Can fall");
        let mut can_fall = vec![];
        for (i, brick) in self
            .bricks
            .iter()
            .enumerate()
            .filter(|(i, _)| Some(*i) != without)
        {
            let mut fall_pos = brick.all_pos();
            fall_pos.iter_mut().for_each(|pos| pos[2] -= 1);
            //println!("  Check {i} -> {fall_pos:?}");
            if fall_pos.iter().any(|pos| pos[2] == 0) {
                continue;
            }
            let brick_can_fall = fall_pos.iter().all(|pos| {
                self.bricks
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| j != &i && Some(*j) != without)
                    .all(|(_j, brick2)| !brick2.intersect(pos))
            });
            if brick_can_fall {
                can_fall.push(i);
            }
        }
        can_fall
    }
    fn count_can_be_desintegrated(&self) -> usize {
        //println!("Can be desintegrated ?");
        (0..self.bricks.len())
            .filter(|i| {
                let would_fall = self.get_bricks_that_can_fall(Some(*i));
                //println!("{i} : {would_fall:?} ; {:?}", self.bricks[*i]);
                would_fall.is_empty()
            })
            .count()
    }
}
pub fn solver(part: Part, input: &str) -> String {
    let mut bricks = Bricks {
        bricks: input.lines().map(Brick::parse).collect::<Vec<_>>(),
    };
    bricks.fall();
    //println!("After Fall");
    //for brick in bricks.bricks.iter() {
    //    println!(" {brick:?}");
    //}
    match part {
        Part::Part1 => bricks.count_can_be_desintegrated(),
        Part::Part2 => {
            let mut impacts = 0;
            println!("LEN: {}", bricks.bricks.len());
            for i in 0..bricks.bricks.len() {
                println!(" {i}");
                let mut start_bricks = bricks.clone();
                start_bricks.bricks.remove(i);
                let mut fall_bricks = start_bricks.clone();
                fall_bricks.fall();
                let impact = start_bricks
                    .bricks
                    .iter()
                    .zip(fall_bricks.bricks.iter())
                    .filter(|(b1, b2)| b1 != b2)
                    .count();
                println!("  -> {impact}");
                impacts += impact;
            }
            impacts
        }
    }
    .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(solver(Part::Part1, include_str!("../example.txt")), "5");
    }
    #[test]
    fn example_part2() {
        assert_eq!(solver(Part::Part1, include_str!("../example.txt")), "7");
    }
}
