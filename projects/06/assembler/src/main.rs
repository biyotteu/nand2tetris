use std::env;
use std::fs::File;
use std::io::Write;

mod parser;
use parser::*;
mod code;
use code::*;
mod symbol_table;
use symbol_table::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut symbol_table = SymbolTable::new();
    let mut parser = Paser::new();

    parser.read_file(&args[1].to_string());
    let output_path = format!("{}.hack",args[1].split(".").collect::<Vec<_>>()[0]);
    let mut output_file = File::create(output_path).expect("cannot create file");

    while parser.has_more_lines(){
        match parser.instruction_type() {
            InstructionType::LCOMMAND=>{
                if symbol_table.contains(&parser.symbol()) == false{
                    symbol_table.add_entryL(&parser.symbol(), parser.get_pc());
                }
            },
            _=>{}
        }
        parser.advance();
    }
    parser.reset_pc();
    while parser.has_more_lines(){
        match parser.instruction_type() {
            InstructionType::ACOMMAND=>{
                let mut address = 0;
                if parser.symbol().chars().next().unwrap().is_alphabetic(){
                    symbol_table.add_entry(&parser.symbol());
                    address = symbol_table.get_address(&parser.symbol());
                }else{
                    address = parser.symbol().parse().unwrap();
                }
                output_file.write_all(format!("0{:015b}\n",address).as_bytes()).expect("fail to write file");
            },
            InstructionType::CCOMMAND=>{
                output_file.write_all(format!("111{}{}{}\n",comp_to_binary(&parser.comp()),dest_to_binary(&parser.dest()),jump_to_binary(&parser.jump())).as_bytes()).expect("fail to write file");
            },
            _=>{}
        }
        parser.advance();
    }
    // Ok(())
}
