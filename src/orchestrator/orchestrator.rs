use reqwest::Client;

pub struct Orchestrator {
    pub client: reqwest::Client,
}
impl Orchestrator {
    pub fn new() -> Orchestrator {
        Orchestrator {client: Client::new()}
    }
}