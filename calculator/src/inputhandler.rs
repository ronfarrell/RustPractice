mod algorithms;  // Declare the algorithms module inside inputhandler

//allows to check enums with == and !=
#[derive(PartialEq)]
enum OperationCheck {
    InvalidOperator,
    InvalidDenom,
    InvalidNum1,
    InvalidNum2,
    ModByZero,
    Valid,
}

pub fn calculate(op: &str, n1: &str, n2: &str){

    print!("your equation was {} {} {} = ", n1, op, n2);
    

    let check_result = equation_check(op, n1, n2);

    if check_result == OperationCheck::Valid {
        
        let n1: i32 = n1.trim().parse().expect("Failed to parse integer");
        let n2: i32 = n2.trim().parse().expect("Failed to parse integer");

        // like a case switch
        match op {

            "+" => algorithms::add(n1, n2),
            "-" => algorithms::subtract(n1, n2),
            "%" => algorithms::modular(n1, n2),
            "*" => algorithms::multiply(n1, n2),
            "/" => algorithms::divide(n1, n2),


            _ => println!("error")

        }
    }
    else{

        match check_result {
            OperationCheck::InvalidOperator => println!("Invalid Operator! Must use +, -, *, %, /"),
            OperationCheck::InvalidDenom => println!("Invalid Denom! Do not divide by 0"),
            OperationCheck::InvalidNum1 => println!("Invalid Num 1! Your first number must be an i32"),
            OperationCheck::InvalidNum2 => println!("Invalid Num 2! Your second number must be an i32"),
            OperationCheck::ModByZero => println!("Invalid Mod! You cannot mod by 0"),
            _ =>  println!("Unexpected Error"),
        }

    }


}

fn equation_check(op: &str, n1: &str, n2: &str) -> OperationCheck{

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