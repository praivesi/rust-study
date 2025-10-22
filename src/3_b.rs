use std::fs::File;
use std::io::{self, Read};

// Result를 반환하는 함수: 성공 시 String, 에러 시 io::Error를 반환함.
fn read_username_from_file() -> Result<String, io::Error> {
    // File::open이 에러를 반환하면, ? 연산자가 그 에러를 함수 호출자에게 즉시 반환함.
    // 성공하면 Ok() 안의 File 인스턴스를 얻어 file 변수에 할당함.
    let mut file = File::open("hello.txt")?;

    let mut username = String::new();

    // file.read_to_string도 에러를 반환하면 ?로 전파함.
    // 성공하면 ()를 반환하며 함수를 계속 진행함.
    file.read_to_string(&mut username)?;
    
    // let mut username = String::from("dummy");

    Ok(username)
}

fn main() {
    match read_username_from_file() {
        Ok(s) => println!("파일 내용: {}", s),
        Err(e) => println!("파일 읽기 에러 발생: {}", e),
    }

    // 파일이 없으면 '파일 읽기 에러 발생' 출력
}