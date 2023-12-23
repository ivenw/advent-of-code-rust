fn main() {
    let input = include_str!("../input.txt");
    let boxes = get_presents(input);
    let wrapping_paper = wrap_presents(&boxes);
    let ribbon = tie_ribbons(&boxes);
    println!("{}", wrapping_paper);
    println!("{}", ribbon);
}

fn get_presents(input: &str) -> Vec<Present> {
    input
        .lines()
        .map(|line| line.parse::<Present>().unwrap())
        .collect()
}

fn wrap_presents(boxes: &[Present]) -> u32 {
    boxes
        .iter()
        .map(|present| present.surface_area() + present.min_side_area())
        .sum()
}

fn tie_ribbons(boxes: &[Present]) -> u32 {
    boxes
        .iter()
        .map(|present| present.volume() + present.smallest_perimeter())
        .sum()
}

struct Present {
    length: u32,
    width: u32,
    height: u32,
}
impl Present {
    fn new(length: u32, width: u32, height: u32) -> Present {
        Present {
            length,
            width,
            height,
        }
    }
    fn surface_area(&self) -> u32 {
        2 * (self.length * self.width + self.width * self.height + self.height * self.length)
    }
    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }
    fn sorted_sides(&self) -> [u32; 3] {
        let mut sides = [self.length, self.width, self.height];
        sides.sort();
        sides
    }
    fn min_side_area(&self) -> u32 {
        let sides = self.sorted_sides();
        sides[0] * sides[1]
    }
    fn smallest_perimeter(&self) -> u32 {
        let sides = self.sorted_sides();
        2 * (sides[0] + sides[1])
    }
}
impl std::str::FromStr for Present {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('x');
        let length = split.next().unwrap().parse::<u32>()?;
        let width = split.next().unwrap().parse::<u32>()?;
        let height = split.next().unwrap().parse::<u32>()?;
        Ok(Present::new(length, width, height))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_presents() {
        assert_eq!(wrap_presents(&[Present::new(2, 3, 4)]), 58);
        assert_eq!(wrap_presents(&[Present::new(1, 1, 10)]), 43);
    }

    #[test]
    fn test_tie_ribbons() {
        assert_eq!(tie_ribbons(&[Present::new(2, 3, 4)]), 34);
        assert_eq!(tie_ribbons(&[Present::new(1, 1, 10)]), 14);
    }
}
