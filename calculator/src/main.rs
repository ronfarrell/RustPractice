use std::io::{self, BufRead};

mod inputhandler;




fn main() {
    let stdin = io::stdin();
    
    loop {
        println!();
        let reader = stdin.lock(); // re-lock fresh each loop
        let should_exit = run_calculator(reader);
        if should_exit {
            break;
        }
    }
}

pub fn run_calculator<R: BufRead>(mut reader: R) -> bool{
    let mut operation = String::new();
    let mut n1 = String::new();
    let mut n2 = String::new();
    let mut exit_result = String::new();

    println!("please enter what operation you would like (+, -, /, %, *)");
    reader.read_line(&mut operation).expect("failed to read line");

    println!("please enter your first number");
    reader.read_line(&mut n1).expect("failed to read line");

    println!("please enter your second number");
    reader.read_line(&mut n2).expect("failed to read line");

    let operation = operation.trim();
    let n1 = n1.trim();
    let n2 = n2.trim();

    println!();
    match crate::inputhandler::calculate(operation, n1, n2) {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {}", e),
    }

    println!();
    println!("would you like to exit? (yes/no)");
    reader.read_line(&mut exit_result).expect("failed to read line");
    let res = crate::inputhandler::read_exit(&exit_result.trim());

    if res {
        println!("Exiting...");
        return true;
    }
    
    return false;
}





#[cfg(test)]
mod tests {
    use super::inputhandler::{equation_check, read_exit, OperationCheck, calculate};
    use super::run_calculator;
        use std::io::Cursor;
    // use mock_io::MockIo;

    #[test]
    fn test_read_exit_yes() {
        assert!(read_exit("yes"));
        assert!(read_exit("y"));
        assert!(!read_exit("no"));
    }

    #[test]
    fn test_valid_equations() {
        let add = equation_check("+", "10", "2");
        let sub = equation_check("-", "10", "2");
        let div = equation_check("/", "10", "2");
        let modu = equation_check("%", "10", "2");
        let mult = equation_check("*", "10", "2");
        assert_eq!(add, OperationCheck::Valid);
        assert_eq!(sub, OperationCheck::Valid);
        assert_eq!(div, OperationCheck::Valid);
        assert_eq!(modu, OperationCheck::Valid);
        assert_eq!(mult, OperationCheck::Valid);
    }

    #[test]
    fn test_valid_equations_calculate() {
        let add = calculate("+", "10", "2");
        let sub = calculate("-", "10", "2");
        let div = calculate("/", "10", "2");
        let modu = calculate("%", "10", "2");
        let mult = calculate("*", "10", "2");
        assert_eq!(add.unwrap(), 12);
        assert_eq!(sub.unwrap(), 8);
        assert_eq!(div.unwrap(), 5);
        assert_eq!(modu.unwrap(), 0);
        assert_eq!(mult.unwrap(), 20);
    }

    #[test]
    fn test_invalid_equations_calculate() {
        let inv_op = calculate("op", "10", "2");
        let inv_denom = calculate("/", "10", "0");
        let inv_num1 = calculate("+", "ten", "2");
        let inv_num2 = calculate("%", "10", "two");
        let mod_by_zero = calculate("%", "10", "0");

        assert_eq!(inv_op, Err("Invalid Operator! Must use +, -, *, %, /"));
        assert_eq!(inv_denom, Err("Invalid Denom! Do not divide by 0"));
        assert_eq!(inv_num1, Err("Invalid Num 1! Your first number must be an i32"));
        assert_eq!(inv_num2, Err("Invalid Num 2! Your second number must be an i32"));
        assert_eq!(mod_by_zero, Err("Invalid Mod! You cannot mod by 0"));
    }
    

    #[test]
    fn test_invalid_operator() {
        let result = equation_check("invalid", "10", "2");
        assert_eq!(result, OperationCheck::InvalidOperator);
    }

    #[test]
    fn test_invalid_num1() {
        let result = equation_check("+", "ten", "2");
        assert_eq!(result, OperationCheck::InvalidNum1);
    }

    #[test]
    fn test_invalid_num2() {
        let result = equation_check("+", "10", "two");
        assert_eq!(result, OperationCheck::InvalidNum2);
    }

    #[test]
    fn test_divide_by_zero() {
        let result = equation_check("/", "10", "0");
        assert_eq!(result, OperationCheck::InvalidDenom);
    }

        
    
        #[test]
        fn test_run_calculator_mock_input() {
            let input = b"+\n10\n2\nyes\n";
            let reader = Cursor::new(&input[..]);
    
            // You won't capture the printed output unless you use a crate like `assert_cmd` or `duct`,
            // but at least this test verifies it runs without panic
            run_calculator(reader);
        }
    }
    


