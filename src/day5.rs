// use tokio::main;
use core::fmt::Error;
use reqwest::header;
use std::fmt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::Client::builder().build()?
        .get("https://adventofcode.com/2020/day/5/input")
        .header(header::COOKIE, "_ga=GA1.2.1079033842.1608623467; _gid=GA1.2.387019889.1608623467; session=53616c7465645f5fbd922e1d768107e33afd9d6a7e015d8ccc22296fc7f34a2529cc2791daecc1ae83e11ba70e7b423b")
        .send()
        .await?
        .text().await?;
    println!("{}", execute(resp).await.unwrap());
    Ok(())
}


async fn execute(resp: String) -> Result<u32, Error> {
    let mut max = 0;
    let mut seats: Vec<Seat> = Vec::new();
    for line in resp.lines() {
        let seat = parse(String::from(line));
        if seat?.id > max {
            max = seat.unwrap().id;
        }
        seats.push(seat.unwrap());
    }
    // seats.into_iter().for_each(|s| {println!("{}", s)});
    seats.sort_by(|a, b| {a.id.cmp(&b.id)});
    for i in 1 .. seats.len() {
        if seats[i].id != (seats[i-1].id + 1) {
            println!("{} {} <-----", seats[i-1].id, seats[i].id );
        }
        else {
            println!("{} {}", seats[i-1].id, seats[i].id );
        }
    }
    Ok(max)
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Seat {
    row: u32,
    column: u32,
    id: u32
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{} ({})", self.row, self.column, self.id)
    }
}

fn parse_row(seat: String, low: u32, high: u32) -> u32 {
    // println!("{} {} {}", seat, low, high);
    if seat.chars().nth(0) == Some('F') {
        return parse_row(seat[1..].to_string(), low, (high - low) / 2 + low);
    }
    else if seat.chars().nth(0) == Some('B') {
        return parse_row(seat[1..].to_string(), (high - low) / 2 + 1 + low, high);
    }
    else {
        return low;
    }
}

fn parse_column(seat: String, low: u32, high: u32) -> u32 {
    // println!("{} {} {}", seat, low, high);
    if seat.chars().nth(0) == Some('L') {
        return parse_column(seat[1..].to_string(), low, (high - low) / 2 + low);
    }
    else if seat.chars().nth(0) == Some('R') {
        return parse_column(seat[1..].to_string(), (high - low) / 2 + 1 + low, high);
    }
    else {
        return low;
    }
}

fn parse(seat: String) -> Result<Seat, Error> {
    let row = parse_row(String::from(&seat), 0, 127);
    let column = parse_column(String::from(&seat[7..]), 0, 7);
    Ok(Seat{row, column, id:row*8+column})
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[tokio::test]
    async fn test_parse_row() {
        assert!(parse_row(String::from("FBFBBFFRLR"), 0, 127) == 44);
    }

    #[tokio::test]
    async fn test_parse_column() {
        assert!(parse_column(String::from("RLR"), 0, 7) == 5);
    }

    #[tokio::test]
    async fn test_parse() {
        assert!(parse(String::from("FBFBBFFRLR")).unwrap() == Seat {row: 44, column: 5, id: 357});
        assert!(parse(String::from("BFFFBBFRRR")).unwrap() == Seat {row: 70, column: 7, id: 567});
        assert!(parse(String::from("FFFBBBFRRR")).unwrap() == Seat {row: 14, column: 7, id: 119});
        assert!(parse(String::from("BBFFBBFRLL")).unwrap() == Seat {row: 102, column: 4, id: 820});
    }

    

}