use libafl::executors::command;
use unicorn_engine::Unicorn;
use unicorn_engine::RegisterARM;
use unicorn_engine::unicorn_const::{Arch, Mode, Permission};

use std::fs;
use serde_json::Value;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input filename (deprecated, use --input-file)
    #[arg(short = 'f', long = "filename", default_value = "input.json")]
    filename: String,

    /// Input file
    #[arg(short = 'i', long = "input-file", default_value = "input.json")]
    input_file: String,
}

// fn setup_unicon(){
//     let mut harness = Unicorn::new(Arch::ARM, Mode::THUMB).unwrap();
    
// }

// fn emulate(){
//     let arm_code: [u8; 6] = [
//         0x01, 0x20,  // movs r0, #1
//         0x02, 0x21,  // movs r1, #2
//         0x0A, 0x18,  // adds r2, r1, r0
//     ];
//     let mut harness = Unicorn::new(Arch::ARM, Mode::THUMB).unwrap();

//     // define constants for emulation
//     const BASE_ADDR:u64 = 0x10000 ;
//     const STACK_ADDR:u64 = 0x20000;
//     const MEM_SIZE:usize = 0x1000;

//     harness.mem_map(BASE_ADDR, MEM_SIZE, Permission::ALL)
//         .expect("Failed to map memory");

//     harness.mem_write(BASE_ADDR, &arm_code)
//         .expect("Failed to write code to memory");

//     harness.reg_write(RegisterARM::PC, BASE_ADDR | 1)
//         .expect("Failed to set PC register");
//     harness.reg_write(RegisterARM::CPSR, 0x20)
//         .expect("Failed to set CSPR register");
//     harness.reg_write(RegisterARM::SP, STACK_ADDR)
//         .expect("Failed to set SP register");

//     harness.add_code_hook(BASE_ADDR, BASE_ADDR + 0x200, |uc, address, size| {
//     println!("Executing 0x{:08x}, size: {}", address, size);
// }).unwrap();

//     println!("Length : {}", arm_code.len());

//     let pc_start = BASE_ADDR | 1;
//     harness.emu_start(pc_start, BASE_ADDR + arm_code.len() as u64, 0, 3)
//         .expect("Failed to start emulation");

//     let r0 = harness.reg_read(RegisterARM::R0).unwrap();
//     let r1 = harness.reg_read(RegisterARM::R1).unwrap();
//     let r2 = harness.reg_read(RegisterARM::R2).unwrap();
//     let pc = harness.reg_read(RegisterARM::PC).unwrap();
//     let cspr = harness.reg_read(RegisterARM::CPSR).unwrap();

//     println!("R0: {r0}");
//     println!("R1: {r1}");
//     println!("R2: {r2}");
//     println!("CSPR: {cspr}");
//     println!("PC: {pc}");
// }

fn main(){
    let args = Args::parse();
    let filename = args.filename;
    let input_file = args.input_file;

    // Read the JSON file
    let json_data = fs::read_to_string(filename).expect("Unable to read file");
    let json: Value = serde_json::from_str(&json_data).expect("JSON was not well-formatted");

    // Print the JSON data
    println!("JSON Data: {}", json);
}