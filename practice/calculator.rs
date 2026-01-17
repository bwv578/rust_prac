fn main(){
    let input: Vec<String> = std::env::args().collect();
    calculate(&input[1]);
}

fn calculate(expression:&String) -> String {
    let mut args:Vec<String> = split(expression);
    //println!("CALC split result:");
    //println!("{:?}", args);

    // 1. solve (...)
    let mut i:usize = 0;
    while i < args.len() {
        let arg = &mut args[i];
        
        match arg.chars().next() {
            Some(c) => {
                if c == '?'{
                    *arg = calculate( &(arg.chars().skip(1).collect()) );
                }
            }
            None => {}
        }

        i += 2;
    }

    // 2. solve * /
    // 3. solve + -

    println!("res:{:?}", &args);
    return String::from("some result");
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
