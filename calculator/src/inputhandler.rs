mod algorithms;  // Declare the algorithms module inside inputhandler

pub fn calculate(op: &str, n1: &str, n2: &str){

    println!("your equation was {} {} {}", n1, op, n2);
    
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

pub fn read_exit(res: &str) -> bool{

    match res {

        "yes" | "y" => true,
        _ => false,

    }

}