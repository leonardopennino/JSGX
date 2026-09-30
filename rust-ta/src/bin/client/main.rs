use clap::Parser;
use rust_ta::client::{self, Client};
use rust_ta::commands::{self, Command};
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(name = "JSGX Client")]
#[command(about = "Client to communicate with JSGX Enclaves")]

struct Args {
    #[arg(short, long)]
    config_file: Option<String>,
    #[arg(short('u'), long)]
    base_url: Option<String>,
    commands: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Args::parse();
    if let Some(config_file) = cli.config_file {
        //let config = client::JSGXClient;
    };
    if let Some(commands) = cli.commands {
        let commands : Vec<Command> = commands::parse_commands(&commands).unwrap();
        let client = Client::new("http://127.0.0.1:3000");
        run_commands(&commands, &client).await.unwrap();
    }
    /*
    let config: ClientConfig = ClientConfig {
        base_url: "http://localhost:3000".into(),
        mr_enclave: "TSTMRENCLAVE".into(),
        key_path: Some("./resources/tests/govA.private.der".into()),
        commands,
    };
    let client = client::TaAuthClient::from_config(&config).unwrap();
    run_commands_with_auth(&config.commands, &client).await;
    */
    Ok(())
}

async fn handle_command(command: &Command, client: &Client) -> Result<Value, Box<dyn std::error::Error>> {
    match command {
        Command::Method(m, p) => {
            Ok(client.call_method(m, Some(p)).await.unwrap())
        }
        Command::Authenticate => {
            client.authenticate().await;
            Ok(Value::Null)
        }
        Command::AuthenticatedMethod(m, p) => {
            Ok(client.call_auth_method(m, Some(p)).await.unwrap())
        }
    }
}
async fn run_commands(commands: &[Command], client: &Client) -> Result<(),Box<dyn std::error::Error>> {
    for command in commands {
        let out = handle_command(command, client).await?;
        println!("{out}");
    }
    Ok(())
}

/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert = get_certificate("http://127.0.0.1:3333/certificate".to_string()).await;
    //ra_tls::verify::ra_tls_verify(&cert).await.unwrap();
    let _client = ClientBuilder::new()
        .tls_info(true)
        .add_root_certificate(reqwest::Certificate::from_pem(cert.as_bytes()).unwrap());
    Ok(())
}

async fn get_certificate(endpoint: String) -> String {
    let client = ClientBuilder::new().build().unwrap();
    match client
        .get(endpoint)
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Ok(x) => x.text().await.unwrap(),
        Err(_) => panic!("Impossible to get RA-TLS certificate"),
    }
}
*/
