#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use tusks::tusks;
pub mod holla {
    use tusks::tusks;
    pub mod tasks {
        pub fn holla() {
            {
                ::std::io::_print(format_args!("Holla Worlds!\n"));
            };
        }
        pub mod __tusks_internal_module {
            use tusks::{TusksNode, Tusk, Argument, LinkNode};
            use std::collections::HashMap;
            pub fn get_tusks_tree() -> TusksNode {
                {
                    let mut node = tusks::TusksNode {
                        module_path: <[_]>::into_vec(
                            ::alloc::boxed::box_new(["tasks".to_string()]),
                        ),
                        tusks: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                tusks::Tusk {
                                    name: "holla".to_string(),
                                    arguments: {
                                        let mut map = tusks::indexmap::IndexMap::new();
                                        map
                                    },
                                },
                            ]),
                        ),
                        childs: ::alloc::vec::Vec::new(),
                        links: ::alloc::vec::Vec::new(),
                        is_link: false,
                        link_name: None,
                    };
                    node
                }
            }
            pub mod mirror_module {}
            pub fn execute_cli() {
                let mut command = clap::Command::new("tusks")
                    .version("1.0")
                    .about("Task runner");
                command = build_cli(command, Vec::new());
                let matches = command.get_matches();
                handle_matches(&matches, Vec::new());
            }
            pub fn build_cli(
                mut command: clap::Command,
                path_prefix: Vec<String>,
            ) -> clap::Command {
                let subcommand = {
                    let cmd_name = {
                        let mut parts = path_prefix.clone();
                        parts.push("holla".to_string());
                        parts.join(".")
                    };
                    let mut cmd = clap::Command::new(cmd_name);
                    cmd
                };
                command = command.subcommand(subcommand);
                command
            }
            pub fn handle_matches(
                matches: &clap::ArgMatches,
                link_path: Vec<String>,
            ) -> bool {
                let (subcommand_name, sub_matches) = matches.subcommand().unwrap();
                let subcommand_prefix = link_path.join(".");
                if !subcommand_prefix.is_empty()
                    && !subcommand_name.starts_with(&subcommand_prefix)
                {
                    return false;
                }
                let actual_command = if !subcommand_prefix.is_empty() {
                    let prefix_len = subcommand_prefix.len() + ".".len();
                    &subcommand_name[prefix_len..]
                } else {
                    subcommand_name
                };
                match actual_command {
                    "holla" => {
                        super::holla();
                        return true;
                    }
                    _ => {
                        {
                            ::std::io::_eprint(
                                format_args!("Unknown command: {0}\n", subcommand_name),
                            );
                        };
                        return true;
                    }
                }
            }
        }
    }
}
pub mod compose {
    use tusks::tusks;
    pub mod compose {
        pub fn up(detached: bool) {
            {
                ::std::io::_print(
                    format_args!("Starting containers (detached: {0})\n", detached),
                );
            };
        }
        pub fn down(force: bool) {
            {
                ::std::io::_print(
                    format_args!("Stopping containers (force: {0})\n", force),
                );
            };
        }
        pub mod services {
            pub fn list() {
                {
                    ::std::io::_print(format_args!("Listing all services\n"));
                };
            }
            pub fn scale(service: String, replicas: String) {
                {
                    ::std::io::_print(
                        format_args!("Scaling {0} to {1} replicas\n", service, replicas),
                    );
                };
            }
            pub fn logs(service: String, follow: bool) {
                {
                    ::std::io::_print(
                        format_args!(
                            "Showing logs for {0} (follow: {1})\n",
                            service,
                            follow,
                        ),
                    );
                };
            }
        }
        pub mod __tusks_internal_module {
            use tusks::{TusksNode, Tusk, Argument, LinkNode};
            use std::collections::HashMap;
            pub fn get_tusks_tree() -> TusksNode {
                {
                    let mut node = tusks::TusksNode {
                        module_path: <[_]>::into_vec(
                            ::alloc::boxed::box_new(["compose".to_string()]),
                        ),
                        tusks: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                tusks::Tusk {
                                    name: "up".to_string(),
                                    arguments: {
                                        let mut map = tusks::indexmap::IndexMap::new();
                                        map.insert(
                                            "detached".to_string(),
                                            tusks::Argument {
                                                name: "detached".to_string(),
                                                type_: "bool".to_string(),
                                                default: None,
                                                optional: false,
                                                flag: true,
                                                positional: false,
                                                count: None,
                                                short: None,
                                                help: None,
                                                hidden: false,
                                                value_hint: None,
                                                arg_enum: None,
                                                validator: None,
                                                arg: None,
                                            },
                                        );
                                        map
                                    },
                                },
                                tusks::Tusk {
                                    name: "down".to_string(),
                                    arguments: {
                                        let mut map = tusks::indexmap::IndexMap::new();
                                        map.insert(
                                            "force".to_string(),
                                            tusks::Argument {
                                                name: "force".to_string(),
                                                type_: "bool".to_string(),
                                                default: None,
                                                optional: false,
                                                flag: true,
                                                positional: false,
                                                count: None,
                                                short: None,
                                                help: None,
                                                hidden: false,
                                                value_hint: None,
                                                arg_enum: None,
                                                validator: None,
                                                arg: None,
                                            },
                                        );
                                        map
                                    },
                                },
                            ]),
                        ),
                        childs: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                {
                                    let mut node = tusks::TusksNode {
                                        module_path: <[_]>::into_vec(
                                            ::alloc::boxed::box_new([
                                                "compose".to_string(),
                                                "services".to_string(),
                                            ]),
                                        ),
                                        tusks: <[_]>::into_vec(
                                            ::alloc::boxed::box_new([
                                                tusks::Tusk {
                                                    name: "list".to_string(),
                                                    arguments: {
                                                        let mut map = tusks::indexmap::IndexMap::new();
                                                        map
                                                    },
                                                },
                                                tusks::Tusk {
                                                    name: "scale".to_string(),
                                                    arguments: {
                                                        let mut map = tusks::indexmap::IndexMap::new();
                                                        map.insert(
                                                            "service".to_string(),
                                                            tusks::Argument {
                                                                name: "service".to_string(),
                                                                type_: "String".to_string(),
                                                                default: None,
                                                                optional: false,
                                                                flag: false,
                                                                positional: false,
                                                                count: None,
                                                                short: None,
                                                                help: None,
                                                                hidden: false,
                                                                value_hint: None,
                                                                arg_enum: None,
                                                                validator: None,
                                                                arg: None,
                                                            },
                                                        );
                                                        map.insert(
                                                            "replicas".to_string(),
                                                            tusks::Argument {
                                                                name: "replicas".to_string(),
                                                                type_: "String".to_string(),
                                                                default: Some("1".to_string()),
                                                                optional: false,
                                                                flag: false,
                                                                positional: false,
                                                                count: None,
                                                                short: None,
                                                                help: None,
                                                                hidden: false,
                                                                value_hint: None,
                                                                arg_enum: None,
                                                                validator: None,
                                                                arg: None,
                                                            },
                                                        );
                                                        map
                                                    },
                                                },
                                                tusks::Tusk {
                                                    name: "logs".to_string(),
                                                    arguments: {
                                                        let mut map = tusks::indexmap::IndexMap::new();
                                                        map.insert(
                                                            "service".to_string(),
                                                            tusks::Argument {
                                                                name: "service".to_string(),
                                                                type_: "String".to_string(),
                                                                default: None,
                                                                optional: false,
                                                                flag: false,
                                                                positional: false,
                                                                count: None,
                                                                short: None,
                                                                help: None,
                                                                hidden: false,
                                                                value_hint: None,
                                                                arg_enum: None,
                                                                validator: None,
                                                                arg: None,
                                                            },
                                                        );
                                                        map.insert(
                                                            "follow".to_string(),
                                                            tusks::Argument {
                                                                name: "follow".to_string(),
                                                                type_: "bool".to_string(),
                                                                default: None,
                                                                optional: false,
                                                                flag: true,
                                                                positional: false,
                                                                count: None,
                                                                short: None,
                                                                help: None,
                                                                hidden: false,
                                                                value_hint: None,
                                                                arg_enum: None,
                                                                validator: None,
                                                                arg: None,
                                                            },
                                                        );
                                                        map
                                                    },
                                                },
                                            ]),
                                        ),
                                        childs: ::alloc::vec::Vec::new(),
                                        links: ::alloc::vec::Vec::new(),
                                        is_link: false,
                                        link_name: None,
                                    };
                                    node
                                },
                            ]),
                        ),
                        links: ::alloc::vec::Vec::new(),
                        is_link: false,
                        link_name: None,
                    };
                    node
                }
            }
            pub mod mirror_module {}
            pub fn execute_cli() {
                let mut command = clap::Command::new("tusks")
                    .version("1.0")
                    .about("Task runner");
                command = build_cli(command, Vec::new());
                let matches = command.get_matches();
                handle_matches(&matches, Vec::new());
            }
            pub fn build_cli(
                mut command: clap::Command,
                path_prefix: Vec<String>,
            ) -> clap::Command {
                let subcommand = {
                    let cmd_name = {
                        let mut parts = path_prefix.clone();
                        parts.push("up".to_string());
                        parts.join(".")
                    };
                    let mut cmd = clap::Command::new(cmd_name);
                    cmd = cmd
                        .arg(
                            clap::Arg::new("detached")
                                .action(clap::ArgAction::SetTrue)
                                .long("detached"),
                        );
                    cmd
                };
                command = command.subcommand(subcommand);
                let subcommand = {
                    let cmd_name = {
                        let mut parts = path_prefix.clone();
                        parts.push("down".to_string());
                        parts.join(".")
                    };
                    let mut cmd = clap::Command::new(cmd_name);
                    cmd = cmd
                        .arg(
                            clap::Arg::new("force")
                                .action(clap::ArgAction::SetTrue)
                                .long("force"),
                        );
                    cmd
                };
                command = command.subcommand(subcommand);
                let mut child_prefix = path_prefix.clone();
                child_prefix.push("services".to_string());
                let subcommand = {
                    let cmd_name = {
                        let mut parts = child_prefix.clone();
                        parts.push("list".to_string());
                        parts.join(".")
                    };
                    let mut cmd = clap::Command::new(cmd_name);
                    cmd
                };
                command = command.subcommand(subcommand);
                let subcommand = {
                    let cmd_name = {
                        let mut parts = child_prefix.clone();
                        parts.push("scale".to_string());
                        parts.join(".")
                    };
                    let mut cmd = clap::Command::new(cmd_name);
                    cmd = cmd
                        .arg(
                            clap::Arg::new("service")
                                .value_name("SERVICE")
                                .value_parser({
                                    use ::clap_builder::builder::impl_prelude::*;
                                    let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .required(true)
                                .long("service"),
                        );
                    cmd = cmd
                        .arg(
                            clap::Arg::new("replicas")
                                .value_name("REPLICAS")
                                .value_parser({
                                    use ::clap_builder::builder::impl_prelude::*;
                                    let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .default_value("1")
                                .long("replicas"),
                        );
                    cmd
                };
                command = command.subcommand(subcommand);
                let subcommand = {
                    let cmd_name = {
                        let mut parts = child_prefix.clone();
                        parts.push("logs".to_string());
                        parts.join(".")
                    };
                    let mut cmd = clap::Command::new(cmd_name);
                    cmd = cmd
                        .arg(
                            clap::Arg::new("service")
                                .value_name("SERVICE")
                                .value_parser({
                                    use ::clap_builder::builder::impl_prelude::*;
                                    let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .required(true)
                                .long("service"),
                        );
                    cmd = cmd
                        .arg(
                            clap::Arg::new("follow")
                                .action(clap::ArgAction::SetTrue)
                                .long("follow"),
                        );
                    cmd
                };
                command = command.subcommand(subcommand);
                command
            }
            pub fn handle_matches(
                matches: &clap::ArgMatches,
                link_path: Vec<String>,
            ) -> bool {
                let (subcommand_name, sub_matches) = matches.subcommand().unwrap();
                let subcommand_prefix = link_path.join(".");
                if !subcommand_prefix.is_empty()
                    && !subcommand_name.starts_with(&subcommand_prefix)
                {
                    return false;
                }
                let actual_command = if !subcommand_prefix.is_empty() {
                    let prefix_len = subcommand_prefix.len() + ".".len();
                    &subcommand_name[prefix_len..]
                } else {
                    subcommand_name
                };
                match actual_command {
                    "up" => {
                        let arg_0 = matches.subcommand().unwrap().1.get_flag("detached");
                        super::up(arg_0);
                        return true;
                    }
                    "down" => {
                        let arg_0 = matches.subcommand().unwrap().1.get_flag("force");
                        super::down(arg_0);
                        return true;
                    }
                    "services.list" => {
                        super::services::list();
                        return true;
                    }
                    "services.scale" => {
                        let arg_0 = matches
                            .subcommand()
                            .unwrap()
                            .1
                            .get_one::<String>("service")
                            .unwrap()
                            .clone();
                        let arg_1 = matches
                            .subcommand()
                            .unwrap()
                            .1
                            .get_one::<String>("replicas")
                            .map(|s| s.clone())
                            .unwrap_or_else(|| "1".to_string());
                        super::services::scale(arg_0, arg_1);
                        return true;
                    }
                    "services.logs" => {
                        let arg_0 = matches
                            .subcommand()
                            .unwrap()
                            .1
                            .get_one::<String>("service")
                            .unwrap()
                            .clone();
                        let arg_1 = matches.subcommand().unwrap().1.get_flag("follow");
                        super::services::logs(arg_0, arg_1);
                        return true;
                    }
                    _ => {
                        {
                            ::std::io::_eprint(
                                format_args!("Unknown command: {0}\n", subcommand_name),
                            );
                        };
                        return true;
                    }
                }
            }
        }
    }
}
mod tasks {
    use tusks::RepeatMinMax;
    pub use crate::holla::tasks as holla;
    pub fn init(name: String) {
        {
            ::std::io::_print(format_args!("Initializing project: {0}\n", name));
        };
    }
    pub fn count(times: u8) {
        for i in 1..=times {
            {
                ::std::io::_print(format_args!("{0}\n", i));
            };
        }
    }
    pub fn optional(opt: Option<u16>) {
        match opt {
            Some(value) => {
                ::std::io::_print(format_args!("Value provided: {0}\n", value));
            }
            None => {
                ::std::io::_print(format_args!("No value provided\n"));
            }
        }
    }
    pub fn sum(numbers: RepeatMinMax<u16, 2, 3>) {
        let sum: u32 = numbers.iter().map(|&n| n as u32).sum();
        {
            ::std::io::_print(format_args!("The sum of the numbers is {0}\n", sum));
        };
    }
    pub fn sum2(numbers: Vec<u16>) {
        let sum: u32 = numbers.iter().map(|&n| n as u32).sum();
        {
            ::std::io::_print(format_args!("The sum of the numbers is {0}\n", sum));
        };
    }
    pub fn sum3(numbers: Vec<u16>) {
        let sum: u32 = numbers.iter().map(|&n| n as u32).sum();
        {
            ::std::io::_print(format_args!("The sum of the numbers is {0}\n", sum));
        };
    }
    pub fn positional(positional: String) {
        {
            ::std::io::_print(
                format_args!("The positional Argument is: {0}\n", positional),
            );
        };
    }
    pub mod docker {
        pub use crate::compose::compose;
        pub fn build(tag: String) {
            {
                ::std::io::_print(
                    format_args!("Building docker image with tag: {0}\n", tag),
                );
            };
        }
        pub fn push(registry: String, image: String) {
            {
                ::std::io::_print(format_args!("Pushing {0}to {1}\n", image, registry));
            };
        }
    }
    pub mod git {
        pub fn commit(message: String) {
            {
                ::std::io::_print(format_args!("Committing: {0}\n", message));
            };
        }
        pub fn push(branch: String, force: bool) {
            {
                ::std::io::_print(
                    format_args!("Pushing to branch: {0} (force: {1})\n", branch, force),
                );
            };
        }
    }
    pub fn t_string(v_string: String) {
        {
            ::std::io::_print(format_args!("{0}\n", v_string));
        };
    }
    pub fn t_int(v_int: i8) {
        {
            ::std::io::_print(format_args!("{0}\n", v_int));
        };
    }
    pub fn t_optional(v_opt: Option<u8>) {
        {
            ::std::io::_print(format_args!("{0:?}\n", v_opt));
        };
    }
    pub mod __tusks_internal_module {
        use tusks::{TusksNode, Tusk, Argument, LinkNode};
        use std::collections::HashMap;
        pub fn get_tusks_tree() -> TusksNode {
            {
                let mut node = tusks::TusksNode {
                    module_path: <[_]>::into_vec(
                        ::alloc::boxed::box_new(["tasks".to_string()]),
                    ),
                    tusks: <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            tusks::Tusk {
                                name: "init".to_string(),
                                arguments: {
                                    let mut map = tusks::indexmap::IndexMap::new();
                                    map.insert(
                                        "name".to_string(),
                                        tusks::Argument {
                                            name: "name".to_string(),
                                            type_: "String".to_string(),
                                            default: None,
                                            optional: false,
                                            flag: false,
                                            positional: false,
                                            count: None,
                                            short: None,
                                            help: None,
                                            hidden: false,
                                            value_hint: None,
                                            arg_enum: None,
                                            validator: None,
                                            arg: None,
                                        },
                                    );
                                    map
                                },
                            },
                            tusks::Tusk {
                                name: "count".to_string(),
                                arguments: {
                                    let mut map = tusks::indexmap::IndexMap::new();
                                    map.insert(
                                        "times".to_string(),
                                        tusks::Argument {
                                            name: "times".to_string(),
                                            type_: "u8".to_string(),
                                            default: None,
                                            optional: false,
                                            flag: false,
                                            positional: false,
                                            count: None,
                                            short: None,
                                            help: None,
                                            hidden: false,
                                            value_hint: None,
                                            arg_enum: None,
                                            validator: None,
                                            arg: None,
                                        },
                                    );
                                    map
                                },
                            },
                            tusks::Tusk {
                                name: "optional".to_string(),
                                arguments: {
                                    let mut map = tusks::indexmap::IndexMap::new();
                                    map.insert(
                                        "opt".to_string(),
                                        tusks::Argument {
                                            name: "opt".to_string(),
                                            type_: "u16".to_string(),
                                            default: None,
                                            optional: true,
                                            flag: false,
                                            positional: false,
                                            count: None,
                                            short: None,
                                            help: None,
                                            hidden: false,
                                            value_hint: None,
                                            arg_enum: None,
                                            validator: None,
                                            arg: None,
                                        },
                                    );
                                    map
                                },
                            },
                            tusks::Tusk {
                                name: "sum".to_string(),
                                arguments: {
                                    let mut map = tusks::indexmap::IndexMap::new();
                                    map.insert(
                                        "numbers".to_string(),
                                        tusks::Argument {
                                            name: "numbers".to_string(),
                                            type_: "u16".to_string(),
                                            default: None,
                                            optional: false,
                                            flag: false,
                                            positional: false,
                                            count: Some(tusks::ArgumentMultiplicity {
                                                min: Some(2),
                                                max: Some(3),
                                            }),
                                            short: None,
                                            help: None,
                                            hidden: false,
                                            value_hint: None,
                                            arg_enum: None,
                                            validator: None,
                                            arg: None,
                                        },
                                    );
                                    map
                                },
                            },
                            tusks::Tusk {
                                name: "sum2".to_string(),
                                arguments: {
                                    let mut map = tusks::indexmap::IndexMap::new();
                                    map.insert(
                                        "numbers".to_string(),
                                        tusks::Argument {
                                            name: "numbers".to_string(),
                                            type_: "u16".to_string(),
                                            default: None,
                                            optional: false,
                                            flag: false,
                                            positional: false,
                                            count: Some(tusks::ArgumentMultiplicity {
                                                min: None,
                                                max: None,
                                            }),
                                            short: None,
                                            help: None,
                                            hidden: false,
                                            value_hint: None,
                                            arg_enum: None,
                                            validator: None,
                                            arg: None,
                                        },
                                    );
                                    map
                                },
                            },
                            tusks::Tusk {
                                name: "sum3".to_string(),
                                arguments: {
                                    let mut map = tusks::indexmap::IndexMap::new();
                                    map.insert(
                                        "numbers".to_string(),
                                        tusks::Argument {
                                            name: "numbers".to_string(),
                                            type_: "u16".to_string(),
                                            default: None,
                                            optional: false,
                                            flag: false,
                                            positional: true,
                                            count: Some(tusks::ArgumentMultiplicity {
                                                min: None,
                                                max: None,
                                            }),
                                            short: None,
                                            help: None,
                                            hidden: false,
                                            value_hint: None,
                                            arg_enum: None,
                                            validator: None,
                                            arg: None,
                                        },
                                    );
                                    map
                                },
                            },
                            tusks::Tusk {
                                name: "positional".to_string(),
                                arguments: {
                                    let mut map = tusks::indexmap::IndexMap::new();
                                    map.insert(
                                        "positional".to_string(),
                                        tusks::Argument {
                                            name: "positional".to_string(),
                                            type_: "String".to_string(),
                                            default: None,
                                            optional: false,
                                            flag: false,
                                            positional: true,
                                            count: None,
                                            short: None,
                                            help: None,
                                            hidden: false,
                                            value_hint: None,
                                            arg_enum: None,
                                            validator: None,
                                            arg: None,
                                        },
                                    );
                                    map
                                },
                            },
                            tusks::Tusk {
                                name: "t_string".to_string(),
                                arguments: {
                                    let mut map = tusks::indexmap::IndexMap::new();
                                    map.insert(
                                        "v_string".to_string(),
                                        tusks::Argument {
                                            name: "v_string".to_string(),
                                            type_: "String".to_string(),
                                            default: None,
                                            optional: false,
                                            flag: false,
                                            positional: false,
                                            count: None,
                                            short: None,
                                            help: None,
                                            hidden: false,
                                            value_hint: None,
                                            arg_enum: None,
                                            validator: None,
                                            arg: None,
                                        },
                                    );
                                    map
                                },
                            },
                            tusks::Tusk {
                                name: "t_int".to_string(),
                                arguments: {
                                    let mut map = tusks::indexmap::IndexMap::new();
                                    map.insert(
                                        "v_int".to_string(),
                                        tusks::Argument {
                                            name: "v_int".to_string(),
                                            type_: "i8".to_string(),
                                            default: None,
                                            optional: false,
                                            flag: false,
                                            positional: false,
                                            count: None,
                                            short: None,
                                            help: None,
                                            hidden: false,
                                            value_hint: None,
                                            arg_enum: None,
                                            validator: None,
                                            arg: None,
                                        },
                                    );
                                    map
                                },
                            },
                            tusks::Tusk {
                                name: "t_optional".to_string(),
                                arguments: {
                                    let mut map = tusks::indexmap::IndexMap::new();
                                    map.insert(
                                        "v_opt".to_string(),
                                        tusks::Argument {
                                            name: "v_opt".to_string(),
                                            type_: "u8".to_string(),
                                            default: None,
                                            optional: true,
                                            flag: false,
                                            positional: false,
                                            count: None,
                                            short: None,
                                            help: None,
                                            hidden: false,
                                            value_hint: None,
                                            arg_enum: None,
                                            validator: None,
                                            arg: None,
                                        },
                                    );
                                    map
                                },
                            },
                        ]),
                    ),
                    childs: <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            {
                                let mut node = tusks::TusksNode {
                                    module_path: <[_]>::into_vec(
                                        ::alloc::boxed::box_new([
                                            "tasks".to_string(),
                                            "docker".to_string(),
                                        ]),
                                    ),
                                    tusks: <[_]>::into_vec(
                                        ::alloc::boxed::box_new([
                                            tusks::Tusk {
                                                name: "build".to_string(),
                                                arguments: {
                                                    let mut map = tusks::indexmap::IndexMap::new();
                                                    map.insert(
                                                        "tag".to_string(),
                                                        tusks::Argument {
                                                            name: "tag".to_string(),
                                                            type_: "String".to_string(),
                                                            default: Some("latest".to_string()),
                                                            optional: false,
                                                            flag: false,
                                                            positional: false,
                                                            count: None,
                                                            short: None,
                                                            help: None,
                                                            hidden: false,
                                                            value_hint: None,
                                                            arg_enum: None,
                                                            validator: None,
                                                            arg: None,
                                                        },
                                                    );
                                                    map
                                                },
                                            },
                                            tusks::Tusk {
                                                name: "push".to_string(),
                                                arguments: {
                                                    let mut map = tusks::indexmap::IndexMap::new();
                                                    map.insert(
                                                        "registry".to_string(),
                                                        tusks::Argument {
                                                            name: "registry".to_string(),
                                                            type_: "String".to_string(),
                                                            default: Some("latest".to_string()),
                                                            optional: false,
                                                            flag: false,
                                                            positional: false,
                                                            count: None,
                                                            short: None,
                                                            help: None,
                                                            hidden: false,
                                                            value_hint: None,
                                                            arg_enum: None,
                                                            validator: None,
                                                            arg: None,
                                                        },
                                                    );
                                                    map.insert(
                                                        "image".to_string(),
                                                        tusks::Argument {
                                                            name: "image".to_string(),
                                                            type_: "String".to_string(),
                                                            default: None,
                                                            optional: false,
                                                            flag: false,
                                                            positional: false,
                                                            count: None,
                                                            short: None,
                                                            help: None,
                                                            hidden: false,
                                                            value_hint: None,
                                                            arg_enum: None,
                                                            validator: None,
                                                            arg: None,
                                                        },
                                                    );
                                                    map
                                                },
                                            },
                                        ]),
                                    ),
                                    childs: ::alloc::vec::Vec::new(),
                                    links: <[_]>::into_vec(
                                        ::alloc::boxed::box_new([
                                            tusks::LinkNode {
                                                name: "compose".to_string(),
                                            },
                                        ]),
                                    ),
                                    is_link: false,
                                    link_name: None,
                                };
                                node.childs
                                    .push({
                                        let mut linked = super::docker::compose::__tusks_internal_module::get_tusks_tree();
                                        linked.is_link = true;
                                        linked.link_name = Some("compose".to_string());
                                        linked
                                    });
                                node
                            },
                            {
                                let mut node = tusks::TusksNode {
                                    module_path: <[_]>::into_vec(
                                        ::alloc::boxed::box_new([
                                            "tasks".to_string(),
                                            "git".to_string(),
                                        ]),
                                    ),
                                    tusks: <[_]>::into_vec(
                                        ::alloc::boxed::box_new([
                                            tusks::Tusk {
                                                name: "commit".to_string(),
                                                arguments: {
                                                    let mut map = tusks::indexmap::IndexMap::new();
                                                    map.insert(
                                                        "message".to_string(),
                                                        tusks::Argument {
                                                            name: "message".to_string(),
                                                            type_: "String".to_string(),
                                                            default: None,
                                                            optional: false,
                                                            flag: false,
                                                            positional: false,
                                                            count: None,
                                                            short: None,
                                                            help: None,
                                                            hidden: false,
                                                            value_hint: None,
                                                            arg_enum: None,
                                                            validator: None,
                                                            arg: None,
                                                        },
                                                    );
                                                    map
                                                },
                                            },
                                            tusks::Tusk {
                                                name: "push".to_string(),
                                                arguments: {
                                                    let mut map = tusks::indexmap::IndexMap::new();
                                                    map.insert(
                                                        "branch".to_string(),
                                                        tusks::Argument {
                                                            name: "branch".to_string(),
                                                            type_: "String".to_string(),
                                                            default: Some("main".to_string()),
                                                            optional: false,
                                                            flag: false,
                                                            positional: false,
                                                            count: None,
                                                            short: None,
                                                            help: None,
                                                            hidden: false,
                                                            value_hint: None,
                                                            arg_enum: None,
                                                            validator: None,
                                                            arg: None,
                                                        },
                                                    );
                                                    map.insert(
                                                        "force".to_string(),
                                                        tusks::Argument {
                                                            name: "force".to_string(),
                                                            type_: "bool".to_string(),
                                                            default: None,
                                                            optional: false,
                                                            flag: true,
                                                            positional: false,
                                                            count: None,
                                                            short: None,
                                                            help: None,
                                                            hidden: false,
                                                            value_hint: None,
                                                            arg_enum: None,
                                                            validator: None,
                                                            arg: None,
                                                        },
                                                    );
                                                    map
                                                },
                                            },
                                        ]),
                                    ),
                                    childs: ::alloc::vec::Vec::new(),
                                    links: ::alloc::vec::Vec::new(),
                                    is_link: false,
                                    link_name: None,
                                };
                                node
                            },
                        ]),
                    ),
                    links: <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            tusks::LinkNode {
                                name: "holla".to_string(),
                            },
                        ]),
                    ),
                    is_link: false,
                    link_name: None,
                };
                node.childs
                    .push({
                        let mut linked = super::holla::__tusks_internal_module::get_tusks_tree();
                        linked.is_link = true;
                        linked.link_name = Some("holla".to_string());
                        linked
                    });
                node
            }
        }
        pub mod mirror_module {}
        pub fn execute_cli() {
            let mut command = clap::Command::new("tusks")
                .version("1.0")
                .about("Task runner");
            command = build_cli(command, Vec::new());
            let matches = command.get_matches();
            handle_matches(&matches, Vec::new());
        }
        pub fn build_cli(
            mut command: clap::Command,
            path_prefix: Vec<String>,
        ) -> clap::Command {
            let subcommand = {
                let cmd_name = {
                    let mut parts = path_prefix.clone();
                    parts.push("init".to_string());
                    parts.join(".")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("name")
                            .value_name("NAME")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .required(true)
                            .long("name"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let subcommand = {
                let cmd_name = {
                    let mut parts = path_prefix.clone();
                    parts.push("count".to_string());
                    parts.join(".")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("times")
                            .value_name("TIMES")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    u8,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .required(true)
                            .long("times"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let subcommand = {
                let cmd_name = {
                    let mut parts = path_prefix.clone();
                    parts.push("optional".to_string());
                    parts.join(".")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("opt")
                            .value_name("OPT")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    u16,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .required(false)
                            .long("opt"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let subcommand = {
                let cmd_name = {
                    let mut parts = path_prefix.clone();
                    parts.push("sum".to_string());
                    parts.join(".")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("numbers")
                            .value_name("NUMBERS")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    u16,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .required(true)
                            .num_args(2..=3)
                            .long("numbers"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let subcommand = {
                let cmd_name = {
                    let mut parts = path_prefix.clone();
                    parts.push("sum2".to_string());
                    parts.join(".")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("numbers")
                            .value_name("NUMBERS")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    u16,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .required(true)
                            .num_args(..)
                            .long("numbers"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let subcommand = {
                let cmd_name = {
                    let mut parts = path_prefix.clone();
                    parts.push("sum3".to_string());
                    parts.join(".")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("numbers")
                            .value_name("NUMBERS")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    u16,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .required(true)
                            .num_args(..)
                            .index(1),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let subcommand = {
                let cmd_name = {
                    let mut parts = path_prefix.clone();
                    parts.push("positional".to_string());
                    parts.join(".")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("positional")
                            .value_name("POSITIONAL")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .required(true)
                            .index(1),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let subcommand = {
                let cmd_name = {
                    let mut parts = path_prefix.clone();
                    parts.push("t_string".to_string());
                    parts.join(".")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("v_string")
                            .value_name("V_STRING")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .required(true)
                            .long("v_string"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let subcommand = {
                let cmd_name = {
                    let mut parts = path_prefix.clone();
                    parts.push("t_int".to_string());
                    parts.join(".")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("v_int")
                            .value_name("V_INT")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    i8,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .required(true)
                            .long("v_int"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let subcommand = {
                let cmd_name = {
                    let mut parts = path_prefix.clone();
                    parts.push("t_optional".to_string());
                    parts.join(".")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("v_opt")
                            .value_name("V_OPT")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    u8,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .required(false)
                            .long("v_opt"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            {
                let mut link_prefix = path_prefix.clone();
                link_prefix.push("holla".to_string());
                command = super::holla::__tusks_internal_module::build_cli(
                    command,
                    link_prefix,
                );
            }
            let mut child_prefix = path_prefix.clone();
            child_prefix.push("docker".to_string());
            let subcommand = {
                let cmd_name = {
                    let mut parts = child_prefix.clone();
                    parts.push("build".to_string());
                    parts.join(".")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("tag")
                            .value_name("TAG")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .default_value("latest")
                            .long("tag"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let subcommand = {
                let cmd_name = {
                    let mut parts = child_prefix.clone();
                    parts.push("push".to_string());
                    parts.join(".")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("registry")
                            .value_name("REGISTRY")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .default_value("latest")
                            .long("registry"),
                    );
                cmd = cmd
                    .arg(
                        clap::Arg::new("image")
                            .value_name("IMAGE")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .required(true)
                            .long("image"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            {
                let mut link_prefix = child_prefix.clone();
                link_prefix.push("compose".to_string());
                command = super::docker::compose::__tusks_internal_module::build_cli(
                    command,
                    link_prefix,
                );
            }
            let mut child_prefix = path_prefix.clone();
            child_prefix.push("git".to_string());
            let subcommand = {
                let cmd_name = {
                    let mut parts = child_prefix.clone();
                    parts.push("commit".to_string());
                    parts.join(".")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("message")
                            .value_name("MESSAGE")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .required(true)
                            .long("message"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let subcommand = {
                let cmd_name = {
                    let mut parts = child_prefix.clone();
                    parts.push("push".to_string());
                    parts.join(".")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("branch")
                            .value_name("BRANCH")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .default_value("main")
                            .long("branch"),
                    );
                cmd = cmd
                    .arg(
                        clap::Arg::new("force")
                            .action(clap::ArgAction::SetTrue)
                            .long("force"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            command
        }
        pub fn handle_matches(
            matches: &clap::ArgMatches,
            link_path: Vec<String>,
        ) -> bool {
            let (subcommand_name, sub_matches) = matches.subcommand().unwrap();
            let subcommand_prefix = link_path.join(".");
            if !subcommand_prefix.is_empty()
                && !subcommand_name.starts_with(&subcommand_prefix)
            {
                return false;
            }
            let actual_command = if !subcommand_prefix.is_empty() {
                let prefix_len = subcommand_prefix.len() + ".".len();
                &subcommand_name[prefix_len..]
            } else {
                subcommand_name
            };
            match actual_command {
                "init" => {
                    let arg_0 = matches
                        .subcommand()
                        .unwrap()
                        .1
                        .get_one::<String>("name")
                        .unwrap()
                        .clone();
                    super::init(arg_0);
                    return true;
                }
                "count" => {
                    let arg_0 = matches
                        .subcommand()
                        .unwrap()
                        .1
                        .get_one::<u8>("times")
                        .unwrap()
                        .clone();
                    super::count(arg_0);
                    return true;
                }
                "optional" => {
                    let arg_0 = matches
                        .subcommand()
                        .unwrap()
                        .1
                        .get_one::<u16>("opt")
                        .map(|s| s.clone());
                    super::optional(arg_0);
                    return true;
                }
                "sum" => {
                    let arg_0 = matches
                        .subcommand()
                        .unwrap()
                        .1
                        .get_many::<u16>("numbers")
                        .unwrap()
                        .clone()
                        .copied()
                        .collect::<Vec<_>>()
                        .into();
                    super::sum(arg_0);
                    return true;
                }
                "sum2" => {
                    let arg_0 = matches
                        .subcommand()
                        .unwrap()
                        .1
                        .get_many::<u16>("numbers")
                        .unwrap()
                        .clone()
                        .copied()
                        .collect::<Vec<_>>()
                        .into();
                    super::sum2(arg_0);
                    return true;
                }
                "sum3" => {
                    let arg_0 = matches
                        .subcommand()
                        .unwrap()
                        .1
                        .get_many::<u16>("numbers")
                        .unwrap()
                        .clone()
                        .copied()
                        .collect::<Vec<_>>()
                        .into();
                    super::sum3(arg_0);
                    return true;
                }
                "positional" => {
                    let arg_0 = matches
                        .subcommand()
                        .unwrap()
                        .1
                        .get_one::<String>("positional")
                        .unwrap()
                        .clone();
                    super::positional(arg_0);
                    return true;
                }
                "t_string" => {
                    let arg_0 = matches
                        .subcommand()
                        .unwrap()
                        .1
                        .get_one::<String>("v_string")
                        .unwrap()
                        .clone();
                    super::t_string(arg_0);
                    return true;
                }
                "t_int" => {
                    let arg_0 = matches
                        .subcommand()
                        .unwrap()
                        .1
                        .get_one::<i8>("v_int")
                        .unwrap()
                        .clone();
                    super::t_int(arg_0);
                    return true;
                }
                "t_optional" => {
                    let arg_0 = matches
                        .subcommand()
                        .unwrap()
                        .1
                        .get_one::<u8>("v_opt")
                        .map(|s| s.clone());
                    super::t_optional(arg_0);
                    return true;
                }
                "docker.build" => {
                    let arg_0 = matches
                        .subcommand()
                        .unwrap()
                        .1
                        .get_one::<String>("tag")
                        .map(|s| s.clone())
                        .unwrap_or_else(|| "latest".to_string());
                    super::docker::build(arg_0);
                    return true;
                }
                "docker.push" => {
                    let arg_0 = matches
                        .subcommand()
                        .unwrap()
                        .1
                        .get_one::<String>("registry")
                        .map(|s| s.clone())
                        .unwrap_or_else(|| "latest".to_string());
                    let arg_1 = matches
                        .subcommand()
                        .unwrap()
                        .1
                        .get_one::<String>("image")
                        .unwrap()
                        .clone();
                    super::docker::push(arg_0, arg_1);
                    return true;
                }
                "git.commit" => {
                    let arg_0 = matches
                        .subcommand()
                        .unwrap()
                        .1
                        .get_one::<String>("message")
                        .unwrap()
                        .clone();
                    super::git::commit(arg_0);
                    return true;
                }
                "git.push" => {
                    let arg_0 = matches
                        .subcommand()
                        .unwrap()
                        .1
                        .get_one::<String>("branch")
                        .map(|s| s.clone())
                        .unwrap_or_else(|| "main".to_string());
                    let arg_1 = matches.subcommand().unwrap().1.get_flag("force");
                    super::git::push(arg_0, arg_1);
                    return true;
                }
                _ => {
                    {
                        let mut new_link_path = link_path.clone();
                        new_link_path.push("holla".to_string());
                        if super::holla::__tusks_internal_module::handle_matches(
                            matches,
                            new_link_path,
                        ) {
                            return true;
                        }
                    }
                    {
                        let mut new_link_path = link_path.clone();
                        new_link_path.push("docker".to_string());
                        new_link_path.push("compose".to_string());
                        if super::docker::compose::__tusks_internal_module::handle_matches(
                            matches,
                            new_link_path,
                        ) {
                            return true;
                        }
                    }
                    {
                        ::std::io::_eprint(
                            format_args!("Unknown command: {0}\n", subcommand_name),
                        );
                    };
                    return true;
                }
            }
        }
    }
}
fn main() {
    tasks::__tusks_internal_module::execute_cli();
}
