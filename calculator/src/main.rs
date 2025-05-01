use std::io;

mod inputhandler;


fn main() {
    
    

    
    loop {

    let mut operation = String::new();
    let mut n1 = String::new();
    let mut n2 = String::new();
    
    let mut exit_result = String::new();

    //get operation
    println!("please enter what operation you would like (+, -, /, %, *)");
    io::stdin().read_line(&mut operation).expect("failed to read line");

    //get first number
    println!("please enter your first number");
    io::stdin().read_line(&mut n1).expect("failed to read line");

    //get second number
    println!("please enter your second number");
    io::stdin().read_line(&mut n2).expect("failed to read line");

    let operation = operation.trim();
    let n1 = n1.trim();
    let n2 = n2.trim();

    // println!("your equation was {} {} {}", n1, operation, n2);

    println!();
    inputhandler::calculate(operation, n1, n2);

    println!();
    println!("would you like to exit? (yes/no)");
    io::stdin().read_line(&mut exit_result).expect("failed to read line");
    let res = inputhandler::read_exit(&exit_result.trim());

    if res {
        break;
    }


    }

}



#[cfg(test)]
mod tests {
    use super::inputhandler::{equation_check, read_exit, OperationCheck};

    #[test]
    fn test_read_exit_yes() {
        assert!(read_exit("yes"));
        assert!(read_exit("y"));
        assert!(!read_exit("no"));
    }

    #[test]
    fn test_valid_equation() {
        let result = equation_check("+", "10", "2");
        assert_eq!(result, OperationCheck::Valid);
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
}
