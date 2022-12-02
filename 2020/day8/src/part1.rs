mod instruction;
mod instructions_loader;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut is = instructions_loader::get_instructions1();
    let mut position: i32 = 0;
    let mut acc: i32 = 0;
    loop {
        let e = &is[position as usize];
        if e.1 {
            break;
        }
        match e.0 {
            instruction::Instruction1::Acc(v) => acc += v,
            instruction::Instruction1::Jmp(v) => {
                position += v;
                continue;
            },
            _ => ()
        }
        is[position as usize].1 = true;
        position += 1;
    }
    println!("Acc: {}", acc);
    println!("Elapsed: {}ms", now.elapsed().as_millis());
}
