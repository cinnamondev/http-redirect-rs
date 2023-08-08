use warp::{Filter, http::Response, reply, http::status, http::header};
use base64::{engine::general_purpose, Engine as _};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct b64 {
    dest: String
}

#[tokio::main]
async fn main() {
    let unauth = warp::any()
    .map(|| {
        reply::with_header(
            reply::with_status(
                warp::reply(),
                status::StatusCode::UNAUTHORIZED
        ),
          "WWW-Authenticate", r#"Basic realm="Test""#
    )});

    let authed = warp::header("authorization")
    .map(|token: String| {
        let redirect = to_uri(&token);
            Response::builder()
                .status(200)
                .header("Refresh", "0;url=http://".to_owned() + &redirect)
                .body(r#"
                <iframe id="if"></iframe>
                <script defer>
                    document.getElementById('if').src = "http://127.0.0.1:3030/payload?dest="#.to_owned() 
                    + &token[6..] + 
                r#"";</script>"#
            )
        });    
    
    
    let opt_query = warp::query::<b64>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<b64>,), std::convert::Infallible>((None,)) });
    

    let payload = warp::path("payload")
        .and(opt_query)
        .map(|b: Option<b64>| {
            let base = match b {
                Some(s) => s.dest,
                None => "".to_owned(),
            };
            println!("{}", base);
            Response::builder()
                .status(200)
                .header("Content-Disposition", r#"attachment; filename="bundle.txt""#)
                .body(format!("boo womp :( . base64: {}",base))
        });

    let routes = warp::get()
        .and(payload)               // /payload
        .or(authed.or(unauth));   // / request auth if not present
        
        
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn to_uri(input: &str) -> String {
    // attempt to convert a user:pass with formatted unicode divide slash to real uri.
    // legend: , -> . / -> / etc
    println!("{}", input);
    //let alphabet = base64::alphabet::Alphabet::new("")
    let a = general_purpose::STANDARD // b64 -> Vec<u8>
        .decode(&input[6..])
        .unwrap()
        .iter()
        .filter_map(|&x| {
            if x != 58 {Some(x)} else {None}
        })
        .collect::<Vec<u8>>();

    String::from_utf8_lossy(&a) // Vec<u8> -> UTF-8 String
        .chars()
        .map(map_special) // map special unicode
        .collect()
}

fn map_special(c: char) -> char {
    match c {
        '∕' => '/', // U+2215 (Division Slash) to /
        ',' => '.',
        c => c,
    }
}