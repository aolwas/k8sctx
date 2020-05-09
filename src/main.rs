use std::fs;

use clap::{Arg, App};

use std::path::{Path, PathBuf};

fn expand_tilde<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
    let p = path_user_input.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
        return dirs::home_dir();
    }
    dirs::home_dir().map(|mut h| {
        if h == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}

fn main() {
    let matches = App::new("k8sctx")
        .version("0.1.0")
        .author("Maxime Cottret (aolwas) <maxime.cottret@gmail.com>")
        .about("generate KUBECONFIG export code")
        .arg(
            Arg::with_name("config_path")
                .about("config directory path")
                .short('c')
                .long("config-path")
                .default_value("~/.kube/configs")
            )
        .subcommand(App::new("list")
            .about("List available configs"))
        .subcommand(App::new("env")
            .about("get context env")
            .arg(
                Arg::with_name("context")
                    .about("The context to set")
                    .required(true),
            ))
        .get_matches();

    let config_path = expand_tilde(matches.value_of("config_path").unwrap()).unwrap();
    match matches.subcommand() {
        ("list", _) => {
            match fs::read_dir(config_path) {
                Ok(paths) => {
                    for path in paths {
                        println!("{}", path.unwrap().path().file_name().and_then(|s| s.to_str()).unwrap())
                    }
                }
                Err(_) => {
                    println!("Error: configs path does not found")
                }
            }
        }
        ("env", Some(env_matches)) => {
            println!("export KUBECONFIG={}/{}", config_path.to_str().unwrap(), env_matches.value_of("context").unwrap());
        }
        _ => unreachable!()
    }
}
