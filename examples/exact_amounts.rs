use bitswop_public::ExactAmount;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // An illustrative six-decimal amount; no token or network is selected.
    let amount = ExactAmount::parse_decimal("12.34", 6)?;
    println!("Major units: {amount}");
    println!("Minor units: {}", amount.minor());
    assert_eq!(amount.minor(), 12_340_000);
    Ok(())
}
