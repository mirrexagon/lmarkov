use super::*;

#[test]
fn train() {
    let mut chain = Chain::new(1);

    chain.train("Hello there! I like cheese.");
    chain.train("Hello there! The day is very nice, like a good knive.");

    println!("{}", chain.generate());
}

#[test]
#[cfg(feature = "serialization")]
fn serialize() {
    let mut chain = Chain::new(2);

    chain.train("Hello there! I like cheese.");
    chain.train("Hello there! The day is very nice, like a good knive.");

    let json = chain.to_json().unwrap();
    println!("{}", json);

    let loaded_chain = Chain::from_json(&json);
}
