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
        .set("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
        .set("Accept-Language", "it-IT,it;q=0.9")
        .set("Accept-Encoding", "gzip, deflate")
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

    let table_selector = Selector::parse("#ADC_ContenutoSpecificoPagina_gvGiornaliero > tbody tr")
        .expect("Selector Error");

    let doc = Html::parse_document(&content);

    let mut rows = doc.select(&table_selector);

    let _ = rows.next();

    for node in rows {
        //println!("{:?}", node.html());
        let text = node.text();
        for t in text {

            if ! t.starts_with("\n") {
                print!("{} |", t);
                println!();
            }
        }

        break;
    }

    Ok(())    
}
