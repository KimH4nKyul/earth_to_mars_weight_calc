fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight/9.81) * 3.711
}

fn main() {
    let mut mars_weight: f32 = calculate_weight_on_mars(64.7);
    mars_weight = mars_weight * 1000.0;
    println!("Ur weight on mars: {}g", mars_weight);
}