use bitswop_public::{AmountError, ExactAmount};

#[test]
fn decimal_input_preserves_minor_units() {
    let amount = ExactAmount::parse_decimal("12.340005", 6).unwrap();
    assert_eq!(amount.minor(), 12_340_005);
    assert_eq!(amount.to_string(), "12.340005");
}

#[test]
fn shorter_fractions_are_padded_without_rounding() {
    let amount = ExactAmount::parse_decimal("12.34", 6).unwrap();
    assert_eq!(amount.minor(), 12_340_000);
    assert_eq!(amount.to_string(), "12.340000");
}

#[test]
fn whole_numbers_and_zero_are_formatted_at_the_requested_precision() {
    assert_eq!(
        ExactAmount::parse_decimal("12", 6).unwrap().minor(),
        12_000_000
    );
    assert_eq!(
        ExactAmount::from_minor(0, 6).unwrap().to_string(),
        "0.000000"
    );
    assert_eq!(ExactAmount::from_minor(42, 0).unwrap().to_string(), "42");
}

#[test]
fn malformed_input_is_rejected() {
    for input in [
        "", " ", " 1", "1 ", "+1", "-1", ".1", "1.", "1.2.3", "1e3", "1,00", "１２", "NaN",
    ] {
        assert_eq!(
            ExactAmount::parse_decimal(input, 6),
            Err(AmountError::InvalidSyntax),
            "{input:?}"
        );
    }
}

#[test]
fn excess_precision_is_rejected_instead_of_truncated() {
    assert_eq!(
        ExactAmount::parse_decimal("1.0000000", 6),
        Err(AmountError::ExcessPrecision)
    );
    assert_eq!(
        ExactAmount::parse_decimal("1.0", 0),
        Err(AmountError::ExcessPrecision)
    );
}

#[test]
fn unsupported_precision_is_rejected_before_formatting() {
    for decimals in 39..=u8::MAX {
        assert_eq!(
            ExactAmount::from_minor(0, decimals),
            Err(AmountError::UnsupportedPrecision)
        );
    }
}

#[test]
fn overflow_in_the_integer_or_scaled_amount_is_rejected() {
    assert_eq!(
        ExactAmount::parse_decimal("340282366920938463463374607431768211456", 0),
        Err(AmountError::Overflow)
    );
    assert_eq!(
        ExactAmount::parse_decimal(&u128::MAX.to_string(), 1),
        Err(AmountError::Overflow)
    );
}

#[test]
fn overflow_when_combining_whole_and_fraction_is_rejected() {
    assert_eq!(
        ExactAmount::parse_decimal("34028236692093846346337460743176821145.6", 1),
        Err(AmountError::Overflow)
    );
}

#[test]
fn precision_mismatch_and_underflow_cannot_produce_a_balance() {
    let amount = ExactAmount::from_minor(10, 6).unwrap();
    assert_eq!(
        amount.checked_sub(ExactAmount::from_minor(1, 2).unwrap()),
        None
    );
    assert_eq!(
        amount.checked_sub(ExactAmount::from_minor(11, 6).unwrap()),
        None
    );
    assert_eq!(amount.checked_sub(amount).unwrap().minor(), 0);
    assert_eq!(
        amount
            .checked_sub(ExactAmount::from_minor(3, 6).unwrap())
            .unwrap()
            .minor(),
        7
    );
}

#[test]
fn display_and_parse_round_trip_at_all_supported_precisions() {
    for decimals in 0..=38 {
        for minor in [0, 1, 9, 10, 999_999, 1_000_000, u64::MAX as u128, u128::MAX] {
            let original = ExactAmount::from_minor(minor, decimals).unwrap();
            assert_eq!(
                ExactAmount::parse_decimal(&original.to_string(), decimals),
                Ok(original)
            );
        }
    }
}

#[test]
fn leading_zeroes_do_not_change_value() {
    assert_eq!(
        ExactAmount::parse_decimal("000012.034", 6).unwrap().minor(),
        12_034_000
    );
}

#[test]
fn high_precision_does_not_overflow_its_scale() {
    let amount =
        ExactAmount::parse_decimal("0.00000000000000000000000000000000000001", 38).unwrap();
    assert_eq!(amount.minor(), 1);
    assert_eq!(amount.decimals(), 38);
}
