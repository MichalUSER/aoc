mod instruction;
mod instructions_loader;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut is = instructions_loader::get_instructions();
    let is_len = is.len() as i32;
    let mut position: i32 = 0;
    let mut acc: i32 = 0;
    let mut found = false;
    loop {
        if position == is_len {
            break;
        }
        let e = &is[position as usize];
        let mut change: Option<instruction::Instruction> = None;
        match e.0 {
            instruction::Instruction::Acc(v) => {
                acc += v;
                position += 1;
                println!("Acc: {}", v);
            },
            instruction::Instruction::Jmp(v) => {
                if !found {
                    let next = &is[(position + v) as usize];
                    if next.1 {
                        change = Some(instruction::Instruction::Nop(v));
                        found = true;
                        println!("changing jmp to nop");
                        position += 1;
                    } else {
                        for i in position..position+v {
                            is[i as usize].1 = true;
                        }
                        position += v;
                    }
                } else {
                    position += v;
                }
            },
            instruction::Instruction::Nop(v) => {
                if v != 0 {
                    let next = &is[(position + v) as usize];
                    if !next.1 {
                        change = Some(instruction::Instruction::Jmp(v));
                        found = true;
                        println!("changed nop to acc");
                    }
                }
                position += 1;
            }
        }
        if let Some(v) = change {
            is[position as usize].0 = v;
        }
    }
    println!("Acc full: {}", acc);
    println!("Elapsed: {}ms", now.elapsed().as_millis());
}
