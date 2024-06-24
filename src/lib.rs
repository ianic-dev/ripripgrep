use std::error::Error;
use std::fs;

mod tests;

pub fn getopts(args: &Vec<String>) -> (String, usize) {
    let mut opts: Vec<String> = vec![];

    let mut n_opts: usize = args.len() - 1;

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if arg.starts_with("-") {
            opts.push(arg[1..].to_string());
        } else {
            n_opts = i - 1;
            break;
        }
    }
    let optsstr = opts.join("").to_string();
    (optsstr, n_opts)
}
#[derive(Debug)]
pub struct Config {
    pub version: bool,
    pub queries: Vec<String>,
    pub fpath: String,
    pub ignorecase: bool,
    pub count: bool,
    pub invert: bool,
    pub pipe: bool,
}

impl Config {
    pub fn runconfig(args: Vec<String>) -> Result<Config, &'static str> {
        let (opts, n_optblocks) = getopts(&args);

        if args.len() == 1 {
            return Err("No path, query or options specified");
        }

        let version: bool = opts.contains("V");
        if version {
            let queries = vec![String::from("L")];
            let fpath = String::from("src/main.rs");
            Ok(Config {
                version,
                queries,
                fpath,
                ignorecase: false,
                count: false,
                invert: false,
                pipe: false,
            })
        } else {
            let ignorecase = opts.contains("i");
            let count = opts.contains("c");
            let invert = opts.contains("v");
            let pipe = opts.contains("p");
            let fpath: String = if pipe {
                String::from("/dev/stdin")
            } else if args.len() < n_optblocks + 2 {
                return Err("Insuficcient arguments");
            } else {
                args[args.len() - 1].to_string()
            };
            let mut queries: Vec<String> = vec![];
            for (i, arg) in args.iter().enumerate() {
                if i > n_optblocks && (pipe || i < args.len() - 1) {
                    queries.push(arg.to_string())
                }
            }
            Ok(Config {
                version,
                queries,
                fpath,
                ignorecase,
                count,
                invert,
                pipe,
            })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.fpath)?;

    let mut results: Vec<&str> = vec![];

    if config.ignorecase {
        for query in config.queries.iter() {
            let returns = ignsearch(config.invert, &query[..], &contents[..]);
            for result in returns {
                results.push(result);
            }
        }
    } else {
        for query in config.queries.iter() {
            let returns = search(config.invert, &query[..], &contents[..]);
            for result in returns {
                results.push(result);
            }
        }
    }

    if config.count {
        println!("{}", results.len());
    } else {
        for result in results {
            println!("{result}");
        }
    }

    Ok(())
}

pub fn search<'a>(invert: bool, query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut finds: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) ^ invert {
            finds.push(line);
        }
    }
    finds
}

pub fn ignsearch<'a>(invert: bool, query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut finds: Vec<&str> = vec![];
    let query_l = query.to_lowercase();
    let contents_l = contents.to_lowercase();
    let mut contentlines = contents.lines();
    for line in contents_l.lines() {
        if line.contains(&query_l) ^ invert {
            finds.push(
                contentlines
                    .next()
                    .expect("This should not happen. Check the ignsearch function as its brokey"),
            );
        } else {
            contentlines.next();
        }
    }
    finds
}
