// use tokio::main;
use reqwest::header;

struct Password<'a> {
    min: usize,
    max: usize,
    letter: char,
    password: &'a str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::Client::builder().build()?
        .get("https://adventofcode.com/2020/day/2/input")
        .header(header::COOKIE, "_ga=GA1.2.1079033842.1608623467; _gid=GA1.2.387019889.1608623467; session=53616c7465645f5fbd922e1d768107e33afd9d6a7e015d8ccc22296fc7f34a2529cc2791daecc1ae83e11ba70e7b423b")
        .send()
        .await?
        .text().await?;
    let lines: Vec<Password> = resp.lines().into_iter().map(|val| { 
        let result: Vec<&str> = val.split(" ").collect();
        let (rule, letter, password) = (&result[0], &result[1], &result[2]);
        let minmax : Vec<usize> = rule.split("-").map(|val| { val.parse::<usize>().expect("int") }).collect();
        Password { 
            min: minmax[0],
            max: minmax[1],
            letter: letter.chars().nth(0).expect("a char"),
            password: *password }
    }).collect();
    let mut total = 0;
    for x in &lines {
        println!("OK {}-{} {}: {}", x.min, x.max, x.letter, x.password);
        if (x.password.chars().nth(x.min - 1).unwrap() == x.letter) ^ (x.password.chars().nth(x.max - 1).unwrap() == x.letter) {
            total = total + 1;
        }
        // let count = x.password.split(x.letter).count() - 1;
        // if count >= x.min && count <= x.max {
        //     println!("count {} OK {}-{} {}: {}", count, x.min, x.max, x.letter, x.password);
        //     total = total + 1;
        // }
    }
    println!("Total ok with the rule : {}", total);
    Ok(())
}