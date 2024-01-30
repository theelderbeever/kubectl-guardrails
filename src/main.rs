use std::{
    env,
    fs::File,
    io::{self, Write},
    process::{exit, Command},
    str::from_utf8,
};

use colored::Colorize;
use homedir::get_my_home;
use serde::Deserialize;
use text_io::try_read;

fn main() {
    let config = get_config();

    let current_context = get_current_context("kubectl");

    let args = env::args().collect::<Vec<String>>()[1..].to_vec();

    let mut cmd = Command::new("kubectl");
    cmd.args(&args);

    if let Some(prompt) = config
        .get_ctx(&current_context)
        .and_then(|ctx| ctx.get_prompt(env::args().nth(1).as_deref()))
    {
        println!(
            "================ {} ==================\n",
            "Context".yellow().bold()
        );
        println!("Current Context: {current_context}\n");
        println!(
            "================ {} ==================\n",
            "Dry Run".yellow().bold()
        );
        if prompt.dry_run {
            let mut dry_run = Command::new("kubectl");
            dry_run.args(&args);
            dry_run.arg("--dry-run=client");
            let output = dry_run.output().expect("Failed to perform dry-run");
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            println!("\n===========================================\n");
        }
        print!("Would you like to proceed? (y/N): ");
        let r: String = try_read!().expect("Invalid input");

        match r.to_lowercase().as_str() {
            "y" => println!("\n❯ {} {}\n", "kubectl".green(), &args.join(" ")),
            "n" => exit(0),
            _ => {
                println!("{}", "Invalid response!".red());
                exit(1)
            }
        }
    }

    let output = cmd.output().expect("Failed to run command");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn get_current_context(bin: &str) -> String {
    let ctx = Command::new(bin)
        .args(["config", "current-context"])
        .output()
        .unwrap_or_else(|_| {
            panic!("Failed to find current context with `{bin} config current-context`")
        })
        .stdout;
    from_utf8(ctx.strip_suffix(b"\n").unwrap_or(&ctx))
        .unwrap()
        .to_string()
}

fn get_config() -> Config {
    let conf_path = std::env::var("KUBECTL_GUARDRAILS_CONF").unwrap_or_else(|_| {
        let home = get_my_home().ok().flatten().unwrap();
        let home = home.to_str().unwrap();
        format!("{home}/.kube/guardrails")
    });

    let rdr = File::open(&conf_path)
        .unwrap_or_else(|_| panic!("Could not open guardrails config at '{conf_path}'"));
    serde_yaml::from_reader(rdr)
        .unwrap_or_else(|e| panic!("Could not read guardrails config at '{conf_path}'. {e}"))
}

#[derive(Deserialize, Debug)]
struct Prompt {
    pub name: String,
    #[serde(default, rename = "dry-run")]
    pub dry_run: bool,
}

#[derive(Deserialize, Debug)]
struct Context {
    pub name: String,
    pub prompts: Vec<Prompt>,
}

impl Context {
    pub fn get_prompt(&self, prompt: Option<&str>) -> Option<&Prompt> {
        match prompt {
            Some(prompt) => self.prompts.iter().find(|p| prompt == p.name),
            None => None,
        }
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default)]
    pub contexts: Vec<Context>,
}

impl Config {
    pub fn get_ctx(&self, ctx: &str) -> Option<&Context> {
        self.contexts.iter().find(|c| ctx == c.name)
    }
}
