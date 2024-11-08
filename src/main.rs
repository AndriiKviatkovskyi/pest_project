use pest::Parser;
use anyhow::anyhow;
use pest_project::*;

fn main() -> Result<(), anyhow::Error>{
    let parsed_data_field = Grammar::parse(Rule::field, "-273.15")?;
    dbg!(parsed_data_field);
    let parsed_data_record = Grammar::parse(Rule::record, "-273.15,5,2.3")?;
    dbg!(parsed_data_record);
    Ok(())
}
