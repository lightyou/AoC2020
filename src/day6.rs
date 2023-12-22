// use tokio::main;
use core::fmt::Error;
use reqwest::header;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::Client::builder().build()?
        .get("https://adventofcode.com/2020/day/6/input")
        .header(header::COOKIE, "_ga=GA1.2.1079033842.1608623467; _gid=GA1.2.387019889.1608623467; session=53616c7465645f5fbd922e1d768107e33afd9d6a7e015d8ccc22296fc7f34a2529cc2791daecc1ae83e11ba70e7b423b")
        .send()
        .await?
        .text().await?;
    println!("{}", execute(resp).await.unwrap());
    Ok(())
}


async fn execute(resp: String) -> Result<i32, Error> {
    let mut total = 0;
    let mut lines: String = String::from("");
    for line in resp.lines() {
        println!("-- {} --", line);
        if line.len() == 0 {
            total += count_group(lines);
            lines = String::from("");
        }
        else {
            lines.push_str(line);
            lines.push_str("\n");
        }
    }
    total += count_group(lines);
    Ok(total)
}


fn count_group(group: String) -> i32 {
    let mut questions = std::collections::HashMap::<char, i32>::new();
    let mut nb_person: i32 = 0;
    for line in group.lines() {
        if line.len() > 0 {
            nb_person += 1;
        }
        println!("{}", line);
        for a in line.chars() {
            *questions.entry(a).or_insert(0) += 1;
            println!("{} {}", a, questions[&a]);
        }
    }
    let val = questions.values().fold(0, |acc, x| {
        println!("{} {}", *x, nb_person);
        if *x == nb_person { 
            return acc + 1; 
        } else { 
            return acc; 
        }
    });
    println!("{}", val);
    val
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[tokio::test]
    async fn test_count_group() {
        assert!(count_group(String::from("
abc")) == 3);
        assert!(count_group(String::from("
a
b
c")) == 0);
        assert!(count_group(String::from("
ab
ac")) == 1);
        assert!(count_group(String::from("
a
a
a
a")) == 1);
        assert!(count_group(String::from("
b")) == 1);
    }



    

}