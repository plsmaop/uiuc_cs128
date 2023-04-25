use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo
}

impl Operation {
    // [COMPLETE THIS FUNCTION]
    pub fn from_char(symbol: char) -> Option<Operation> {
        match symbol {
            '+' => Some(Operation::Add),
            '-' => Some(Operation::Subtract),
            '*' | 'x' => Some(Operation::Multiply),
            '/' => Some(Operation::Divide),
            '%' => Some(Operation::Modulo),
            _ => None,
        }
    }
}

// [HELPER FUNCTION - DO NOT EDIT]
pub fn get_equation_tuple(line: &String) -> (Option<&str>, Option<&str>) {
    let v: Vec<&str> = line.split(&['-', '+', 'x', '*', '/', '%'][..]).collect();
    
    (v.get(0).cloned(), v.get(1).cloned())
}

// [COMPLETE THIS HELPER FUNCTION]
pub fn parse_equation(line: &String) -> Result<(f32, f32, Operation), ()> {
    let (first_num_str_result, second_num_str_result) = get_equation_tuple(line);

    let first_num = first_num_str_result.ok_or_else(|| ())?.trim().parse::<f32>().map_err(|_| ())?;
    let second_num = second_num_str_result.ok_or_else(|| ())?.trim().parse::<f32>().map_err(|_| ())?;

    match line
    .chars()
    .find(|c| "+-x*/%".contains(*c)) 
    {
        None => Err(()),
        Some(operation_char) => Ok((first_num, second_num, Operation::from_char(operation_char).ok_or_else(|| ())?))
    }    
}

// [COMPLETE THIS FUNCTION]
pub fn solve(equation: &String) -> Result<f32, ()> {
    let result_tuple = parse_equation(equation).map_err(|_| ())?;

    match result_tuple.2 {
        Operation::Add => Ok(result_tuple.0 + result_tuple.1),
        Operation::Subtract => Ok(result_tuple.0 - result_tuple.1),
        Operation::Multiply => Ok(result_tuple.0 * result_tuple.1),
        Operation::Divide => Ok(result_tuple.0 / result_tuple.1),
        Operation::Modulo => Ok(result_tuple.0 % result_tuple.1),
    }
}

// [COMPLETE THIS FUNCTION]
pub fn solve_all(file_path: &str) -> Result<f32, ()> {
    BufReader::new(File::open(file_path).map_err(|_| ())?).lines().fold(Ok(0.0), |acc, line| {
        acc.and_then(|cur| {
            match line {
                Err(_) => Ok(cur),
                Ok(line_str) => Ok(solve(&line_str).unwrap_or_default() + cur)
            }
        })
    })
}
