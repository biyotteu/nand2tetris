use std::fs::read_to_string;
use regex::Regex;

#[derive(PartialEq)]
pub enum InstructionType{
    ACOMMAND,
    CCOMMAND,
    LCOMMAND
}

pub struct Paser{
    lines:Vec<String>,
    pc:usize,
}

impl Paser{
    pub fn new() -> Paser{
        Paser{
            lines:Vec::new(),
            pc:0,
        }
    }
    pub fn read_file(&mut self, file_name: &str){
        // read file and remove space
        let file_content = read_to_string(file_name).expect("Failed to read input").replace(" ", "");
        
        // remove comments
        let pattern1 = Regex::new(r"//.*\n").unwrap();
        let pattern2 = Regex::new(r"/\*.*\*/").unwrap();
        let content1 = pattern1.replace_all(&file_content,"");
        let content2 = pattern2.replace_all(&content1,"");

        for line in content2.lines(){
            if line.starts_with("\n") || line.is_empty(){
                continue;
            }
            self.lines.push(line.to_string());
        }
    }

    pub fn has_more_lines(&self) -> bool{
        self.pc < self.lines.len() as usize
    }
    
    pub fn advance(&mut self){
        if self.has_more_lines(){
            self.pc += 1;
        }
    }

    pub fn instruction_type(&self) -> InstructionType{
        if self.lines[self.pc].starts_with("@"){
            return InstructionType::ACOMMAND;
        }
        else if self.lines[self.pc].starts_with("("){
            return InstructionType::LCOMMAND;
        }
        else{
            return InstructionType::CCOMMAND;
        }
    }
    pub fn symbol(&self)->String{
        if self.instruction_type() == InstructionType::ACOMMAND{
            return self.lines[self.pc].split("@").collect::<Vec<_>>()[1].to_string();
        }
        else if self.instruction_type() == InstructionType::LCOMMAND{
            return self.lines[self.pc].split("(").collect::<Vec<_>>()[1].split(")").collect::<Vec<_>>()[0].to_string();
        }
        else{
            panic!("This instruction is not A or L command");
        }
    }
    pub fn comp(&self)->String{
        if self.instruction_type() == InstructionType::CCOMMAND{
            if self.lines[self.pc].contains("="){
                let tmp = self.lines[self.pc].split("=").collect::<Vec<_>>()[1];
                if tmp.contains(";"){
                    return tmp.split(";").collect::<Vec<_>>()[0].to_string();
                }
                return tmp.to_string();
            }
            else if self.lines[self.pc].contains(";"){
                return self.lines[self.pc].split(";").collect::<Vec<_>>()[0].to_string();
            }else{
                return "".to_string();
            }
        }
        else{
            panic!("This instruction is not C command");
        }
    }
    pub fn dest(&self)->String{
        if self.instruction_type() == InstructionType::CCOMMAND{
            if self.lines[self.pc].contains("="){
                return self.lines[self.pc].split("=").collect::<Vec<_>>()[0].to_string();
            }
            else{
                return "".to_string();
            }
        }
        else{
            panic!("This instruction is not C command");
        }
    }
    pub fn jump(&self)->String{
        if self.instruction_type() == InstructionType::CCOMMAND{
            if self.lines[self.pc].contains(";"){
                return self.lines[self.pc].split(";").collect::<Vec<_>>()[1].to_string();
            }else{
                return "".to_string();
            }
        }else{
            panic!("This instruction is not C command");
        }
    }
    pub fn reset_pc(&mut self){
        self.pc = 0;
    }
    pub fn get_pc(&self)->usize{
        self.pc
    }
    // pub fn println(&self){
    //     println!("##{}##",self.lines[self.pc]);
    // }
}

impl Drop for Paser {
    fn drop(&mut self) {
        self.lines.clear();
    }
}