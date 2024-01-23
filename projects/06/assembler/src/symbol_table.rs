use std::collections::HashMap;


pub struct SymbolTable{
    table:HashMap<String,usize>,
    count:usize,
    last_index:usize
}

impl SymbolTable{
    pub fn new() -> SymbolTable{
        let mut symbol_table = SymbolTable{
            table:HashMap::new(),
            count:0,
            last_index:15,
        };
        for i in 0..16{
            symbol_table.table.insert(format!("R{}",i),i);
        }
        symbol_table.table.insert("SP".to_string(),0);
        symbol_table.table.insert("LCL".to_string(),1);
        symbol_table.table.insert("ARG".to_string(),2);
        symbol_table.table.insert("THIS".to_string(),3);
        symbol_table.table.insert("THAT".to_string(),4);
        symbol_table.table.insert("SCREEN".to_string(),16384);
        symbol_table.table.insert("KBD".to_string(),24576);
        return symbol_table;
    }
    pub fn add_entryL(&mut self,symbol:&str,address:usize){
        if self.contains(symbol) == false{
            self.table.insert(symbol.to_string(),address - self.count);
            self.count += 1;
        }
    }
    pub fn add_entry(&mut self,symbol:&str){
        if self.contains(symbol) == false{
            self.last_index += 1;
            self.table.insert(symbol.to_string(),self.last_index);
        }
    }
    pub fn contains(&self,symbol:&str) -> bool{
        if self.table.contains_key(symbol){
            return true;
        }else{
            return false;
        }
    }
    pub fn get_address(&self, symbol:&str) -> usize{
        if let Some(&address) = self.table.get(symbol) {
            return address;
        } else {
            panic!("The value({}) you are looking for does not exist in the hash table.",symbol);
        }
    }
}