use clap::{Parser, Subcommand};

mod gen_cert;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand, Debug)]
enum CliCommand {
    GenAllCerts,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = match cli.command {
        CliCommand::GenAllCerts => {
            // TODO: support configuring via CLI or config file
            let config = gen_cert::IyesMpCertConfig {
                name_authsrv: "auth.iyes.games".into(),
                name_hostsrv: "host.iyes.games".into(),
                certdir: "certs".into(),
            };
            gen_cert::gen_certs(&config)
        }
    } {
        eprintln!("Error: {:#}", e);
    }
}
