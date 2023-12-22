use core::fmt::Error;
use reqwest::header;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::Client::builder().build()?
        .get("https://adventofcode.com/2020/day/7/input")
        .header(header::COOKIE, "_ga=GA1.2.1079033842.1608623467; _gid=GA1.2.387019889.1608623467; session=53616c7465645f5fbd922e1d768107e33afd9d6a7e015d8ccc22296fc7f34a2529cc2791daecc1ae83e11ba70e7b423b")
        .send()
        .await?
        .text().await?;
    println!("{}", execute(resp).await.unwrap());
    Ok(())
}


async fn execute(resp: String) -> Result<u64, Error> {
    let bags = parse_bags(resp);
    let total = search_bag(&String::from("shiny gold"), &String::from("shiny gold"),&bags).unwrap_or(0)-1;
    Ok(total)
}

#[derive(Debug)]
struct InBag {
    color: String,
    card: u32,
}
#[derive(Debug)]
struct Bag {
    color: String,
    contains: Vec<InBag>,
}

// impl Display for Bag {
//     fn fmt()
// }

fn search_bag(bag: &String, current: &String, bags: &Vec<Bag>) -> Option<u64> {
    let mut sum: u64 = 0;
    for this_bag in bags.iter().filter(|b| b.color == *current) {
        println!(">> b.color {}", this_bag.color);
        sum += 1;
        if this_bag.contains.iter().fold(0, |c, acc| acc.card + c) > 0 {
            for inside in this_bag.contains.iter() {
                let inside_count = search_bag(&bag, &inside.color, bags);
                sum += inside_count.unwrap_or(0) * inside.card as u64;
                println!(">> << inside.color {} inside.card {} inside_count {} total {}", inside.color, inside.card, inside_count.unwrap_or(0), sum);
            }
        }
        else {
            break;
        }
    }
    Some(sum)
}

fn parse_bags(rules: String) -> Vec<Bag> {
    let mut bags = Vec::<Bag>::new();
    for line in rules.lines() {
        bags.push(parse_bag(String::from(line)).unwrap());
    }
    bags
}

fn parse_bag(rule: String) -> Result<Bag, Error> {
    let re = Regex::new(r"^(\w* \w*) bags contains? ([^,]*,?)([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?([^,]*,?)?\s*\.").unwrap();
    // println!("{}", rule);
    let caps = re.captures(&rule).unwrap();
    let color = String::from(caps.get(1).map_or("", |m| m.as_str()));
    let contains = caps.iter().enumerate().filter(|&(i, _)| i > 1 ).map(|(_, bag)| {
        let pattern = bag.map_or("", |m| m.as_str());
        // println!("|{}|", pattern);
        if pattern == "" { 
            return InBag {color: String::from(""), card:0}; 
        }
        let re = Regex::new(r"\s*(\d?|no) (.*) bags?,?").unwrap();
        let bag_caps = re.captures(&pattern).unwrap();
        let nb = bag_caps.get(1).map_or("", |m| m.as_str()).parse::<u32>();
        let color = String::from(bag_caps.get(2).map_or("", |m| m.as_str()));
        // println!("{:#?} {}", nb, color);
        match nb {
            Err(_) => return InBag {color, card:0},
            Ok(card) => return InBag {color, card},
        }
    }).filter(|x| x.card != 0).collect();
    
    Ok(Bag { color, contains })
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_rules() {
        let rules = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
");
        let bags = parse_bags(rules);
        assert!(bags[0].color == "light red");
        // let mut value = 0;
        // for this_bag in bags.iter() {
            let value = search_bag(&String::from("shiny gold"), &String::from("shiny gold"),&bags).unwrap_or(0);
            // value += temp.unwrap_or(0);
            // println!("{} {}", this_bag.color, temp.unwrap_or(0));
        // }
        println!("{}", value);
        assert!(value == 32);
    }
    #[tokio::test]
    async fn test_rule() {
        let rule = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags, 3 vibrant red bags.");
        let bag = parse_bag(rule).unwrap();
        println!("{}", bag.color);
        assert!(bag.color == "light red");
        assert!(bag.contains[0].color == "bright white");

    }

}