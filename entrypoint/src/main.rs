use colored::Colorize;

mod cli;

fn main() {
    let args = cli::parse();

    println!(); // break line

    println!("{} {}", "Running the:".cyan(), args.appname.bold().cyan());

    match args.appname.as_str() {
        "renderer" => {
            renderer::main();
        }
        _ => panic!("App name {} not implemented yet", args.appname),
    }

    println!(); // break line
}
