fn main(){
    let input: Vec<String> = std::env::args().collect();
    println!("result: {}" , calculate(&input[1]));
}

fn calculate(expression:&String) -> String {
    let mut args:Vec<String> = split(expression);
    let mut i:usize = 1;

    // 1. operate * /
    while i < args.len() {
        match args[i].chars().next() {
            Some('*') => {
                let lhs:f64 = parse_token(&args[i-1]);
                let rhs:f64 = parse_token(&args[i+1]);
                args[i-1] = (lhs * rhs).to_string();
                args.remove(i+1);
                args.remove(i);
            }
            Some('/') => {
                let lhs:f64 = parse_token(&args[i-1]);
                let rhs:f64 = parse_token(&args[i+1]);
                args[i-1] = (lhs / rhs).to_string();
                args.remove(i+1);
                args.remove(i);
            }
            _ => {i += 2;}
        }
    }

    // 2. operate + -
    i = 1;
    while i < args.len() {
        match args[i].chars().next() {
            Some('+') => {
                let lhs:f64 = parse_token(&args[i-1]);
                let rhs:f64 = parse_token(&args[i+1]);
                args[i-1] = (lhs + rhs).to_string();
                args.remove(i+1);
                args.remove(i);
            }
            Some('-') => {
                let lhs:f64 = parse_token(&args[i-1]);
                let rhs:f64 = parse_token(&args[i+1]);
                args[i-1] = (lhs - rhs).to_string();
                args.remove(i+1);
                args.remove(i);
            }
            _ => {i += 2;}
        }
    }

    return args.remove(0);
}

fn parse_token(token:&String) -> f64 {
    if token.chars().next() == Some('?') {
        return calculate(
            &( token.chars().skip(1).collect() )
        ).parse::<f64>().unwrap();
    }
    return token.parse::<f64>().unwrap();
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
