use std::io;

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight/9.81) * 3.711
}

fn main() {
    println!("당신의 몸무게를 입력하세요: ");
    let mut input = String::new();   
    io::stdin().read_line(&mut input).unwrap();
    println!("Input: {}", input);

    // let w: f32 = input.trim().parse().unwrap();
    let w = input.trim();
    let real_w = w.parse::<f32>().unwrap();

    let mars_weight: f32 = calculate_weight_on_mars(real_w);
    println!("Ur weight on mars: {}kg", mars_weight);
}