use std::io;

fn main() {
    //준비
    let mut num_subject = String::new();
    let mut scores = String::new();

    io::stdin().read_line(&mut num_subject).unwrap();
    io::stdin().read_line(&mut scores).unwrap();

    let num_subject: Vec<&str> = num_subject.split_whitespace().collect();
    let num_subject = num_subject[0].parse::<f32>().unwrap();

    let mut scores: Vec<i16> = scores
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    scores.sort();

    let max_score: i16 = scores[scores.len() - 1];
    let mut new_scores: Vec<f32> = vec![];

    for score in scores {
        // if (score == max_score) {
        //     new_scores.push(max_score as f32);
        //     break;
        // }
        let mut new_score: f32 = (score as f32) / (max_score as f32) * 100.0;
        new_scores.push(new_score);
    }

    let sum: f32 = new_scores.iter().sum();

    println!("{:?}", sum / num_subject);
}
