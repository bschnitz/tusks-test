use tusks::tusks;

pub mod holla;
pub mod compose;

#[tusks]
mod tasks {
    use tusks::RepeatMinMax;

    pub use crate::holla::tasks as holla;

    pub fn init(name: String) {
        println!("Initializing project: {}", name);
    }

    pub fn count(times: u8) {
        for i in 1..=times {
            println!("{}", i);
        }
    }

    pub fn optional(opt: Option<u16>) {
        match opt {
            Some(value) => println!("Value provided: {}", value),
            None => println!("No value provided"),
        }
    }

    pub fn sum(numbers: RepeatMinMax<u16, 2, 3>) {
        let sum: u32 = numbers.iter().map(|&n| n as u32).sum();
        println!("The sum of the numbers is {}", sum);
    }

    pub fn sum2(numbers: Vec<u16>) {
        let sum: u32 = numbers.iter().map(|&n| n as u32).sum();
        println!("The sum of the numbers is {}", sum);
    }

    #[positional(numbers)]
    pub fn sum3(numbers: Vec<u16>) {
        let sum: u32 = numbers.iter().map(|&n| n as u32).sum();
        println!("The sum of the numbers is {}", sum);
    }

    #[positional(positional)]
    pub fn positional(positional: String) {
        println!("The positional Argument is: {}", positional);
    }

    pub mod docker {
        pub use crate::compose::compose;

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
        pub fn push(branch: String, force: bool) {
            println!("Pushing to branch: {} (force: {})", branch, force);
        }
    }
}

fn main() {
    //let tree = tasks::__tusks_internal_module::get_tusks_tree();
    //println!("{:#?}", tree);
    //tasks::__tusks_internal_module::mirror_module::init("my project".into());
    //tasks::__tusks_internal_module::mirror_module::count("5".into());
    //tasks::__tusks_internal_module::mirror_module::optional(Some("42".into()));
    //tasks::__tusks_internal_module::mirror_module::optional(None);
    //tasks::__tusks_internal_module::mirror_module::git::push("main".into(), true);
    tasks::__tusks_internal_module::execute_cli();
    //tasks::__tusks_internal_module::mirror_module::git::push("hello".into(), Some("hello".into()));
    //tasks::holla::__tusks_internal_module::mirror_module::holla();
}
