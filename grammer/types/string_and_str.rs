fn main(){

    // !!! str타입으로 변수선언 불가
    // 왜?ㅅㅂ
    // => 스택변수는 컴파일타임에 크기 미리 정해져야됨
    // => str 자체는 길이 미정(런타임에 바뀔수 있음)
    // => 컴파일러가 스택에 사이즈 얼마할당해야할지 모름 
    // => 정확히는 str 타입에 Sized 트레이트가 구현되어있지 않음

    //let str1:str = "this is str";
    // 하드코딩된 위 문자열은 사실 값이 아니고 저 문자열 상수의 참조를 나타냄
    // => 대신 &str 변수에 할당 가능
    
    //let str1:str = *"뭐시기문자열";
    // => 이것도 당연히 안됨. 
    // 타입이 맞아도 어차피 컴파일타임에 길이 확정 불가(하다고 컴파일러가
    // 판단함ㅄ) 근본적으로 변수로 선언해서 값 할당이 불가.

    let mut str1:&str = "문자열1";
    println!("str1:{}",str1);
    //*str1 = "변경시도1"; => 당연히 타입 안맞아서 안됨
    //*str1 = *"변경시도2"; => 참조가 가리키는 상수를 변경하는 시도이므로 불가능
    str1 = "변경시도3"; // 참조 자체를 바꿔서야 겨우 가능
    println!("str1:{}",str1);

    // String은str의 단순 참조 대신 문자열의 포인터, 길이, 할당용량을 가진 구조체임
    let mut string1:String = String::from("문자열");
    // => from메소드가 문자열의 참조를 받아서 구조체 생성, 반환
    println!("string1:{}", string1);

    // String이 구조체면 변수 필드 접근 되나?
    //println!("fields:{:?}, {}, {}", string1.ptr, string1.len, string1.capacity);
    // => 될리가 없다. 접근 못하게 private 로 돼있음. 제공되는 메소드로 안전하게 간접 접근 필요
    
    string1 = String::from("string 변경");
    println!("string1 변경시도:{}", string1);
}
