// 1. 트레이트 정의
pub trait Summary {
    // 기본 구현이 없는 메서드
    fn summarize(&self) -> String;

    // 기본 구현이 있는 메서드
    fn read_more(&self) -> String {
        String::from("(더 읽어보기...)")
    }
}

// 2. 트레이트 구현 (구조체)
pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    // read_more는 기본 구현을 사용함.
}

// 제네릭
fn identity_picker<T>(x: T, y: T) -> T {
    x
}

fn main() {
    let tweet = Tweet {
        username: String::from("러스트_공식"),
        content: String::from("트레이트를 배웠습니다!"),
    };

    println!("요약: {}", tweet.summarize()); // 구현된 메서드 사용
    
    // 1. i32 타입으로 함수 사용
    let num = identity_picker(10, 20);
    println!("숫자 선택: {}", num); // 출력: 숫자 선택: 10

    // 2. String 타입으로 함수 사용
    let text = identity_picker(String::from("Hello"), String::from("World"));
    println!("문자열 선택: {}", text); // 출력: 문자열 선택: Hello
}