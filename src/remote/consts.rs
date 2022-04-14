use crate::sleep_for;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref IS_ONLINE: bool = {
        let client = reqwest::blocking::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap();

        let res = if let Ok(req) = client.get("https://github.com").send() {
            drop(client);
            req.status().is_success()
        } else {
            false
        };

        if !res {
            use colored::Colorize;
            println!("{}","warning: you are offline. you will only be able to use cached templates which may be out of date".yellow());
            sleep_for!(3000);
        }

        res
    };
}
