// use tokio::main;
use reqwest::header;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::Client::builder().build()?
        .get("https://adventofcode.com/2020/day/1/input")
        .header(header::COOKIE, "_ga=GA1.2.1079033842.1608623467; _gid=GA1.2.387019889.1608623467; session=53616c7465645f5fbd922e1d768107e33afd9d6a7e015d8ccc22296fc7f34a2529cc2791daecc1ae83e11ba70e7b423b")
        .send()
        .await?
        .text().await?;
    let lines: Vec<u32> = resp.lines().into_iter().map(|val| {val.parse::<u32>().expect("Not an int")}).collect();
    for x in &lines {
        for y in &lines {
            for z in &lines {
                if x + y + z == 2020 {
                    println!("{:#?}", x*y*z);
                }
            }
        }
    }
    Ok(())
}