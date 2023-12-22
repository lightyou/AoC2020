// use tokio::main;
use reqwest::header;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::Client::builder().build()?
        .get("https://adventofcode.com/2020/day/3/input")
        .header(header::COOKIE, "_ga=GA1.2.1079033842.1608623467; _gid=GA1.2.387019889.1608623467; session=53616c7465645f5fbd922e1d768107e33afd9d6a7e015d8ccc22296fc7f34a2529cc2791daecc1ae83e11ba70e7b423b")
        .send()
        .await?
        .text().await?;
    let grid: Vec<Vec<char>> = resp.lines().into_iter().map(|val| { 
        val.chars().collect::<Vec<_>>()
    }).collect();
    let mut grandtotal: i64 = 1;
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for slope in slopes {
        let mut xp = 0;
        let mut yp = 0;
        let mut total = 0;
        for x in 0 .. grid.len() {
            for y in 0 .. grid[0].len() {
                if x == xp && y == yp {
                    yp = (yp + slope.0) % grid[0].len();
                    xp = xp + slope.1;
                    if xp >= grid.len() { continue }
                    if grid[xp][yp] == '#' {
                        // print!("X");
                        total += 1;
                    }
                    else {
                        // print!("O");
                    }
                }
                else {
                    // print!("{}", grid[x][y]);
                }
            }
            // println!(" {} {}", xp, yp);
        }
        grandtotal *= total;
        println!("{} {}", total, grandtotal);
    }
    println!("{}", grandtotal);
    Ok(())
}