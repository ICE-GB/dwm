use zbus::Connection;
use std::future::pending;
use zbus::{ConnectionBuilder, dbus_interface};

const DESTINATION: &str = "org.dwm.statusbar.rust";
const PATH: &str = "/org/dwm/statusbar/rust";


struct Greeter {
    count: u64
}

struct Action {
    name: String,
    button: String,
}

#[dbus_interface(name = "org.dwm.statusbar.rust.greeter")]
impl Greeter {
    async fn say_hello(&mut self, name: &str) -> String {
        self.count += 1;
        format!("Hello {}! I have been called {} times.", name, self.count)
    }
}

#[dbus_interface(name = "org.dwm.statusbar.rust.action")]
impl Action {
    async fn do_action(&mut self, name: String, button: String) -> String {
        format!("call {} by button {}.", name, button)
    }
}

pub async fn server() -> Result<()>{
    let greeter = Greeter { count: 0 };
    let action = Action{ name: String::from("action"), button: String::from("button") };

    let _conn = ConnectionBuilder::session()?
        .name(DESTINATION)?
        .serve_at(PATH, greeter)?
        .serve_at(PATH, action)?
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;
    Ok(())
}

use zbus::{Result, dbus_proxy};

#[dbus_proxy(
interface = "org.dwm.statusbar.rust.greeter",
default_service = "org.dwm.statusbar.rust",
default_path = "/org/dwm/statusbar/rust"
)]
trait MyGreeter {
    async fn say_hello(&self, name: &str) -> Result<String>;
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

    // `dbus_proxy` macro creates `MyGreaterProxy` based on `Notifications` trait.
    let proxy = MyGreeterProxy::new(&connection).await?;
    let reply = proxy.say_hello(args[1].as_ref()).await?;
    println!("{reply}");

    let proxy = ActionSenderProxy::new(&connection).await?;
    let reply = proxy.do_action(args[1].as_ref(), args[2].as_ref()).await?;
    println!("{reply}");

    Ok(())
}


