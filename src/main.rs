use core::fmt::Error;
use reqwest::header;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::Client::builder().build()?
        .get("https://adventofcode.com/2020/day/10/input")
        .header(header::COOKIE, "_ga=GA1.2.1079033842.1608623467; _gid=GA1.2.919644257.1616490123; session=53616c7465645f5fce4b2e369b19f74ea6f6a655e180fc7ea48946dfe81eb46c7506311152cf85fe4a5f78d1cb22fb40")
        .send()
        .await?
        .text().await?;
    println!("{:?}", resp);
    println!("{:?}", execute(resp).await.or_else(|_| Err(0)));
    Ok(())
}

async fn execute(resp: String) -> Result<usize, Error> {
    let mut values: Vec<usize> = resp.lines().map(|string| string.trim().parse::<usize>().unwrap()).collect();
    values.sort();
    println!("{:?}", values);
let (one, three) = fun_name(values);
    println!("one {:?} three {:?}", one, three);
    Ok(one * three)
}

fn fun_name(values: Vec<usize>) -> (i32, i32) {
    let mut val: usize  = 0;
    let  (mut one, mut three) = (0, 1);
    values.into_iter().for_each(|elem| {
        match elem - val {
            1 => one += 1,
            3 => three += 1,
            _ => ()
        };
        val = elem;
    });
    (one, three)
}


#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_rules() {
        let instructions = r#"16
10
15
5
1
11
7
19
6
12
4"#;
        let value = execute(String::from(instructions)).await.unwrap();
        println!("{}", value);
        assert!(35 == value);
    }

}