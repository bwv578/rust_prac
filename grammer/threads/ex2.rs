use std::thread;
use std::time::Duration;
use thread::JoinHandle;

fn main() {
    // https://doc.rust-kr.org/ch16-01-threads.html#join-%ED%95%B8%EB%93%A4%EC%9D%84-%EC%82%AC%EC%9A%A9%ED%95%98%EC%97%AC-%EB%AA%A8%EB%93%A0-%EC%8A%A4%EB%A0%88%EB%93%9C%EA%B0%80-%EB%81%9D%EB%82%A0-%EB%95%8C%EA%B9%8C%EC%A7%80-%EA%B8%B0%EB%8B%A4%EB%A6%AC%EA%B8%B0

    let handle:JoinHandle<i32> = thread::spawn(|| {
        let mut sum: i32 = 0;
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
            sum+=i;
        }
        return sum;
    });

    // [1]
    // 현재 스레드(메인)에서 파생된 스레드 => JoinHandle<T> 반환 (스레드 핸들)
    // T는 클로저의 리턴타입
    // 클로저 리턴이 없으면 ? => JoinHandle<()>
    // 단순히 파생된 스레드의 반환값을 변수에 담는다 해서 부모스레드가 블로킹되지는 않음

    // [3]
    //println!("sum is {}", handle.join().unwrap());
    // 파생스레드의 조인시점도 중요하다.
    // 주석처리한 위 조인명령을 주석해제하고 [2] 의 코드를 지우면 메인스레드 다음코드 시작 전에 파생스레드 실행 완료를 기다리게 된다.
    // => 스레드 실행순서 보장 효과
    // => 조인 메소드가 부모스레드 위에서 당장 파생스레드의 실행 결과를 내뱉게끔 동작하는것을 생각하면 자연스러움

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // [2]
    println!("sum is {}", handle.join().unwrap());
    // 부모스레드에서 파생스레드에 대해 join() 메소드를 호출하면 파생스레드의 리턴값을 반환함
    // => 부모스레드(메인) 블로킹됨. 이제 ex1.rs와 다르게 메인과 파생스레드 모두 전체 코드 실행이 보장된다.

}