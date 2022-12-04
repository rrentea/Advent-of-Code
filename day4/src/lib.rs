use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Section {
    start: u32,
    stop: u32
}

impl Section {
    fn contains(&self, section: Section) -> bool {
        self.start >= section.start && self.stop <= section.stop
    }

    fn overlap(&self, section: Section) -> bool {
        if self.contains(section) {
            return true;
        }

        if (self.start..=self.stop).contains(&section.start) {
            return true;
        } else if (self.start..=self.stop).contains(&section.stop) {
            return true;
        } 
        
        return false;
    }
}

impl FromStr for Section {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<u32> = s
            .split("-")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        Ok(Section {
            start: split[0],
            stop: split[1]
        })

    }
}

pub fn process_part1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let sections: Vec<Section> = line
                .split(",")
                .map(|section| section.parse::<Section>().unwrap())
                .collect(); 
            if sections[0].contains(sections[1]) || sections[1].contains(sections[0]) {
                1 as u32
            } else {
                0 as u32
            }
        })
        .sum();

    result.to_string()
}


pub fn process_part2(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let sections: Vec<Section> = line
                .split(",")
                .map(|section| section.parse::<Section>().unwrap())
                .collect(); 
            if sections[0].overlap(sections[1]) || sections[1].overlap(sections[0]) {
                1 as u32
            } else {
                0 as u32
            }
        })
        .sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "4");
    }
}
