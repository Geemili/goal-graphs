#[macro_use] extern crate quicli;
use quicli::prelude::*;

extern crate plotlib;
extern crate structopt;
extern crate toml;
extern crate regex;
extern crate serde;
#[macro_use] extern crate lazy_static;

use regex::Regex;
use plotlib::scatter::Scatter;
use plotlib::view::View;
use plotlib::plot::Plot;
use std::path::Path;

#[derive(Debug, StructOpt)]
struct Cli {
    file: String,

    /// Pass many times for more output
    #[structopt(long = "verbose", short = "v")]
    verbosity: u64,
}

main!(|args: Cli, log_level: verbosity| {
    let content = read_file(&args.file)?;
    let config: Config = toml::from_str(&content)?;

    for goal in config.goals.iter() {
        println!("{:?}", goal);
        plot_goal(&args, goal)?;
    }
});

fn plot_goal(args: &Cli, config: &GoalConfig) -> Result<()> {
    let dir = Path::new(&args.file).parent().expect("File should not be root directory");
    let path = dir.join(&config.data);

    let content = read_file(&path)?;
    let lines = content.lines();

    let mut data = vec![];

    for line in lines {
        let cols: Vec<&str> = line.split(',').collect();
        match date_to_float(cols[0]) {
            Ok(date) => {
                let value: f64 = cols[1].parse()?;
                data.push((date as f64, value));
            }
            _ => {
                println!("{}\t{}", cols[0], cols[1]);
            }
        }
    }

    let s1 = Scatter::from_vec(&data);

    let v = View::new()
         .add(&s1);

    println!("{}", Plot::single(&v).to_text());
    Ok(())
}

fn date_to_float(date_string: &str) -> Result<i64> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\A(\d{4})w(\d{2})([[:lower:]]{2})?\z").unwrap();
    }
    let captures = RE.captures(date_string).ok_or_else(|| format_err!("date_string did not match format! {}", date_string))?;

    let year = captures.get(1).ok_or_else(|| format_err!("No year in date"))?.as_str();
    let week = captures.get(2).ok_or_else(|| format_err!("No week in date"))?.as_str();

    let day = captures.get(3).map(|x| x.as_str()).unwrap_or("ze");

    let year: i64 = year.parse()?;
    let week: i64 = week.parse()?;

    let day = match day {
        "no" => -1,
        "pa" =>  0,
        "re" =>  1,
        "ci" =>  2,
        "vo" =>  3,
        "mu" =>  4,
        "xa" =>  5,
        "ze" =>  6,
        _ => bail!("Invalid day string: {}", day),
    };

    let date_number = year * 53 * 7 + week * 7 + day;
    Ok(date_number)
}

#[derive(Debug, Deserialize)]
struct Config {
    goals: Vec<GoalConfig>,
}

#[derive(Debug, Deserialize)]
struct GoalConfig {
    name: String,

    /// The file from which to pull data from
    data: String,
    aim: String,
    goal: f64,
}
