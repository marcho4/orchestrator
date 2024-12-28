use actix_web::HttpResponse;
use crate::orchestrator::orchestrator::Orchestrator;


impl Orchestrator {
    pub async fn process_login(&self, wallet: String) -> HttpResponse {
        let memberships_url = format!("http://license_service:8001/license/all/{}", wallet);
        let communities_url = format!("http://community_service:8003/community/all/{}", wallet);

        let memberships = self.client.get(memberships_url).send().await.unwrap();
        let ownerships = self.client.get(communities_url).send().await.unwrap();
        println!("{:?}", ownerships);
        println!("{:?}", memberships);

        // request to jwt

        // Ok with JWT token
        let jwt: String = "asdasd".to_string();

        HttpResponse::Ok().body(jwt)
    }
}