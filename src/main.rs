slint::include_modules!();
use slint::{Timer, TimerMode};
use adb_client::AdbTcpConnection;
use std::net::Ipv4Addr;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let timer = Timer::default();
    
    ui.on_start_stop_mining({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            if ui.get_is_mining() {
                timer.stop();
            } else {
                let mut connection = 
                    AdbTcpConnection::new(Ipv4Addr::from([127,0,0,1]), 5037).unwrap();
                timer.start(TimerMode::Repeated, std::time::Duration::from_millis(1500), move || {
                    let str_commands = "input tap 270 940 && input tap 540 940 && input tap 810 940 && input tap 270 1040 && input tap 540 1040 && input tap 810 1040 && input tap 270 1140 && input tap 540 1140 && input tap 810 1140 && input tap 270 1340 && input tap 540 1340 && input tap 810 1340 && input tap 370 1440 && input tap 540 1440 && input tap 710 1440";
                    let commands: Vec<&str> = str_commands.split_whitespace().collect();
                    let _ = connection.shell_command(&None, commands);
                });
                
            }
            ui.set_is_mining(!ui.get_is_mining());
            
        }
    });
    

    ui.run()
}
