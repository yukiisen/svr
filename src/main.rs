use stress::{
    HTTPServer,
    request::Request,
    response::Response
};
use stress::router::RouteResult;
use stress::middlewares::static_serve::serve_static;
use std::env::args;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args()?;

    let path = Box::leak(config.path.to_string_lossy().into());
    let addr = Box::leak(format!("{}:{}", config.host, config.port).into_boxed_str());

    let mut server = HTTPServer::new(config.workers.into());

    server.middleware("*", serve_static(path));
    server.last("*", Box::new(not_found));

    #[cfg(debug_assertions)]
    dbg!("{}", config.path);

    println!("Server listening on {addr}");

    server.listen(addr)?;

    Ok(())
}

struct Config {
    path: PathBuf,
    port: u16,
    host: String,
    workers: u8,
}

impl Config {
    fn from_args () -> Result<Self, Box<dyn Error>> {
        let mut argv = args().skip(1);
        let mut port = 3000;
        let mut host = "127.0.0.1".to_string();
        let mut path = PathBuf::from(".");
        let mut workers = 1;

        while let Some(arg) = argv.next() {
            if arg.starts_with("--") {
                match arg.strip_prefix("--").unwrap() {
                    "port" => { 
                        if let Some(p) = argv.next() {
                            port = p.parse()?;
                        }
                    },
                    "host" => {
                        if let Some(h) = argv.next() {
                            host = h;
                        }
                    },
                    "workers" => {
                        if let Some(w) = argv.next() {
                            workers = w.parse()?;
                        }
                    }
                    _ => {}
                }
            } else {
                path = PathBuf::from(arg);
            }
        };

        Ok(Config { path, port, host, workers })
    }
}

fn not_found(_req: &mut Request, res: &mut Response) -> RouteResult {
    res.set_status(404)?;
    res.send_file("./not-found.html")?;
    Ok(true)
}
