use tusks::tusks;

pub mod holla;
pub mod docker;

#[tusks]
mod tasks {
    pub fn hello() {
        println!("Hello Worlds!");
    }

    pub use crate::holla;
    pub use crate::docker;
}

fn main() {
}
