use std::{net::TcpStream, io::{Write, Read}};
use std::collections::HashMap;




fn make_url(url: &str) -> String{
    let mut url= url.to_string();
    if url.starts_with("https://"){
        url.drain(..8);
        let a =  url.find("/");
        if let Some(i) = a {
            url.insert_str(i, ":443")
        }else {
            url.push_str(":443")
        }
    }else if url.starts_with("http://"){
        url.drain(..7);
        let a =  url.find("/");
        if let Some(i) = a {
            url.insert_str(i, ":80")
        }else {
            url.push_str(":80")
        }
    }
    url
}

pub fn get(url: &str,  mut optionals: Option<HashMap<&str, HashMap<&str, &str>>>) -> std::io::Result<()>{
    let url = make_url(url);
    let headers = if optionals.is_some(){
        optionals.as_mut().unwrap().remove("headers")
    }else {
        None
    };
    let url = &*url;
    let (host, path) = work_on_url(url);
    let mut stream = TcpStream::connect(host)?;
    stream.write(write_message(host,path, headers).as_bytes())?;
    let mut a = [0u8; 1024];
    stream.read(&mut a)?;
    let b = String::from_utf8(a.to_vec()).unwrap();
    println!("{}",b);
    Ok(())
}

fn write_message(host: &str, path: &str, optionals: Option<HashMap<&str, &str>>) -> String {
    let mut message = String::new();
    message.push_str(&*format!("GET {} HTTP/1.1\nHost: {}\n",path, host));
    if optionals.is_none() {
        message.push_str(&*format!("User-Agent: summon/test\nAccept-Encoding: gzip, deflate\nAccept: */*\nConnection: keep-alive\n\n"));
    }else{
        let mut map = optionals.unwrap();
        let keys = map.keys().cloned().map(|s| (s.to_lowercase(), s)).collect::<Vec<_>>();
        println!("{:?}",keys);
        let params = ["user-agent", "accept-encoding", "accept", "connection"];
        for i in params {
            let a = keys
                    .iter()
                    .find(|s| {s.0 == *i});
            if let Some(a) = a {
                message.push_str(&*format!("{}: {}\n", i, map.remove(a.1).unwrap()))
            } else {
                match i {
                    "user-agent" => {
                        message.push_str(&*format!("User-Agent: summon/test\n"))
                    },
                    "accept-encoding" => {
                        message.push_str(&*format!("Accept-Encoding: gzip, deflate\n"))
                    },
                    "accept" => {
                        message.push_str(&*format!("Accept: */*\n"))
                    },
                    "connection" => {
                        message.push_str(&*format!("Connection: keep-alive\n"))
                    },
                    _ => {}
                }
            }
        }

        for (i,k) in map.drain() {
            message.push_str(&*format!("{}: {}\n",i,k));
        }
        message.push('\n');
    }


    message
}


fn work_on_url<'a>(url: &'a str) -> (&'a str, &'a str) {
    let (host, path) = if let Some(pos) = url.find('/') {
        url.split_at(pos)
    }else {
        (url, "/")
    };
    
    (host,path)
}





#[test]
fn url(){
    let a = HashMap::from(
        [("accept", "image/jpeg")]
    );
    let b = HashMap::from([("headers",a) ]);
    match get("http://httpbin.org/image", Some(b)) {
        Ok(_) => {},
        Err(e) => println!("{:?}",e)
    }
}


#[test]
fn localhost(){
    let headers = HashMap::from(
        [
            ("accept", "image/jpeg"),
            ("user-agent", "my-app0.0.1")
        ],  
    );
    let b = HashMap::from([("headers",headers) ]);
    match get("localhost:27015/image", Some(b)) {
        Ok(_) => {},
        Err(e) => println!("{:?}",e)
    }
}


#[test]
fn modify() {
    match get("https://www.google.com/k", None) {
        Ok(_) => println!("Everything was cool."),
        Err(_) => println!("errr!")
    }
}