use ureq;
use std::fs::File;
use std::io::Write;

use scraper::Html;
use scraper::Selector;

use clap::{Command, Arg, ArgAction};

fn main() -> Result<(), ureq::Error> {

    let mut username:String = String::from("");

    if let Ok(u) = std::env::var("USERNAME") {
        username = u.clone();
    }

    if let Ok(u) = std::env::var("USER") {
        username = u.clone();
    }

    let _ = std::io::stdout().flush();

    let password = rpassword::prompt_password("Password: ")
        .unwrap();

    let cmd = Command::new("dcr")
        .arg(
            Arg::new("username")
            .short('u')
            .long("username")
            .action(ArgAction::Set)
            .ignore_case(true)
            .required(false)
            .help("Override username from environment")
        )
        .arg(
            Arg::new("dump")
            .short('d')
            .long("dump")
            .default_value("response.html")
            .default_missing_value("response.html")
            .num_args(0..=1)
            .action(ArgAction::Set)
            .required(false)
            .help("Dump html response in specified file (default name: response.html)")
        );

    let matches = cmd.get_matches();

    if username.len() == 0 {

        if let Some(uname) = matches.get_one::<String>("username") {
            username = uname.to_string();
        } else {
            println!("Username option (-u,--username) is mandatory");
            std::process::exit(1);
        }
    }

    let dump = matches.get_one::<String>("dump")
        .expect("Expected to have default and not to fail");

    get_report(username.as_str(), &password, Some(dump.to_string()))
}

fn day() -> String {
    //dd%2FDD%2Fyyyy
    "24%2F11%2F2024".to_string()
}

fn get_report(username:&str, password:&str, dump:Option<String>) -> Result<(), ureq::Error> {

    let mut basic_auth = format!( "rete\\{}:{}", username, password );
    basic_auth = format!("Basic {}", simple_base64::encode(basic_auth) );

    let body = File::open("body.txt").unwrap();

    let res = ureq::post("http://10.194.137.36/ACCESSIDC/ReportGiornaliero.aspx")
        .set("Authorization", &basic_auth )
        .set("Content-Type", "application/x-www-form-urlencoded")
        .set("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
        .set("Accept-Language", "it-IT,it;q=0.9")
        .set("Accept-Encoding", "gzip, deflate")
        .send(body);

    if let Err(err) = res {
        println!("{:?}", err);
        std::process::exit(1);
    }

    let resp = res.expect("Response");
    let content = resp.into_string()?;

    if let Some(name) = dump {
        let mut f = File::create(name).unwrap();
        f.write( content.as_bytes() ).unwrap();
    }


    let table_selector = Selector::parse("#ADC_ContenutoSpecificoPagina_gvGiornaliero > tbody tr")
        .expect("Selector Error");

    let doc = Html::parse_document(&content);

    let mut rows = doc.select(&table_selector);

    let _ = rows.next();

    for node in rows {
        let text:Vec<&str> = node.text()
            .filter(|s| !s.starts_with("\n") )
            .collect();
        for t in text {
            print!("{} |", t);
            println!();
        }

        break;
    }

    Ok(())    
}
