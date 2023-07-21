use std::io;

fn main() {
    //준비
    let mut condition = String::new();
    let mut numbers = String::new();

    io::stdin().read_line(&mut condition).unwrap();
    io::stdin().read_line(&mut numbers).unwrap();

    let condition: Vec<i32> = condition
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let N = condition[0];
    let M = condition[1];

    let mut numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    numbers.sort();
    numbers.reverse();
    // println!("{:?}", &numbers);

    //시작!
    //숫자배열에 M보다 큰 값이 있는 경우는 고려하지 않아도 됨.
    let mut i: i32 = 0;
    for num in &numbers {
        if num <= &M {
            break;
        } else {
            i += 1;
        }
    }

    let target = &numbers[i as usize..];
    let mut result: i32 = 0;
    // println!("{:?}", target);

    for i in 0..(target.len() - 2) {
        for j in (i + 1)..(target.len() - 1) {
            for k in (j + 1)..target.len() {
                let sum: i32 = target[i] + target[j] + target[k];
                if sum > M {
                    continue;
                } else {
                    result = std::cmp::max(result, sum);
                    if (sum == M) {
                        break;
                    }
                }
            }
        }
    }
    println!("{}", result);
}
