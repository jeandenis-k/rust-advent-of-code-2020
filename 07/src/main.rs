use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug)]
struct BagMap(HashMap<String, Vec<(usize, String)>>);

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let file = std::fs::File::open(&args[1]).unwrap();
    let bag_map: BagMap = BagMap(
        std::io::BufReader::new(file)
            .lines()
            .filter_map(Result::ok)
            .map(|line| {
                let re = Regex::new(r"(.*) bags contain (.*)").unwrap();
                let matches = re.captures(&line).unwrap();
                let color = matches.get(1).unwrap().as_str().to_string();

                let contained_string = matches.get(2).unwrap().as_str();
                if contained_string == "no other bags." {
                    (color, vec![])
                } else {
                    let contained_parts: Vec<_> = contained_string
                        .split(",")
                        .map(|part| {
                            let re = Regex::new(r"(\d) (.*) bag").unwrap();
                            let matches = re.captures(&part).unwrap();
                            (
                                matches.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                                matches.get(2).unwrap().as_str().to_string(),
                            )
                        })
                        .collect();
                    (color, contained_parts)
                }
            })
            .collect(),
    );
    println!("{:?}", bag_map);

    println!(
        "Solution is {}",
        bag_map
            .0
            .keys()
            .filter(|color| bag_map.color_contains_shiny(color))
            .count()
    );
}

impl BagMap {
    fn color_contains_shiny(self: &BagMap, color: &String) -> bool {
        let children = self.0.get(color).unwrap();
        let has_direct_shiny_gold = children.iter().any(|(_, color)| color == "shiny gold");
        has_direct_shiny_gold
            || children
                .iter()
                .any(|(_, color)| self.color_contains_shiny(color))
    }
}
