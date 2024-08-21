
use std::path::Path;
use tokio;
use std::net::{Ipv4Addr, SocketAddrV4};
use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), String> {
    // let remote_host = "root@192.168.7.1";
    // let password = "p@ssw0rd";


    let local_file_path: &str = "/home/factorytb/Documents/testlogsave";//C:/Users
    let serverlog_file_path = "/var/log/journal/a4d8f7e56af544a5967a386b8f483060";
    let canlog_file_path = "/usr/share/hmi-logging/log";

    if let Err(err) = copy_files_from_remote(serverlog_file_path, local_file_path) {
        error!("Failed to copy serverlogs from IMX: {:?}", err);
        return Err(format!("Failed to copy serverlogs from IMX: {:?}", err));
    }
    if let Err(err) = copy_files_from_remote(&canlog_file_path, local_file_path) {
        error!("Failed to copy canlogs from IMX: {:?}", err);
        return Err(format!("Failed to copy canlogs from IMX: {:?}", err));
    }
    
    println!("File downloaded successfully!");
    Ok(())
}

pub fn copy_files_from_remote(source: &str, dest: &str) -> Result<(), String> {
    // Create new session for ssh
    let localhost = Ipv4Addr::new(192, 168, 7, 1);
    let socket_v4 = SocketAddrV4::new(localhost, 22);
    let ssh_connector = ssh::create_session()
        .username("root")
        .password("p@ssw0rd")
        .connect(socket_v4);

    match ssh_connector {
        Ok(_) => {
            info!("Connect to 192.168.7.1 successfully!");
        }
        Err(e) => {
            error!("Failed to ssh to 192.168.7.1, error {}", e);
            return Err(format!("Failed to ssh to 192.168.7.1, error: {}", e));
        }
    }
    let mut ssh_section = ssh_connector.unwrap().run_local();
    let scp = ssh_section.open_scp().unwrap();
    match scp.download(source, dest) {
        Ok(_) => {
            info!("scp from {} to {} executed successfully!", source, dest);
            Ok(())
        }
        Err(_) => {
            error!("scp from {} to {} Failed!", source, dest);
            Err(format!("scp from {} to {} Failed!", source, dest))
        }
    }
}

// pub fn remote_command_execute(cmd: &str) -> Result<(), String> {
//     // Create new session for ssh
//     let localhost = Ipv4Addr::new(192, 168, 7, 1);
//     let socket_v4 = SocketAddrV4::new(localhost, 22);
//     let ssh_connector = ssh::create_session()
//         .username("root")
//         .password("")
//         .connect(socket_v4);

//     match ssh_connector {
//         Ok(_) => {
//             info!(
//                 "Connect to 192.168.7.1 successfully! \nRun command: {}",
//                 cmd
//             );
//         }
//         Err(e) => {
//             error!("Failed to ssh to 192.168.7.1, error {}", e);
//             return Err(format!("Failed to ssh to 192.168.7.1, error: {}", e));
//         }
//     }

//     let mut ssh_section = ssh_connector.unwrap().run_local();

//     let mut exec = ssh_section.open_exec().unwrap();
//     exec.exec_command(cmd).unwrap();

//     let vec = exec.get_output().unwrap();
//     info!("{}", String::from_utf8(vec).unwrap());

//     let result = exec.exit_status().unwrap();
//     match result {
//         0 => {
//             info!("command \"{}\" executed successfully!", cmd);
//             Ok(())
//         }
//         _ => {
//             error!("command \"{}\" executed failed with code {}!", cmd, result);
//             Err(format!(
//                 "command \"{}\" executed failed with code {}!",
//                 cmd, result
//             ))
//         }
//     }
// }