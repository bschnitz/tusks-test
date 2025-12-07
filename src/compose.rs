use tusks::tusks;

#[tusks]
pub mod compose {
    pub fn up(detached: Option<bool>) {
        let detached_flag = detached.unwrap_or(false);
        println!("Starting containers (detached: {})", detached_flag);
    }
    
    pub fn down(force: bool) {
        println!("Stopping containers (force: {})", force);
    }
    
    pub mod services {
        pub fn list() {
            println!("Listing all services");
        }
        
        #[defaults(replicas="1")]
        pub fn scale(service: String, replicas: String) {
            println!("Scaling {} to {} replicas", service, replicas);
        }
        
        pub fn logs(service: String, follow: bool) {
            println!("Showing logs for {} (follow: {})", service, follow);
        }
    }
}
