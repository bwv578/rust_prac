fn main(){
    
    println!("start");

    let x1= 10;
    let y1 = 15;
    
    println!("x1:{x1} y1:{y1}");

    let x2 = 20;
    let y2 = 25;

    println!("x2:{x2} y2:{y2}");

    let sum1 = x1 + y1;
    let sum2 = x2 + y2;
    
    println!("sum1:{sum1} sum2:{sum2}");
    println!("end");

    let result = test_loop(sum1);
    println!("result: {result}");

}

fn test_loop(ceil: i32) -> i32 {

    for index in 1..=ceil{
        println!("for.. {index}");
    }

    return ceil * 2;
}