use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 입력받은 문자열을 읽어 u64로 파싱
    // u32로 파싱하면 런타임 에러 발생
    // 32비트 크기면, 문제에 의거한 최댓값을 표시할 수 없음.
    let input_line = lines.next().unwrap().unwrap();
    let mut s: u64 = input_line.trim().parse().unwrap();

    let mut i: u64 = 1;

    while s - i > i {
        s -= i;
        i += 1;
    }
    
    println!("{}", i);
}