use std::fs;

#[derive(Debug)]
pub struct Operation {
    pub operator: char,
    pub operands: Vec<String>,
    pub size: usize
}

impl Operation {
    pub fn new(operator: char) -> Self {
        Operation {
            operator,
            operands: Vec::new(),
            size: 0
        }
    }
    pub fn add_operand(&mut self, operand: String) {
        self.operands.push(operand);
    }

    pub fn evaluate(&self, horizontal: bool) -> u64 {
        if horizontal {
            self.evaluate_horizontal()
        } else {
            self.evaluate_vertical()
        }
    }

    fn evaluate_horizontal(&self) -> u64 {
        match self.operator {
            '+' => self.operands.iter().map(|s| s.trim().parse::<u64>().unwrap()).sum(),
            '*' => self.operands.iter().map(|s| s.trim().parse::<u64>().unwrap()).product(),
            _ => 0,
        }
    }

    fn evaluate_vertical(&self) -> u64 {
        let mut column_values: Vec<u64> = Vec::new();
        for i in 0..self.size {
            let mut column_value = String::new();
            for operand in self.operands.iter() {
                let c = operand.chars().nth(i).unwrap();
                column_value.push(c);
            }
            column_values.push(column_value.trim().parse::<u64>().unwrap());
        }
        match self.operator {
            '+' =>  column_values.iter().sum(),
            '*' => column_values.iter().product(),
            _ => 0,
        }
    }
}


pub fn day_6() {
    let contents = fs::read_to_string("./src/day_6/input.txt").expect("Could not read file");
    let mut operations: Vec<Operation> = Vec::new();
    let last = contents.lines().last().unwrap().to_string();
    let mut count = 0;
    for c in last.chars() {
        if c == '+' || c == '*' {
            let op = Operation::new(c);
            if let Some(op) = operations.last_mut() {
                op.size = count;
            }
            operations.push(op);
            count = 0;
        }
        else {
            count += 1;
        }
    }
    
    if let Some(op) = operations.last_mut() {
        op.size = count + 1; // +1 to account for missing last space
    }

    contents.lines().take(contents.lines().count() - 1).for_each(|line| {
        
        let mut start = 0;
        
        for operation in operations.iter_mut() {
            let str = &line[start..start+operation.size];
            operation.add_operand(str.to_string());
            start += operation.size + 1; // +1 to skip space
        }
    });

    eprintln!("Operations: {:?}", operations);
    operations.iter().for_each(|op| {
        println!("Operation {:?} horizontal evaluation: {}", op, op.evaluate(false));
    });

    let total = operations.iter().fold(0, |acc, op| acc + op.evaluate(false));
    println!("Total evaluation: {}", total);
}
