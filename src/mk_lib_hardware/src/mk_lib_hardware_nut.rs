// https://github.com/aramperes/nut-rs
// https://crates.io/crates/rups

use rups::tokio::Connection;
use rups::{Auth, ConfigBuilder};
use std::convert::TryInto;

// 3493 is the default port
pub async fn mk_lib_hardware_nut_connection(
    nut_host: String,
    nut_port: u16,
    nut_user: String,
    nut_password: String,
) -> Result<Connection, Box<dyn std::error::Error>> {
    let auth = Auth::new(nut_user, Some(nut_password));
    let config = ConfigBuilder::new()
        .with_host((nut_host, nut_port).try_into().unwrap_or_default())
        .with_auth(Some(auth))
        .with_debug(false)
        .build();
    let mut nut_connection = Connection::new(&config).await?;
    Ok(nut_connection)
}

pub async fn mk_lib_hardware_nut_ups_list(
    mut nut_connection: Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    for (name, description) in nut_connection.list_ups().await? {
        println!("\t- Name: {}", name);
        println!("\t  Description: {}", description);
        println!(
            "\t  Number of logins: {}",
            nut_connection.get_num_logins(&name).await?
        );
        // Get list of mutable variables
        let mutable_vars = nut_connection.list_mutable_vars(&name).await?;

        // List UPS variables (key = val)
        println!("\t  Mutable Variables:");
        for var in mutable_vars.iter() {
            println!("\t\t- {}", var);
            println!(
                "\t\t  {:?}",
                nut_connection.get_var_type(&name, var.name()).await?
            );
        }

        // List UPS immutable properties (key = val)
        println!("\t  Immutable Properties:");
        for var in nut_connection.list_vars(&name).await? {
            if mutable_vars.iter().any(|v| v.name() == var.name()) {
                continue;
            }
            println!("\t\t- {}", var);
            println!(
                "\t\t  {:?}",
                nut_connection.get_var_type(&name, var.name()).await?
            );
        }

        // List UPS commands
        println!("\t  Commands:");
        for cmd in nut_connection.list_commands(&name).await? {
            let description = nut_connection.get_command_description(&name, &cmd).await?;
            println!("\t\t- {} ({})", cmd, description);
        }
    }
    Ok(())
}

pub async fn mk_lib_hardware_nut_connection_close(
    mut nut_connection: Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    nut_connection.close().await?;
    Ok(())
}
