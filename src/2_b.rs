fn main() {
    let mut s = String::from("빌림_테스트");

    // 1. 불변 빌림: 여러 개 허용 (읽기 접근)
    let r1 = &s; 
    let r2 = &s; 
    println!("r1: {}, r2: {}", r1, r2); 

    // 2. 가변 빌림: 오직 하나만 허용 (쓰기 접근)
    let r3 = &mut s; 
    r3.push_str("_변경됨");
    
    // let r4 = &mut s; // r3이 유효한 동안 다른 가변 참조자를 만들려 하면 컴파일 에러
    
    // 러스트의 NLL(Non-Lexical Lifetimes) 
    // - 참조자의 수명을 스코프의 끝이 아니라 참조자의 마지막 사용 지점까지만으로 제한
    
    println!("r3: {}", r3); 
    // println!("r1: {}", r1); // r3가 생성된 이후 불변 참조자를 사용하려 하면 컴파일 에러
}