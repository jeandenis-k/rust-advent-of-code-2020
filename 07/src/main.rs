use regex::Regex;
use std::io::BufRead;

#[derive(Debug)]
struct Rule {
    color: String,
    contained: Vec<(usize, String)>,
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let file = std::fs::File::open(&args[1]).unwrap();
    let rules: Vec<Rule> = std::io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let re = Regex::new(r"(.*) bags contain (.*)").unwrap();
            let matches = re.captures(&line).unwrap();
            let color = matches.get(1).unwrap().as_str().to_string();

            let contained_string = matches.get(2).unwrap().as_str();
            if contained_string == "no other bags." {
                Rule {
                    color,
                    contained: vec![],
                }
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
                Rule {
                    color,
                    contained: contained_parts,
                }
            }
        })
        .collect();
    println!("{:?}", rules);
}
