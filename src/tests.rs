use super::*;

#[test]
fn train() {
    let mut chain = Chain::new(2);

    chain.train("Hello there! I like cheese.");

    println!("{:#?}", chain);
}
