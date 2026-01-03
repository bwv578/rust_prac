fn main(){
    let input: Vec<String> = std::env::args().collect();
    println!("check args:");
    println!("{:?}", input);
    calculate(&input[1]);
    //print!("{} = {}", i);
}

fn calculate(expression:&String) {
    split(expression);
}

fn split(expression:&String) -> Vec<String>{
    let mut depth:i32 = 0;
    let mut args:Vec<String> = Vec::new();
    let mut num_bucket:String = "".to_string();
    let mut exp_bucket:String = "".to_string();
    
    for c in expression.chars(){
        match c {
            ' ' => {continue;}

            '+'|'-'|'*'|'/' => {
                if depth>0 {
                    exp_bucket.push(c);
                }else {
                    args.push(num_bucket);
                    num_bucket = "".to_string();
                    args.push(c.to_string());
                }
            }

            '(' => {
                if depth>0 {
                    exp_bucket.push(c);
                }
                depth += 1;
            }

            ')' => {
                depth -= 1;

                if depth>0 {
                    exp_bucket.push(c);
                }else {
                    args.push(exp_bucket);
                    exp_bucket = "".to_string();
                }
            }

            _ => {
                if depth>0 {
                    exp_bucket.push(c);
                }else {
                    num_bucket.push(c);
                }
            }
        }
    }

    println!("split result:");
    println!("{:?}", args);
    return args;
}

/*fn add(arg1:f64, arg2:f64) -> f64 {
    print!("add start");
}

fn subtract(arg1:f64, arg2:f64) -> f64 {
    print!("subtract start");
}

fn divide(arg1:f64, arg2:f64) -> f64 {
    print!("divied start");
}

fn multiply(arg1:f64, arg2:f64) -> f64 {
    print!("multiply start");
}*/
