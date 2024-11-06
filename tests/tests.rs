use pest::Parser;
use anyhow::anyhow;
use pest_project::*;

#[test]
fn main_field_test() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::field, "-273.15")?.next().ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "-273.15");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 7);

    let pair = Grammar::parse(Rule::field, "abc");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::field, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn main_record_test() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::record, "-273.15,5,2.3")?.next().ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "-273.15,5,2.3");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 13);

    let pair = Grammar::parse(Rule::record, "-273.15,abc,2.3");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::record, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn record_test_incorrect_format() {
    let pair = Grammar::parse(Rule::record, "-273.15 5 2.3");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::record, "-273.15,,2.3");
    assert!(pair.is_err());
}

#[test]
fn file_test() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::file, "-273.15,99\n")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "-273.15,99\n");

    let pair = Grammar::parse(Rule::file, "x");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::file, "-273.15 99\n");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::file, "-273.15,99,85,98.0");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn file_test_equal() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::file, "-273.15,99\n36.8,98.1\n")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "-273.15,99\n36.8,98.1\n");

    Ok(())
}