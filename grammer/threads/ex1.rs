use std::thread;
use std::time::Duration;

fn main() {
    // https://doc.rust-kr.org/ch16-01-threads.html#spawn%EC%9C%BC%EB%A1%9C-%EC%83%88%EB%A1%9C%EC%9A%B4-%EC%8A%A4%EB%A0%88%EB%93%9C-%EC%83%9D%EC%84%B1%ED%95%98%EA%B8%B0
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // 스폰된 스레드는 작업내용 남아있어도 메인스레드 종료되면 같이종료
}