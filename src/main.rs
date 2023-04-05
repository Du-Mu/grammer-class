use std::io;
use std::fmt;
struct Grammer {
    name: String,
    production_num: usize,
    v_n: Vec<char>,
    v_t: Vec<char>,
    v_n_str: String,
    production: Vec<String>,
    start_symbol: char,
    grammer_type: i32
}
impl Default for Grammer {
    fn default() -> Self {
        Grammer {
            name: "".to_string(),
            v_n: vec![],
            v_n_str: "".to_string(), 
            production_num: 0,
            production: vec![],
            start_symbol: ' ',
            grammer_type: 0,
            v_t: vec![]
        }
    }
}
impl Grammer {
    fn get_v_t(&self) -> Vec<char> {
        let mut v_t_vec = vec![];
        for i in &self.production {
            let mut split_parts = i.split("::=");
            let right_part = split_parts.nth(1);
            let target_parts = right_part.unwrap().chars();
            for j in target_parts {
                if j != '|' && !self.v_n.contains(&j)
                    { v_t_vec.push(j); }
            }
        }
    v_t_vec
    }
}
impl fmt::Display for Grammer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output_str = String::new();
        write!(f, "{}=({{{}}},{{",self.name,self.v_n_str).expect("Display Error");
        for i in self.v_t.iter() {
            output_str.push(*i);
            output_str.push(',');
        }
        if output_str.len() != 0 { output_str.pop(); }
        write!(f,"{}",output_str).expect("Display Error");
        write!(f,"}},P,{})\nP:",self.start_symbol).expect("Display Error");
        for i in self.production.iter() {
            write!(f, "\t{}\n", i).expect("Display Error");
        }
        write!(f, "Chomsky type {}\n", self.grammer_type)
    }
    
}

fn main() {
    let mut tmp_line = String::new();
    let mut grammer_1 = Grammer::default();

    println!("Please input the Grammer:");
    io::stdin()
        .read_line(&mut tmp_line)
        .expect("Failed to read Grammer");
    tmp_line.pop();
    grammer_1.name = tmp_line;

    println!("Please input the V_N, use ',' to separate:");
    let mut tmp_line = String::new();
    io::stdin()
        .read_line(&mut tmp_line)
        .expect("Failed to read V_N");
    
    grammer_1.production_num = tmp_line.len()/2;
    
    let mut index = 0;
    let mut v_n_chars = tmp_line.chars();

    println!("Please enter the production rules for the grammar:");
    while index < grammer_1.production_num {
        grammer_1.v_n.push(v_n_chars.next().unwrap());
        v_n_chars.next().unwrap();
        index+=1;
        let mut tmp_line_product = String::new();
        io::stdin()
            .read_line(&mut tmp_line_product)
            .expect("Failed to read V_N");
        tmp_line_product.pop();
        grammer_1.production.push(tmp_line_product);
    }
    tmp_line.pop();
    grammer_1.v_n_str = tmp_line;
    grammer_1.start_symbol = grammer_1.v_n[0];
    grammer_1.v_t = grammer_1.get_v_t();

    if check_if_type3(&grammer_1.production, &grammer_1.v_n) {
        grammer_1.grammer_type = 3;       
    } else if check_if_type2(&grammer_1.production){
        grammer_1.grammer_type = 2;
    } else if check_if_type1(&grammer_1.production) {
        grammer_1.grammer_type = 1;
    }

    println!("\nThe grammer info:");
    println!("{}",grammer_1);

}

fn check_if_type1(production_set: &Vec<String>) -> bool {

    for i in production_set {
        let mut split_parts = i.split("::=");
        let left_part = split_parts.next();
        let right_part = split_parts.next();
        if left_part.unwrap().len() > right_part.unwrap().len() {
            return false;
        }
    }
    true
}

fn check_if_type2(production_set: &Vec<String>) -> bool {
    let derive_str = String::from("::=");
    for i in production_set {
        if i[1..4] != derive_str {
            return  false;
        }
    }
    true
}

fn check_if_type3(production_set: &Vec<String>, v_n: &Vec<char>) -> bool {
    let mut is_left = -1;

    for i in production_set {
        let mut split_parts = i.split("::=");
        let left_part = split_parts.next();
        let right_part = split_parts.next();
        let target_parts = right_part.unwrap().split("|");
        if left_part.unwrap().len() > right_part.unwrap().len() {
            return false;
        }
        for j in target_parts {
            if j.len() == 1 && !v_n.contains(&j.chars().nth(0).unwrap()) {
                continue;
            } else if j.len() == 2 {
                if is_left == -1 {
                    is_left  = if  v_n.contains(&j.chars().nth(0).unwrap()) {1} else {0};
                }
                if is_left == 1 {
                    if !v_n.contains(&j.chars().nth(0).unwrap()) ||
                      v_n.contains(&j.chars().nth(1).unwrap())
                      {return false;}
                } else {
                    if v_n.contains(&j.chars().nth(0).unwrap()) ||
                      !v_n.contains(&j.chars().nth(1).unwrap())
                      {return false;}
                }
            }
        }    
    }
    true
}