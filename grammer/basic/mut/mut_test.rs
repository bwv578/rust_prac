
fn main(){
    println!("rust test");
    
    let mut a = 10;
    println!("a = {}", a);

    a = 20;
    println!("a has changed : {}", a);

    // 타입명시
    let mut str:String;
    str = "this is string".to_string();
    println!("String str = {}", str);

    str = "string has changed.".to_string();
    println!("String str = {}", str);
    println!("-----------------------------------");

    let mut immutable_str:&str;
    immutable_str = "this is immut str."; 
    println!("this &str is {}", immutable_str);

    immutable_str = "ohhohohoh immutable_str has changed";
    println!("&str type can be changed. {}", immutable_str);

}
