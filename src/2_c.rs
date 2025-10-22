// 'a 라이프타임 파라미터를 사용하여, 
// 반환되는 참조자는 인수인 x, y 중 더 짧은 라이프타임('a)을 가지는 것과 동일하거나 더 짧음을 명시함.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
// fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let result;
    let string_a = String::from("aaaaaaaa");

    {
        let string_b = String::from("bbbb");
        result = longest(&string_a, &string_b); 
        println!("The longest is: {}", result); // 컴파일 에러 발생 지점 (만약 string_b를 반환했다면)
    }

    // 댕글링 위험 지점
    // result는 여전히 유효한 'main 함수' 스코프에 있지만,
    // 만약 result가 string_b를 참조했다면, string_b는 이미 소멸되어
    // result는 해제된 메모리(댕글링)를 가리킴
    // println!("The longest is: {}", result); // 컴파일 에러 발생 지점 (만약 string_b를 반환했다면)
}