fn main() {

    // 정적 타입은 선언시 항상 타입 명시 필요함.
    static STATIC_VAR:f32 = 20.0;
    println!("STATIC_VAR = {}", STATIC_VAR);

    // let 으로 변수 하나 선언 => f32 타입 사용하니까 자동으로 타입 추론
    let none_static_var = STATIC_VAR * 5.0;
    println!("none_static_var = {}", none_static_var);

    // let 으로 기존 변수와 동일한 이름으로 변수 선언 가능
    let none_static_var = STATIC_VAR * 7.0;
    println!("none_static_var has changed  = {}", none_static_var);

    // 재선언시 다른 타입으로 선언 가능
    let none_static_var: i32 = 50;
    println!("none_static_var has changed again  = {}", none_static_var);

    // 동일한 변수명은 mut 없이 재할당이 안될뿐 재선언은 가능
    //none_static_var = 30;
    println!("none_static_var can not be changed without redeclaration");

    // mut로 변수 하나 선언
    let mut integer_val:i32 = 999;
    println!("integer_val = {}", integer_val);

    //integer_val = 10.5; mut 여도 타입이 명시된 경우 타입은 못바꿈
    //integer_val:f32 = 10.5; 타입 다시 명시하면서 바꾸는것도 안됨
    let integer_val:f32 = 1000.5; // let 으로 아예 재선언 해버려야 타입 변경 가능
    println!("integer_val has changed = {}", integer_val);

    // 정적 타입 변수는 당연히 못바꿈
    //STATIC_VAR = 1.0;
    // static으로 선언된 경우 let 과 달리 재선언도 불가능
    //static STATIC_VAR = 1.0;
    println!("STATIC VAR = {}", STATIC_VAR);
}
