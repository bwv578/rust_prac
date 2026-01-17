fn main(){
    let input: Vec<String> = std::env::args().collect();
    println!("result: {}" , calculate(&input[1]));
}

fn calculate(expression:&String) -> String {
    let mut args:Vec<String> = split(expression);

    // 1. solve (..exp..)
    let mut i:usize = 0;
    while i < args.len() {
        let arg = &mut args[i];
        
        match arg.chars().next() {
            Some('?') => {
                *arg = calculate( &(arg.chars().skip(1).collect()) );
            }
            _ => {}
        }

        i += 2;
    }

    // 2. operate * /
    i = 1;
    while i < args.len() {
        let operator = args[i].chars().next();
        let lhs:f64 = args[i-1].parse::<f64>().unwrap();
        let rhs:f64 = args[i+1].parse::<f64>().unwrap();

        match operator {
            Some('*') => {
                //let v = &mut args[i-1];
                //*v = (lhs * rhs).to_string();
                args[i-1] = (lhs * rhs).to_string();
                args.remove(i+1);
                args.remove(i);
            }
            Some('/') => {
                args[i-1] = (lhs / rhs).to_string();
                args.remove(i+1);
                args.remove(i);
            }
            _ => {i += 2;}
        }
    }

    // 3. operate + -
    i = 1;
    while i < args.len() {
        let operator = args[i].chars().next();
        let lhs:f64 = args[i-1].parse::<f64>().unwrap();
        let rhs:f64 = args[i+1].parse::<f64>().unwrap();

        match operator {
            Some('+') => {
                args[i-1] = (lhs + rhs).to_string();
                args.remove(i+1);
                args.remove(i);
            }
            Some('-') => {
                args[i-1] = (lhs - rhs).to_string();
                args.remove(i+1);
                args.remove(i);
            }
            _ => {i += 2;}
        }
    }

    return args.remove(0);
}

fn split(expression:&String) -> Vec<String> {
    let mut depth:i32 = 0;
    let mut args:Vec<String> = Vec::new();
    let mut num_bucket:String = String::from("");
    let mut exp_bucket:String = String::from("?");
    
    for c in expression.chars(){
        match c {
            ' ' => {continue;}

            '+'|'-'|'*'|'/' => {
                if depth>0 {
                    exp_bucket.push(c);
                }else {
                    if num_bucket.len()>0 {
                        args.push(num_bucket);
                    }
                    num_bucket = String::from("");
                    args.push(String::from(c));
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
                    exp_bucket = String::from("?");
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
    
    if num_bucket.len()>0 {
        args.push(num_bucket);
    }

    return args;
}
