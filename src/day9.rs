use core::fmt::Error;
use reqwest::header;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::Client::builder().build()?
        .get("https://adventofcode.com/2020/day/9/input")
        .header(header::COOKIE, "_ga=GA1.2.1079033842.1608623467; _gid=GA1.2.387019889.1608623467; session=53616c7465645f5fbd922e1d768107e33afd9d6a7e015d8ccc22296fc7f34a2529cc2791daecc1ae83e11ba70e7b423b")
        .send()
        .await?
        .text().await?;
    println!("{:?}", execute(resp, 25).await.or_else(|_| Err(0)));
    Ok(())
}

async fn execute(resp: String, sz: usize) -> Result<usize, Error> {
    let value = execute_partone(&resp, sz).await.unwrap() as usize;
    let values: Vec<usize> = resp.lines().map(|string| string.trim().parse::<usize>().unwrap()).collect();
    for sz in 2..values.len()-2 {
        for i in 0..values.len()-sz {
            let window = &values[i..i+sz];
            let sum = window.iter().fold(0, |acc, x| acc + *x);
            if sum == value {
                let min = window.iter().min().unwrap();
                let max = window.iter().max().unwrap();
                println!("i {} sz {} min {} max {} value {} sum {} {:?}", i, sz, min, max, value, sum, window);
                if *max < value {
                    return Ok(min + max)
                }
            }
        }
    }
    Err(Error {})
}

async fn execute_partone(resp: &String, sz: usize) -> Result<i32, Error> {
    let values: Vec<usize> = resp.lines().map(|string| string.trim().parse::<usize>().unwrap()).collect();
    for i in 0..=(values.len() - sz) {
        let window = &values[i..i+sz];
        let next = values[i+sz];
        let combinations: Vec<usize> = window.iter().map(|x| window.iter().map(move |y| x+y)).flatten().collect();
        if !combinations.contains(&next) {
            return Ok(next as i32)
        }
    }
    Ok(0)
}


#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_rules() {
        let instructions = "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";
        let value = execute(String::from(instructions), 5).await.unwrap();
        println!("{}", value);
        assert!(127 == value);
    }

}