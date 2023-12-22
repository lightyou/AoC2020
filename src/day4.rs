// use tokio::main;
use core::fmt::Error;
use reqwest::header;
#[derive(Debug, Copy, Clone)]
struct Passport<'a> {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<u32>,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::Client::builder().build()?
        .get("https://adventofcode.com/2020/day/4/input")
        .header(header::COOKIE, "_ga=GA1.2.1079033842.1608623467; _gid=GA1.2.387019889.1608623467; session=53616c7465645f5fbd922e1d768107e33afd9d6a7e015d8ccc22296fc7f34a2529cc2791daecc1ae83e11ba70e7b423b")
        .send()
        .await?
        .text().await?;
    println!("{}", execute(resp).await.unwrap());
    Ok(())
}

fn validate_height(value: &str) -> bool {
    if let Some(index) = value.find("cm") {
        let heigth = value[0..index].parse::<u32>().expect("Number");
        return 150 <= heigth && heigth <= 193;
    }
    if let Some(index) = value.find("in") {
        let heigth = value[0..index].parse::<u32>().expect("Number");
        return 59 <= heigth && heigth <= 76;
    }
    false
}
fn validate_hair(value: &str) -> bool {
    let re = regex::Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    return re.is_match(&value);
}
fn validate_eyes(value: &str) -> bool {
    match value {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => return true,
        _ => return false
    }
}
fn validate_pid(value: &str) -> bool {
    let re = regex::Regex::new(r"^\d{9}$").unwrap();
    return re.is_match(&value);
}
async fn execute(resp: String) -> Result<u32, Error> {
    let mut passports: Vec<Passport> = Vec::new();
    let mut passport = Passport { byr: None, iyr: None, eyr: None, hgt: None, hcl: None, ecl: None, pid: None, cid: None, };
    for line in resp.lines() {
        if line.len() == 0 {
            passports.push(passport);
            passport = Passport { byr: None, iyr: None, eyr: None, hgt: None, hcl: None, ecl: None, pid: None, cid: None, };
        }
        else {
            for elem in line.split(" ") {
                let pair: Vec<&str> = elem.split(":").collect();
                let (name, value) = (pair[0], pair[1]);
                match name {
                    "byr"  if value.len() == 4 && 1920 <= value.parse::<u32>().unwrap() && value.parse::<u32>().unwrap() <= 2002 => passport.byr = Some(value.parse().unwrap()) ,
                    "iyr"  if value.len() == 4 && 2010 <= value.parse::<u32>().unwrap() && value.parse::<u32>().unwrap() <= 2020 => passport.iyr = Some(value.parse().unwrap()),
                    "eyr" if value.len() == 4 && 2020 <= value.parse::<u32>().unwrap() && value.parse::<u32>().unwrap() <= 2030 => passport.eyr = Some(value.parse().unwrap()),
                    "hgt" if validate_height(value) => passport.hgt = Some(value),
                    "hcl" if validate_hair(value) => passport.hcl = Some(value),
                    "ecl" if validate_eyes(value) => passport.ecl = Some(value),
                    "pid" if validate_pid(value) => passport.pid = Some(value),
                    "cid" => passport.cid = Some(value.parse().unwrap()),
                    _ => () //println!("Should never append {}", name)
                }
            }
        }
    }
    passports.push(passport);
    let mut total = 0;
    for p in passports {
        if p.byr.is_some() && p.iyr.is_some() && p.eyr.is_some() && p.hgt.is_some() && p.hcl.is_some() && p.ecl.is_some() && p.pid.is_some() {
               total += 1;
        }
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", p.byr, p.iyr, p.eyr, p.hgt, p.hcl, p.ecl, p.pid, p.cid);
    }
    println!("{}", total);
    Ok(total)
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[tokio::test]
    async fn test() {
        let invalid: String = String::from("
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007");
        assert!(0 == execute(invalid).await.unwrap());
    }

    #[tokio::test]
    async fn should_count_valid() {
        let valid: String = String::from("
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719");
       assert!(4 == execute(valid).await.unwrap());
    }
}