use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> Result<(), anyhow::Error>{
    let parsed_data_field = Grammar::parse(Rule::field, "-273.15")?;
    dbg!(parsed_data_field);
    let parsed_data_record = Grammar::parse(Rule::record, "-273.15,5,2.3")?;
    dbg!(parsed_data_record);
    Ok(())
}
