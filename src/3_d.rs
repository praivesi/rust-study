use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 1. Arc<T>: 멀티 스레드에서 안전한 참조 카운팅을 제공하여 복수 소유권을 가능하게 함.
    // 2. Mutex<T>: 내부 값을 보호하며, 한 번에 하나의 스레드만 접근하도록 보장함.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    // 10개의 스레드를 생성하여 counter 값을 1씩 증가시킴
    for i in 0..10 {
        let counter_clone = Arc::clone(&counter); // Arc를 클론하여 소유권을 각 스레드에 복사

        // 러스트의 강력한 소유권 및 생명주기 규칙 덕분에 댕글링 참조 방지 (move)
        let handle = thread::spawn(move || {
            // Mutex의 락(Lock)을 획득 (락이 풀릴 때까지 현재 스레드를 블록시킴)
            let mut num = counter_clone.lock().unwrap();

            // 락이 유지되는 동안 안전하게 데이터 접근 및 변경
            *num += 1;
            
            // num의 스코프가 끝나면 락이 자동으로 해제됨 (Drop 트레이트 덕분)
            println!("스레드 {}가 현재 값 {}을 설정함", i, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 최종 결과 출력
    println!("최종 카운터 값: {}", *counter.lock().unwrap());
}