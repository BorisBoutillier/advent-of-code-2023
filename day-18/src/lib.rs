use aoc_commons::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum VDirection {
    Up,
    Down,
}
#[derive(Debug)]
struct VWall {
    x: i64,
    y1: i64,
    y2: i64,
}
impl VWall {
    fn border(&self, y: i64) -> Option<VDirection> {
        use VDirection::*;
        if self.y1 == y {
            Some(Down)
        } else if self.y2 == y {
            Some(Up)
        } else {
            None
        }
    }
}
pub fn solver(part: Part, input: &str) -> String {
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut vwalls = vec![];
    let mut min_y = pos_y;
    let mut max_y = pos_y;
    let mut perimeter = 0;
    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();
        let mut dir = split.next().unwrap();
        let mut length = split.next().unwrap().parse::<i64>().unwrap();
        if part == Part::Part2 {
            let color = split.next().unwrap();
            let color = i64::from_str_radix(&color[2..(color.len() - 1)], 16).unwrap();
            dir = match color % 16 {
                0 => "R",
                1 => "D",
                2 => "L",
                3 => "U",
                _ => panic!("WHAT ? {}", color % 16),
            };
            length = color / 16;
        }
        assert!(length > 0);
        perimeter += length;
        match dir {
            "R" => {
                pos_x += length;
            }
            "L" => {
                pos_x -= length;
            }
            "D" => {
                let new_y = pos_y + length;
                vwalls.push(VWall {
                    x: pos_x,
                    y1: pos_y,
                    y2: new_y,
                });
                pos_y = new_y;
            }
            "U" => {
                let new_y = pos_y - length;
                vwalls.push(VWall {
                    x: pos_x,
                    y1: new_y,
                    y2: pos_y,
                });
                pos_y = new_y;
            }
            _ => panic!("What ? {dir}"),
        }
        min_y = min_y.min(pos_y);
        max_y = max_y.max(pos_y);
    }
    assert_eq!(pos_x, 0);
    assert_eq!(pos_y, 0);
    vwalls.sort_by_key(|wall| wall.x);
    use VDirection::*;
    let mut count = 0;
    for y in min_y..=max_y {
        //println!("Y:{y}");
        let mut x = i64::MIN;
        let mut inside = false;
        let mut border = None;
        for vwall in vwalls.iter().filter(|vwall| vwall.y1 <= y && vwall.y2 >= y) {
            //println!("  Count:{count} B:{border:?} I:{inside}, X:{x}");
            //println!("    Wall: {:?}", vwall);
            let new_x = vwall.x;
            if inside && border.is_none() {
                count += new_x - x - 1;
            }
            match (border, vwall.border(y)) {
                (None, None) => {
                    inside = !inside;
                }
                (None, Some(x)) => {
                    border = Some(x);
                }
                (Some(Up), Some(Up)) | (Some(Down), Some(Down)) => {
                    border = None;
                }
                (Some(Up), Some(Down)) | (Some(Down), Some(Up)) => {
                    border = None;
                    inside = !inside;
                }
                (b1, b2) => panic!("Unexpeced {:?} {:?}", b1, b2),
            }
            x = new_x;
            //println!("    -> Count:{count} B:{border:?} I:{inside}, X:{x}");
        }
        assert!(!inside);
        assert!(x != i64::MIN);
        //println!(" -> {Count}");
    }
    count += perimeter;
    count.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1_1() {
        assert_eq!(solver(Part::Part1, include_str!("../example.txt")), "62");
    }
    #[test]
    fn example_part1_2() {
        assert_eq!(solver(Part::Part1, include_str!("../example.2.txt")), "69");
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solver(Part::Part2, include_str!("../example.txt")),
            "952408144115"
        );
    }
}
