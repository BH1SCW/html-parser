
use html_parser::{Dom, Result};

    fn main() -> Result<()> {
        let html = "<div id=cat />";
        let json = Dom::parse(html)?.to_json_pretty()?;
        println!("{}", json);
        Ok(())
    }
