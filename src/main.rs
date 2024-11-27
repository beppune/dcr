use ureq;
use std::fs::File;
use std::io::Write;
use std::io::Read;

fn main() -> Result<(), ureq::Error> {

    let body = File::open("body.txt").unwrap();

    let res = ureq::post("http://10.194.137.36/ACCESSIDC/ReportGiornaliero.aspx")
        .set("Authorization", "Basic cmV0ZVxtYW56b2dpOTo0S3J1bTFyMQ==")
		.set("Content-Type", "application/x-www-form-urlencoded")
        .send(body);

    if let Ok(resp) = res {
        println!("{}", resp.status_text());

        let mut f = File::create("resp.html").unwrap();
        f.write( resp.into_string()?.as_bytes() ).unwrap();
    }

    Ok(())    
}
