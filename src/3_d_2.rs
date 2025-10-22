use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // 새로운 채널을 생성
    // 'tx'는 송신자 (Transmitter), 'rx'는 수신자 (Receiver)
    let (tx, rx) = mpsc::channel();

    // 새 스레드를 생성
    thread::spawn(move || {
        let msg = String::from("안녕하세요, 채널을 통해 전달된 메시지입니다!");
        println!("송신 스레드: '{}' 메시지를 보내려고 합니다.", msg);

        match tx.send(msg) {
            Ok(_) => println!("송신 스레드: 메시지 전송 성공!"),
            Err(e) => eprintln!("송신 스레드: 메시지 전송 실패: {:?}", e),
        }
        
        // 참고: 메시지 전송 후, msg 값은 이동(move)되어 더 이상 이 스레드에서 사용할 수 없음
    });

    println!("메인 스레드: 메시지를 기다리는 중...");
    
    match rx.recv() {
        Ok(received) => {
            println!("메인 스레드: 메시지 수신 성공!");
            println!("수신된 메시지: **{}**", received);
        },
        Err(e) => {
            eprintln!("메인 스레드: 메시지 수신 실패: {:?}", e);
        }
    }
    
    // 메시지 수신 후 1초간 대기 (스레드 종료 전에 결과를 보기 위함)
    thread::sleep(Duration::from_secs(1));
    println!("프로그램 종료.");
}