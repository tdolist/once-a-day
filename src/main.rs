
#[macro_use]
extern crate clap;

use clap::App;



fn main() {
    let yml = load_yaml!("cli.yml");
    let app = App::from_yaml(yml)
        .version(crate_version!())
        .get_matches();

    match app.subcommand() {
        ("run", Some(_)) => println!("run with notifications", ),
        ("server", Some(_)) => println!("Run as server", ),
        _ => println!("{}", app.usage()),
    }
}
