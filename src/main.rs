
// =============================================================================
// FILE: src/main.rs
// =============================================================================
use tusks::tusks;

mod external_root;
mod external1;
mod external2;

#[tusks(root)]
pub mod tasks {
    pub struct Parameters<'a> {
        #[arg(long)]
        pub root_param: &'a String,
        
        #[arg(short, long)]
        pub verbose: &'a bool,
    }

    /// Task with Parameters argument - accesses root parameters
    pub fn task1(params: &Parameters, #[arg(long)] arg1: String) -> i32 {
        println!("=== root::task1 ===");
        println!("  root_param: {}", params.root_param);
        println!("  verbose: {}", params.verbose);
        println!("  arg1: {}", arg1);
        
        if *params.verbose {
            println!("  [VERBOSE] Executing task1 with root_param='{}'", params.root_param);
        }
        0
    }

    /// Task without Parameters argument, with return value
    pub fn task2(#[arg(short, long)] value: i32) -> i32 {
        println!("=== root::task2 ===");
        println!("  value: {}", value);
        println!("  returning: {}", value * 2);
        value * 2
    }

    /// Task with no arguments at all
    pub fn task3() {
        println!("=== root::task3 ===");
        println!("  No arguments, just executing");
    }

    /// Task with Vec parameter
    pub fn task4(
        params: &Parameters,
        #[arg(long)]
        items: Vec<String>
    ) {
        println!("=== root::task4 ===");
        println!("  root_param: {}", params.root_param);
        println!("  items: {:?}", items);
        println!("  item count: {}", items.len());
    }

    pub mod level1 {
        pub struct Parameters<'a> {
            #[arg(long)]
            pub level1_field: &'a Option<String>,
            
            #[arg(long, default_value = "42")]
            pub level1_number: &'a i32,
        }

        /// Subtask accessing both level1 and root parameters
        pub fn subtask1(
            params: &Parameters,
            #[arg(long)]
            arg: Option<String>
        ) -> u8 {
            println!("=== level1::subtask1 ===");
            println!("  level1_field: {:?}", params.level1_field);
            println!("  level1_number: {}", params.level1_number);
            println!("  arg: {:?}", arg);
            
            // Access parent parameters via super_
            println!("  root_param (via super_): {}", params.super_.root_param);
            println!("  verbose (via super_): {}", params.super_.verbose);
            
            if *params.super_.verbose {
                println!("  [VERBOSE] Level1 subtask1 accessing root_param='{}'", 
                         params.super_.root_param);
            }
            1
        }

        /// Subtask with only Parameters argument - demonstrates super_ access
        pub fn subtask2(params: &Parameters) {
            println!("=== level1::subtask2 ===");
            println!("  level1_field: {:?}", params.level1_field);
            println!("  Accessing root via super_:");
            println!("    root_param: {}", params.super_.root_param);
            println!("    verbose: {}", params.super_.verbose);
        }

        pub mod level2 {
            pub struct Parameters<'a> {
                #[arg(long)]
                pub level2_id: &'a u64,
            }

            /// Deep task accessing level2, level1, and root parameters
            pub fn deep_task(
                params: &Parameters,
                #[arg(long)] 
                enabled: bool
            ) {
                println!("=== level2::deep_task ===");
                println!("  level2_id: {}", params.level2_id);
                println!("  enabled: {}", enabled);
                
                // Access level1 parameters
                println!("  level1_field (via super_): {:?}", params.super_.level1_field);
                println!("  level1_number (via super_): {}", params.super_.level1_number);
                
                // Access root parameters via super_.super_
                println!("  root_param (via super_.super_): {}", params.super_.super_.root_param);
                println!("  verbose (via super_.super_): {}", params.super_.super_.verbose);
                
                if *params.super_.super_.verbose {
                    println!("  [VERBOSE] Deep in level2, can still access root!");
                }
            }

            pub mod level3 {
                /// Very deep task - no Parameters, but we could pass parent params
                pub fn very_deep(#[arg(long)] depth: u32) -> u32 {
                    println!("=== level3::very_deep ===");
                    println!("  depth: {}", depth);
                    println!("  Nested {} levels deep!", depth);
                    depth
                }
            }
        }

        // External module at level1
        pub use crate::external1::tasks as ext1;
    }

    pub mod level1b {
        // No Parameters struct in this module

        pub fn task_no_params(#[arg(long)] x: u8) -> i32 {
            println!("=== level1b::task_no_params ===");
            println!("  x: {}", x);
            println!("  No Parameters struct in this module");
            x as i32
        }

        pub fn task_multi_args(
            #[arg(short, long)]
            name: String,
            #[arg(long)]
            age: Option<i32>,
            #[arg(short)]
            active: bool,
            tags: Vec<String>
        ) {
            println!("=== level1b::task_multi_args ===");
            println!("  name: {}", name);
            println!("  age: {:?}", age);
            println!("  active: {}", active);
            println!("  tags: {:?}", tags);
        }
    }

    pub mod empty_module {
        // Module with only Parameters, no tasks
        pub struct Parameters<'a> {
            #[arg(long)]
            pub empty_field: &'a String,
        }
    }

    // External module at root level
    pub use crate::external_root::tasks as extroot;
}

fn main() {
    tasks::__internal_tusks_module::exec_cli();
}
