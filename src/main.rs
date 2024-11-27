use ureq;
use std::fs::File;
use std::io::Write;

use scraper::Html;
use scraper::Selector;

fn main() -> Result<(), ureq::Error> {

    let body = File::open("body.txt").unwrap();

    let res = ureq::post("http://10.194.137.36/ACCESSIDC/ReportGiornaliero.aspx")
        .set("Authorization", "Basic cmV0ZVxtYW56b2dpOTo0S3J1bTFyMQ==")
		.set("Content-Type", "application/x-www-form-urlencoded")
        .send(body);

    if let Err(err) = res {
		println!("{:?}", err);
		std::process::exit(1);
    }
	
	let resp = res.unwrap();
	println!("{}", resp.status_text());

	let mut f = File::create("resp.html").unwrap();
    let content = resp.into_string()?;
	f.write( content.as_bytes() ).unwrap();
	
	let table_selector = Selector::parse("#ADC_ContenutoSpecificoPagina_gvGiornaliero > tbody tr").unwrap();
    
    let doc = Html::parse_document(&content);

    let rows = doc.select(&table_selector);

    for tr in rows {
        println!("{}", tr.inner_html());
    }

    Ok(())    
}
