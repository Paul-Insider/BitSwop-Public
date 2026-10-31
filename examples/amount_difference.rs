use bitswop_public::ExactAmount;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Synthetic values for the SAME illustrative asset and network.
    // This arithmetic does not verify a transaction or confirm a payment.
    let expected = ExactAmount::parse_decimal("100.00", 6)?;
    let observed = ExactAmount::parse_decimal("99.75", 6)?;
    let difference = expected
        .checked_sub(observed)
        .expect("matching precision and nonnegative difference");
    println!("Expected: {expected}");
    println!("Observed: {observed}");
    println!("Difference: {difference}");
    assert_eq!(difference.minor(), 250_000);
    Ok(())
}
