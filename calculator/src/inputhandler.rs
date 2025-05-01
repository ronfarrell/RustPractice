mod algorithms;  // Declare the algorithms module inside inputhandler

//allows to check enums with == and !=
#[derive(Debug, PartialEq)]
pub enum OperationCheck {
    InvalidOperator,
    InvalidDenom,
    InvalidNum1,
    InvalidNum2,
    ModByZero,
    Valid,
}

pub fn calculate(op: &str, n1: &str, n2: &str) -> Result<i32, &'static str> {
    print!("your equation was {} {} {} = ", n1, op, n2);

    let check_result = equation_check(op, n1, n2);

    if check_result == OperationCheck::Valid {
        let n1: i32 = n1.trim().parse().unwrap();
        let n2: i32 = n2.trim().parse().unwrap();


        return match op {
            "+" => Ok(algorithms::add(n1, n2)),
            "-" => Ok(algorithms::subtract(n1, n2)),
            "%" => Ok(algorithms::modular(n1, n2)),
            "*" => Ok(algorithms::multiply(n1, n2)),
            _ => Ok(algorithms::divide(n1, n2)),
            // _ => Err("unexpected"),
        };
    }

    // Error mapping
    let msg = match check_result {
        OperationCheck::InvalidOperator => "Invalid Operator! Must use +, -, *, %, /",
        OperationCheck::InvalidDenom => "Invalid Denom! Do not divide by 0",
        OperationCheck::InvalidNum1 => "Invalid Num 1! Your first number must be an i32",
        OperationCheck::InvalidNum2 => "Invalid Num 2! Your second number must be an i32",
        OperationCheck::ModByZero => "Invalid Mod! You cannot mod by 0",
        _ => "Unexpected Error",
    };

    Err(msg)
}


pub fn equation_check(op: &str, n1: &str, n2: &str) -> OperationCheck{

    //operator check
    if op != "+" && op != "-" && op != "%" && op != "*" && op != "/" {
       return  OperationCheck::InvalidOperator
    }

    // Check if n1 is a valid number
    if n1.parse::<i32>().is_err() {
        return OperationCheck::InvalidNum1;
    }

    // Check if n2 is a valid number
    if n2.parse::<i32>().is_err() {
        return OperationCheck::InvalidNum2;
    }

    let denom: i32 =  n2.parse().unwrap();

    //check for dividing by 0
    if op == "/" && denom == 0{
        return OperationCheck::InvalidDenom;
    }

    //mod by 0
    if op == "%" && denom == 0{
        return OperationCheck::ModByZero;
    }
    

    OperationCheck::Valid

}

pub fn read_exit(res: &str) -> bool{

    match res {

        "yes" | "y" => true,
        _ => false,

    }

}

