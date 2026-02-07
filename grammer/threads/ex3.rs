use std::thread;

fn main() {
    // https://doc.rust-kr.org/ch16-01-threads.html#join-%ED%95%B8%EB%93%A4%EC%9D%84-%EC%82%AC%EC%9A%A9%ED%95%98%EC%97%AC-%EB%AA%A8%EB%93%A0-%EC%8A%A4%EB%A0%88%EB%93%9C%EA%B0%80-%EB%81%9D%EB%82%A0-%EB%95%8C%EA%B9%8C%EC%A7%80-%EA%B8%B0%EB%8B%A4%EB%A6%AC%EA%B8%B0

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // 클로저 서명에 move 명시가 없다면 이 코드는 오류가 난다. 왜냐
    // => 클로저에서 v 캡처방식을 추론 => 참조만 필요하겠구만
    // => 참조를 쓰려니 이후 부모스레드 코드에서 곧바로 v가 드랍되는 시나리오 존재
    // => 안전하지 않다. 컴파일 실패
    // 일례로 이 위치에 drop(v)를 넣는다면 메인과 파생스레드가 동시에 실행되는 도중 v가 사라질수있음
    // 그래서 소유권 자체를 아예 클로저 내부로 가져오게끔 move를 쓰면 해결된다.

    handle.join().unwrap();
}