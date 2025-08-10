use clap::{CommandFactory, Parser};

/// Doc comment maybe be the about string
#[derive(Parser, Debug)]
#[command(name = "clap-test-name", version = "0.0.0-xxx", about = "about in command attribute, highest priority", long_about = None)]
//#[command(version, about, long_about = None)] // Uncomment to use default version and about
struct Args {
    #[arg(short, long, default_value_t = 1.0)]
    length: f64,
}

fn main() {
    // Include the Cargo.toml file so that changes to Cargo.toml triggers a rebuild
    include_str!("../Cargo.toml");

    let args = Args::parse();

    // Programmatic access to clap metadata, there are many get_xxx methods available
    let args_commmand_metadata = Args::command();

    // Priority:
    //  1) command attribte name = "clap-test-name"
    //  2) Cargo.tomal name = "clap-test"
    //        "clap-test" if from Carog.toml::name
    //        or "clap-test-name" if from command attribute name above
    //
    // There is a bug, if name is set in command attribute above it should always
    // be "clap-test-name", but for `-h` it is "clap-test" instead.
    let name = args_commmand_metadata.get_name();
    println!("name = \"{}\"", name);

    // Priorities:
    //  1) command attribte about = "about in command attribute, highest priority"
    //  2) Cargo.toml description = "about if not set in command attribute" 
    //  3) Doc comment about = "Doc comment maybe be the about string"
    let about = args_commmand_metadata.get_about().unwrap_or_default();
    println!("about = \"{}\"", about);

    let version = args_commmand_metadata.get_version().unwrap_or_default();
    println!("version = {}", version); // Prints: "clap-test 0.1.0" or "0.0.0" if not set in Cargo.toml

    // Your app logic
    println!("length = {:.2}", args.length);
}