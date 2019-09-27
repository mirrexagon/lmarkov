use super::*;

#[test]
fn train() {
    let mut chain = Chain::new(1);

    chain.train("Hello there! I like cheese.");
    chain.train("Hello there! The day is very nice, like a good knive.");

    println!("{}", chain.generate());
}
