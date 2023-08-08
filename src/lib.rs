use std::{fs, env, error::Error};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn from(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next();
        Ok(Config {
            query: match args.next() {
                Some(query) => query,
                None => return Err("Query not found")
            },
            file_path: match args.next() {
                Some(path) => path,
                None => return Err("File path not found")
            },
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let Config {
        query,
        file_path,
        ignore_case,
    } = config;

    let text = fs::read_to_string(file_path)?;
    let res = if ignore_case {
        search_case_insensitive(&text, &query)
    } else {
        search(&text, &query)
    };
    for (n, line) in res {
        println!("Line {n:02}: {line}");
    }

    Ok(())
}

fn search<'a>(
    text: &'a str,
    query: &str,
) -> Vec<(usize, &'a str)> {
    text
        .lines()
        .enumerate()
        .filter(
            |line|
            line.1.contains(query)
        )
        .map(|(n, l)| (n + 1, l))
        .collect()
}
fn search_case_insensitive<'a>(
    text: &'a str,
    query: &str,
) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    
    text
        .lines()
        .enumerate()
        .filter(
            |line|
            line.1.to_lowercase().contains(&query)
        )
        .map(|(n, l)| (n + 1, l))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn case_sensitive() {
        let query = "to";
        let text = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";
        
        assert_eq!(
            vec![
                (2, "Are you nobody, too?"),
                (6, "How dreary to be somebody!"),
            ],
            search(text, query),
        );
    }
    #[test]
    fn case_insensitive() {
        let query = "To";
        let text = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";
        
        assert_eq!(
            vec![
                (2, "Are you nobody, too?"),
                (6, "How dreary to be somebody!"),
                (8, "To tell your name the livelong day"),
                (9, "To an admiring bog!"),
            ],
            search_case_insensitive(text, query),
        );
    }
}
