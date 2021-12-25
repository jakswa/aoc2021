use std::thread;

fn main() {
    let instruct_input = std::fs::read_to_string("input.txt").unwrap();
    let instructs = instruct_input
        .lines()
        .map(|line| {
            //println!("proc: {}", line);
            let mut line = line.split_whitespace();
            let cmd = line.next().unwrap();
            let arg1 = reg_index(&line.next().unwrap().chars().next().unwrap());
            let arg2 = line.next().unwrap_or("99999");
            let arg2i = arg2.parse::<i64>().unwrap_or(99999);
            let arg2c = match arg2 {
                "w" => 'w',
                "x" => 'x',
                "y" => 'y',
                "z" => 'z',
                _ => '_',
            };
            (cmd_from_str(cmd), arg1, arg2c, arg2i)
        })
        .collect::<Vec<(Cmd, usize, char, i64)>>();

    // println!("check: {}", process(95299897999897, &instructs[..]));
    let mut handles = vec![];
    (1..=3).for_each(|i| {
        let inst = instructs.clone();
        handles.push(thread::spawn(move || {
            process_one(0, i, &mut [0, 0, 0, 0], &inst);
            println!("1/10th completed: {}", i);
        }));
    });
    handles.drain(..).for_each(|h| {
        h.join();
    });
    println!("\n\ndone.");
}

#[derive(Clone, Debug)]
enum Cmd {
    Mul,
    Add,
    Eql,
    Mod,
    Div,
    Inp,
}

fn cmd_from_str(cmd: &str) -> Cmd {
    match cmd {
        "inp" => Cmd::Inp,
        "mul" => Cmd::Mul,
        "add" => Cmd::Add,
        "div" => Cmd::Div,
        "mod" => Cmd::Mod,
        "eql" => Cmd::Eql,
        _ => Cmd::Inp,
    }
}

fn input_conv(input: i64) -> Vec<i64> {
    format!("{}", input)
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>()
}

fn process_one(
    mut depth: u8,
    input: i64,
    registers: &mut [i64; 4],
    instructs: &Vec<(Cmd, usize, char, i64)>,
) -> bool {
    if input == 0 {
        return false;
    }
    depth += 1;
    let histlen = depth;
    let mut go_down = false;
    let orig = registers[3];

    for (cmd, reg_ind, val2c, val2i) in
        &instructs[(histlen as usize - 1) * 18..(histlen as usize * 18)]
    {
        //println!("processing: {:?}", (&hist, cmd, &registers));
        //instructs.iter().for_each(|(cmd, reg_ind, val2c, val2i)| {
        let right = match val2c {
            'w' => registers[0],
            'x' => registers[1],
            'y' => registers[2],
            'z' => registers[3],
            _ => *val2i, // '_' in reality
        };
        //print!(">>>>>: {:?}", (cmd, reg_ind, val2c, val2i));
        match cmd {
            Cmd::Inp => {
                registers[*reg_ind] = input;
            }
            Cmd::Add => {
                let left = registers[*reg_ind];
                registers[*reg_ind] = left + right;
            }
            Cmd::Mul => {
                let left = registers[*reg_ind];
                registers[*reg_ind] = left * right;
            }
            Cmd::Div => {
                let left = registers[*reg_ind];
                if right == 26 {
                    go_down = true;
                }
                registers[*reg_ind] = left / right;
            }
            Cmd::Mod => {
                let left = registers[*reg_ind];
                registers[*reg_ind] = left % right;
            }
            Cmd::Eql => {
                let left = registers[*reg_ind];
                if left == right {
                    registers[*reg_ind] = 1;
                } else {
                    registers[*reg_ind] = 0;
                }
            }
        }
    }

    if go_down && registers[3] != orig / 26 {
        return false;
    }

    if depth >= 14 {
        if registers[3] == 0 {
            print!("found: {}", registers[0]);
            return true;
        } else {
            return false;
        }
    }
    //fn process_one(hist: &mut Vec<i64>, input: i64, registers: [i64; 4], instructs: &Vec<(Cmd, usize, char, i64)>) -> i64 {
    let mut found = false;
    for next_input in 1..=9 {
        let cfound = process_one(depth, next_input, &mut registers.clone(), instructs);
        if cfound {
            found = true;
            print!("{}", registers[0]);
            break;
        }
    }

    return found;
}
fn process(input: i64, instructs: &[(Cmd, usize, char, i64)]) -> i64 {
    let mut registers: [i64; 4] = [0; 4];
    let mut input = input_conv(input);
    if input.iter().any(|i| *i == 0) {
        return i64::MAX;
    }

    instructs.iter().for_each(|(cmd, reg_ind, val2c, val2i)| {
        let right = match val2c {
            'w' => registers[0],
            'x' => registers[1],
            'y' => registers[2],
            'z' => registers[3],
            _ => *val2i, // '_' in reality
        };
        //print!(">>>>>: {:?}", (cmd, reg_ind, val2c, val2i));
        match cmd {
            Cmd::Inp => {
                //println!("---- input ----");
                let inp = input.pop().unwrap();
                registers[*reg_ind] = inp;
            }
            Cmd::Add => {
                let left = registers[*reg_ind];
                registers[*reg_ind] = left + right;
            }
            Cmd::Mul => {
                let left = registers[*reg_ind];
                registers[*reg_ind] = left * right;
            }
            Cmd::Div => {
                let left = registers[*reg_ind];
                registers[*reg_ind] = left / right;
            }
            Cmd::Mod => {
                let left = registers[*reg_ind];
                registers[*reg_ind] = left % right;
            }
            Cmd::Eql => {
                let left = registers[*reg_ind];
                if left == right {
                    registers[*reg_ind] = 1;
                } else {
                    registers[*reg_ind] = 0;
                }
            }
        }
        //println!("\t\t>>: {:?}", registers);
        //print!(">> {:?}\t\t", (cmd, reg_ind, val2c, val2i));
        //println!("={:?}", registers,);
    });

    registers[3]
}

fn reg_index(c: &char) -> usize {
    (*c as u8 - 'w' as u8) as usize
}

