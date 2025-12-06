use tusks::tusks;

#[tusks]
mod project {
    pub fn init(name: String) {
        println!("Initializing project: {}", name);
    }

    pub mod docker {
        #[defaults(tag="latest")]
        pub fn build(tag: String) {
            println!("Building docker image with tag: {}", tag);
        }
        
        #[defaults(registry="latest")]
        pub fn push(registry: String, image: String) {
            println!("Pushing {}to {}", image, registry);
        }
    }
    
    pub mod git {
        pub fn commit(message: String) {
            println!("Committing: {}", message);
        }
        
        #[defaults(branch="main")]
        pub fn push(branch: String, force: Option<bool>) {
            let force_flag = force.unwrap_or(false);
            println!("Pushing to branch: {} (force: {})", branch, force_flag);
        }
    }
}

fn main() {
}
