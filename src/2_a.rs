fn main() {
    let s1 = String::from("소유권_테스트"); // s1이 이 값의 소유자가 됨

    // s2 = s1: 소유권이 s1에서 s2로 이동(move).
    // s1은 더 이상 유효하지 않으므로 사용하려 하면 컴파일 에러 발생.
    let s2 = s1; 

    println!("s2: {}", s2); 
    // println!("s1: {}", s1); // <- 컴파일 에러 발생 (use of moved value)
    
    // main 함수 종료 시, s2의 스코프가 끝나고 s2가 소유한 메모리는 자동으로 해제(drop)됨.
}