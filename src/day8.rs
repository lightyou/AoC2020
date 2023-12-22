use core::fmt::Error;
use reqwest::header;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::Client::builder().build()?
        .get("https://adventofcode.com/2020/day/8/input")
        .header(header::COOKIE, "_ga=GA1.2.1079033842.1608623467; _gid=GA1.2.387019889.1608623467; session=53616c7465645f5fbd922e1d768107e33afd9d6a7e015d8ccc22296fc7f34a2529cc2791daecc1ae83e11ba70e7b423b")
        .send()
        .await?
        .text().await?;
    println!("{:?}", execute(resp).await.or_else(|_| Err(0)));
    Ok(())
}

async fn execute(resp: String) -> Result<i32, Error> {
    let instructions: Vec<&str> = resp.lines().collect();
    for i in 0..=instructions.len() {
        let mut current = instructions.iter().map(|value| value.to_string()).collect::<Vec<String>>();
        let array: Vec<&str> = current[i].split(" ").collect();
        let (op, value) = (array[0], array[1].parse::<i32>().unwrap());
        current[i] = match op {
            "jmp" => format!("nop {}", value),
            "nop" => format!("jmp {}", value),
            _ => current[i].clone()
        };
        let x = subexecute(current).await;
        if x.is_ok() {
            return x
        }
    }
    Err(Error {})
}

async fn subexecute(instructions: Vec<String>) -> Result<i32, Error> {
    let mut state: State = State {pc: 0, acc: 0};
    let mut used: std::collections::HashMap<u32, bool> = std::collections::HashMap::new();
    loop {
        let instruction = instructions[state.pc as usize].clone();
        // println!("{} {:?}", instruction, state);
        state = run(instruction, state);
        if state.pc as usize >= instructions.len() {
            return Ok(state.acc)
        }
        if used.contains_key(&state.pc) {
            return Err(Error {});
        }
        used.insert(state.pc, true);
    }
}
#[derive(Debug, Copy, Clone)]
struct State {
    acc: i32,
    pc: u32,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.acc == other.acc &&
        self.pc == other.pc
    }
}

fn run(instruction: String, state: State) -> State {
    let array: Vec<&str> = instruction.split(" ").collect();
    let (op, value) = (array[0], array[1].parse::<i32>().unwrap());
    let mut ret = state;
    match op {
        "nop" => ret.pc += 1,
        "jmp" => ret.pc = ((ret.pc as i32) + value) as u32,
        "acc" => { 
            ret.acc += value;
            ret.pc += 1
        },
        _ => ()
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_rules() {
        let instructions = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let value = execute(String::from(instructions)).await.unwrap();
        println!("{}", value);
        assert!(5 == value);
    }
    #[tokio::test]
    async fn test_rule() {
        assert!(State {pc:1, acc:0} == run("nop +1", State {pc:0, acc:0}));
        assert!(State {pc:2, acc:0} == run("jmp +2", State {pc:0, acc:0}));
        assert!(State {pc:0, acc:-2} == run("acc -2", State {pc:0, acc:0}));
    }

}