

fn main() {

    let model = tch::CModule::load("CartPole-v1.pt");

    println!("{:?}", model);
}
