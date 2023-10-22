use std::process::exit;
use std::sync::RwLock;
use std::time::Duration;

use lazy_static::lazy_static;
use tokio::sync::broadcast::Receiver;
use tokio::sync::mpsc as async_mpsc;
use tokio::sync::mpsc::Sender as AsyncSender;

use crate::common::Command;

#[allow(dead_code)]
mod common;
mod battery;
mod cpu;
mod date;
mod icon;
mod memory;
mod music;
mod net;
mod vol;
mod wifi;
mod bus;
#[allow(dead_code)]
mod theme;

lazy_static!(
    static ref BATTERY_TEXT: RwLock<String> = RwLock::new(String::new());
    static ref CPU_TEXT: RwLock<String> = RwLock::new(String::new());
    static ref DATE_TEXT: RwLock<String> = RwLock::new(String::new());
    static ref ICON_TEXT: RwLock<String> = RwLock::new(String::new());
    static ref MEMORY_TEXT: RwLock<String> = RwLock::new(String::new());
    static ref MUSIC_TEXT: RwLock<String> = RwLock::new(String::new());
    static ref NET_TEXT: RwLock<String> = RwLock::new(String::new());
    static ref VOL_TEXT: RwLock<String> = RwLock::new(String::new());
    static ref WIFI_TEXT: RwLock<String> = RwLock::new(String::new());
);

#[tokio::main]
async fn main() {
    // 判断是否带有参数启动
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        bus::send(args).await.expect("发送命令失败");
        return;
    }

    run().await;
}

async fn get_package_data(package: common::Package, tx: AsyncSender<common::PackageData>) {
    loop {
        let data = (package.fuc)();
        tx.send(data).await.expect("发送失败");
        tokio::time::sleep(package.delay_time).await;
    }
}

async fn receive_text(mut rx: async_mpsc::Receiver<common::PackageData>) {
    loop {
        let received = rx.recv().await;
        match received {
            None => { println!("什么也没有收到"); }
            Some(received) => {
                match received.module_name {
                    "battery" => { *BATTERY_TEXT.write().unwrap() = received.data }
                    "cpu" => { *CPU_TEXT.write().unwrap() = received.data }
                    "date" => { *DATE_TEXT.write().unwrap() = received.data }
                    "icon" => { *ICON_TEXT.write().unwrap() = received.data }
                    "memory" => { *MEMORY_TEXT.write().unwrap() = received.data }
                    "music" => { *MUSIC_TEXT.write().unwrap() = received.data }
                    "net" => { *NET_TEXT.write().unwrap() = received.data }
                    "vol" => { *VOL_TEXT.write().unwrap() = received.data }
                    "wifi" => { *WIFI_TEXT.write().unwrap() = received.data }
                    _ => {}
                }
            }
        }
    }
}

async fn receive_command(package: common::Package, mut rx: Receiver<Command>, tx: AsyncSender<common::PackageData>) {
    loop {
        let received = rx.recv().await.unwrap();
        if received.name == package.name {
            (package.control_fuc)(received.button);
            let data = (package.fuc)();
            tx.send(data).await.expect("发送失败");
        }
    }
}

async fn set_text() {
    loop {
        let tmp = format!(
            "{} {} {} {} {} {} {} {} {}",
            MUSIC_TEXT.read().unwrap(),
            WIFI_TEXT.read().unwrap(),
            NET_TEXT.read().unwrap(),
            CPU_TEXT.read().unwrap(),
            MEMORY_TEXT.read().unwrap(),
            VOL_TEXT.read().unwrap(),
            BATTERY_TEXT.read().unwrap(),
            DATE_TEXT.read().unwrap(),
            ICON_TEXT.read().unwrap(),
        );

        // println!("{}", tmp);

        // 执行系统命令 "xsetroot -name '" + str(tmp) + "'" 更新状态栏
        std::process::Command::new("xsetroot")
            .arg("-name")
            .arg(tmp)
            .output()
            .expect("failed to execute process");

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn run() {
    let (tx, rx) = async_mpsc::channel(32);

    let mut packages: Vec<common::Package> = Vec::new();

    packages.push(common::Package::new("battery", Duration::from_secs(10), battery::get, battery::api));
    packages.push(common::Package::new("cpu", Duration::from_secs(2), cpu::get, cpu::api));
    packages.push(common::Package::new("date", Duration::from_secs(1), date::get, date::api));
    packages.push(common::Package::new("icon", Duration::from_secs(100), icon::get, icon::api));
    packages.push(common::Package::new("memory", Duration::from_secs(2), memory::get, memory::api));
    packages.push(common::Package::new("music", Duration::from_secs(1), music::get, music::api));
    packages.push(common::Package::new("net", Duration::from_secs(1), net::get, net::api));
    packages.push(common::Package::new("vol", Duration::from_secs(1), vol::get, vol::api));
    packages.push(common::Package::new("wifi", Duration::from_secs(5), wifi::get, wifi::api));

    let mut tasks = Vec::new();

    let packages_clone = packages.clone();

    let (tx_command, _rx_command) = tokio::sync::broadcast::channel(32);
    for package in packages_clone {
        let rx_command_clone = tx_command.subscribe();
        let tx_clone = tx.clone();
        let task = tokio::spawn(async move {
            receive_command(package, rx_command_clone, tx_clone).await;
        });
        tasks.push(task);
    }

    let server_task = tokio::spawn(async move {
        match bus::server(tx_command).await {
            Ok(_) => {}
            Err(error) => {
                match error.to_string().as_str() {
                    "name already taken on the bus" => {
                        println!("已有相同程序正在运行");
                        exit(1)
                    }
                    _ => {
                        println!("Error: {}", error);
                    }
                }
            }
        };
    });

    for package in packages {
        let tx_clone = tx.clone();
        let task = tokio::spawn(async move {
            get_package_data(package, tx_clone).await;
        });
        tasks.push(task);
    }

    let print_task = tokio::spawn(async move {
        receive_text(rx).await;
    });

    let set_task = tokio::spawn(async move {
        set_text().await;
    });


    server_task.await.expect("bus服务启动失败");

    for task in tasks {
        task.await.expect("任务执行失败");
    }

    print_task.await.expect("打印任务执行失败");
    set_task.await.expect("设置任务执行失败");
}