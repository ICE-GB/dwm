use std::future::pending;

use zbus::{ConnectionBuilder, dbus_interface};
use zbus::{dbus_proxy, Result};
use zbus::Connection;

use crate::common::Button;
use crate::common::Command;

const DESTINATION: &str = "org.dwm.statusbar.rust";
const PATH: &str = "/org/dwm/statusbar/rust";


struct TxSender {
    tx: tokio::sync::broadcast::Sender<Command>,
}


#[dbus_interface(name = "org.dwm.statusbar.rust.action")]
impl TxSender {
    fn do_action(&mut self, name: String, button: String) -> String {
        let text =
            format!("call {} by button {}.", &name, &button);
        self.tx.send(Command::new(name, Button::from_str(&button))).expect("发送失败");
        text
    }
}

pub async fn server(tx: tokio::sync::broadcast::Sender<Command>) -> Result<()> {
    let tx_sender = TxSender { tx };

    let _conn = ConnectionBuilder::session()?
        .name(DESTINATION)?
        .serve_at(PATH, tx_sender)?
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;
    Ok(())
}


#[dbus_proxy(
interface = "org.dwm.statusbar.rust.action",
default_service = "org.dwm.statusbar.rust",
default_path = "/org/dwm/statusbar/rust"
)]
trait ActionSender {
    async fn do_action(&self, name: &str, button: &str) -> Result<String>;
}


pub async fn send(args: Vec<String>) -> Result<()> {
    let connection = Connection::session().await?;
    let proxy = ActionSenderProxy::new(&connection).await?;
    let reply = proxy.do_action(args[1].as_ref(), args[2].as_ref()).await?;
    println!("{reply}");

    Ok(())
}


