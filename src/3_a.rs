// 구조체 정의
#[derive(Debug)]
struct User {
    id: u32,
    active: bool,
}

// 열거형 정의
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    // 패턴 매칭 (match)
    match msg {
        Message::Quit => {
            println!("Quit 메시지 수신: 프로그램을 종료합니다.");
        }
        Message::Move { x, y } => {
            println!("Move 메시지 수신: 좌표 ({}, {})로 이동합니다.", x, y);
        }
        Message::Write(text) => {
            println!("Write 메시지 수신: \"{}\" 내용을 작성합니다.", text);
        }
    }
}

fn main() {

    let user1 = User { id: 1, active: true };
    println!("유저 정보: {:?}", user1);

    process_message(Message::Move { x: 10, y: 20 });
    process_message(Message::Write(String::from("헬로 러스트")));

    println!("=====================================================");

    let safe_value: Option<i32> = Some(15);
    // let safe_value: Option<i32> = None;

    match safe_value {
        Some(num) => {
            println!("Some(값 있음): 값은 {}이고, 5를 더하면 {}입니다.", num, num + 5);
        },
        None => {
            println!("None(값 없음): 연산을 수행할 수 없습니다.");
        }
    }

    println!("=====================================================");
    
    // 클로저 정의: 두 정수를 받아 합을 반환합니다.
    let sum_closure = |x: i32, y: i32| -> i32 {
        x + y
    };

    // 클로저 사용
    let a = 5;
    let b = 10;
    let result = sum_closure(a, b);

    println!("{} + {} = {}", a, b, result); // 출력: 5 + 10 = 15

    // 환경 캡처를 보여주는 클로저: 외부 변수를 캡처하여 사용
    let factor = 3;
    let multiplier = |n| n * factor; // 'factor' 변수를 캡처

    let value = 7;
    let multiplied_result = multiplier(value);

    println!("{} * {} = {}", value, factor, multiplied_result); // 출력: 7 * 3 = 21


}