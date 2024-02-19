use std::process::Command;

const EXE_PATH: &str = ""; //dummy-exe

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	//wait until global network connected
	let client = reqwest::Client::new();
	let mut req_count = 0;
  while req_count < u64::MAX {
		req_count+=1;
    match client.get("http://1.1.1.1:80").send().await {
			Ok(res) => {
				if res.status().is_success() {
						//cushion
					tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
					//call exe
					call_exe().await;
					break;
				} else {
					println!("code: {}",res.status());
				}
			}
			Err(_) => {
				println!("Connection Failed.");
			}
		}
		println!("loop.");
		tokio::time::sleep(tokio::time::Duration::from_secs(req_count)).await;
  }
	Ok(())
}

async fn call_exe() {
	println!("finish");
	Command::new(EXE_PATH).output().expect("failed to execute exe");
}
