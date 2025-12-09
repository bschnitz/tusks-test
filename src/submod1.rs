use tusks::tusks;

#[tusks]
pub mod tasks {
    pub use crate::submod2::submod2;

    pub fn t_submod1() {
        println!("Submodule 1");
    }
}
