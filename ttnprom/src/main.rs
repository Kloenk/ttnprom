use clap::{App, Arg, SubCommand};
use ttnprom::{Config, PluginConf};

fn main() {
    let mut app = App::new("ttnprom")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Kloenk <me@kloenk.de>")
        .about("ttn to prometheus gateway")
        .setting(clap::AppSettings::ColorAuto)
        .setting(clap::AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("set config file")
                .takes_value(true)
                .default_value("config.toml")
        )
        .arg(
            Arg::with_name("plugin")
                .short("p")
                .long("plugin")
                .value_name("FILE")
                .help("load plugin file (arguments seperated with ':')")
                .takes_value(true)
                .multiple(true)
        );
    
    if cfg!(feature = "completion") {
        app = app.subcommand(
            SubCommand::with_name("completion")
                .about("create completions")
                .version("0.1.0")
                .author("Kloenk <me@kloenk.de>")
                .arg(
                    Arg::with_name("shell")
                        .help("set the shell to create for. Tries to identify with env variable")
                        .index(1)
                        .required(false)
                        .value_name("SHELL")
                        .possible_value("fish")
                        .possible_value("bash")
                        .possible_value("zsh")
                        .possible_value("powershell")
                        .possible_value("elvish"),
                )
                .arg(
                    Arg::with_name("out")
                        .help("sets output file")
                        .value_name("FILE")
                        .short("o")
                        .long("output"),
                )
                .setting(clap::AppSettings::ColorAuto)
                .setting(clap::AppSettings::ColoredHelp)
        );
    }

    let matches = app.clone().get_matches();

    if cfg!(feature = "completion") {
        if let Some(matches) = matches.subcommand_matches("completion") {
            completion(&matches, &mut app);
            std::process::exit(0);
        }
    }

    let mut config = Config::new();

    if let Some(values) = matches.values_of("plugin") {
        let values: Vec<_> = values.collect();
        for v in values.iter() {
            config.plugins.push(PluginConf::parse(v));
        }
    }

    config.run();
}


/// create completion
#[cfg(feature = "completion")]
fn completion(args: &clap::ArgMatches, app: &mut App) {
    let shell: String = match args.value_of("shell") {
        Some(shell) => shell.to_string(),
        None => shell()
    };

    use clap::Shell;
    let shell_l = shell.to_lowercase();
    let shell: Shell;
    if shell_l == "fish".to_string() {
        shell = Shell::Fish;
    } else if shell_l == "zsh".to_string() {
        shell = Shell::Zsh;
    } else if shell_l == "powershell".to_string() {
        shell = Shell::PowerShell;
    } else if shell_l == "elvish".to_string() {
        shell = Shell::Elvish;
    } else {
        shell = Shell::Bash;
    }

    use std::fs::File;
    use std::io::BufWriter;
    use std::io::Write;

    let mut path = BufWriter::new(match args.value_of("out") {
        Some(x) => Box::new(
            File::create(&std::path::Path::new(x)).unwrap_or_else(|err| {
                eprintln!("Error opening file: {}", err);
                std::process::exit(1);
            }),
        ) as Box<Write>,
        None => Box::new(std::io::stdout()) as Box<Write>,
    });


    app.gen_completions_to("ttnprom", shell, &mut path);
}

#[cfg(all(feature = "completion", not(windows)))]
fn shell() -> String {
    let shell: String = match std::env::var("SHELL") {
            Ok(shell) => shell,
            Err(_) => "/bin/bash".to_string(),
    };
    let shell = std::path::Path::new(&shell);
    match shell.file_name() {
        Some(shell) => shell.to_os_string().to_string_lossy().to_string(),
        None => "bash".to_string(),
    }
}

#[cfg(all(feature = "completion", windows))]
fn shell() -> String {
    "powershell".to_string()    // always default to powershell on windows
}

#[cfg(not(feature = "completion"))]
fn completion(_: &clap::ArgMatches, _: &mut App) {
    eprintln!("Completion command fired but completion not included in features");
    std::process::exit(-1);
}
