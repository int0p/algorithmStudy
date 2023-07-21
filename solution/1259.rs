use std::io;
mod test{

}
fn main() {
    loop {
        //준비
        let mut number = String::new();

        io::stdin().read_line(&mut number).unwrap();
        number = number.trim().to_string(); // 개행 문자 제거

        if number.parse::<i16>() == Ok(0) {
            //입력된 문자열을 i16로 변환한 결과가 0인지 확인
            break;
        }

        //시작

        let reversed: String = number.chars().rev().collect();

        if number == reversed {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
