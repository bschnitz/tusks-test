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
                        module_path: <[_]>::into_vec(::alloc::boxed::box_new(["tasks"])),
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
            pub mod mirror_module {
                pub fn holla() {
                    super::super::holla()
                }
            }
            pub fn execute_cli(path_sep: String) {
                let mut command = clap::Command::new("tusks")
                    .version("1.0")
                    .about("Task runner");
                command = build_cli(command, Vec::new(), path_sep);
                let _matches = command.get_matches();
            }
            pub fn build_cli(
                mut command: clap::Command,
                path_prefix: Vec<String>,
                path_sep: String,
            ) -> clap::Command {
                let subcommand = {
                    let cmd_name = {
                        let mut parts = path_prefix.clone();
                        if !parts.is_empty() {
                            parts.push(path_sep.clone());
                        }
                        parts.push("holla".to_string());
                        parts.join("")
                    };
                    let mut cmd = clap::Command::new(cmd_name);
                    cmd
                };
                command = command.subcommand(subcommand);
                command
            }
        }
    }
}
pub mod compose {
    use tusks::tusks;
    pub mod compose {
        pub fn up(detached: Option<bool>) {
            let detached_flag = detached.unwrap_or(false);
            {
                ::std::io::_print(
                    format_args!("Starting containers (detached: {0})\n", detached_flag),
                );
            };
        }
        pub fn down(force: String) {
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
            pub fn logs(service: String, follow: Option<bool>) {
                let follow_flag = follow.unwrap_or(false);
                {
                    ::std::io::_print(
                        format_args!(
                            "Showing logs for {0} (follow: {1})\n",
                            service,
                            follow_flag,
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
                            ::alloc::boxed::box_new(["compose"]),
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
                                                optional: true,
                                                flag: false,
                                                value: None,
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
                                                type_: "String".to_string(),
                                                default: Some("false".to_string()),
                                                optional: false,
                                                flag: false,
                                                value: None,
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
                                            ::alloc::boxed::box_new(["compose", "services"]),
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
                                                                value: None,
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
                                                                value: None,
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
                                                                value: None,
                                                            },
                                                        );
                                                        map.insert(
                                                            "follow".to_string(),
                                                            tusks::Argument {
                                                                name: "follow".to_string(),
                                                                type_: "bool".to_string(),
                                                                default: None,
                                                                optional: true,
                                                                flag: false,
                                                                value: None,
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
            pub mod mirror_module {
                pub fn up(detached: impl Into<Option<String>>) {
                    super::super::up(detached.into().map(|v: String| v.parse().unwrap()))
                }
                pub fn down(force: String) {
                    super::super::down(force.parse().unwrap())
                }
                pub mod services {
                    pub fn list() {
                        super::super::super::services::list()
                    }
                    pub fn scale(service: String, replicas: String) {
                        super::super::super::services::scale(
                            service.parse().unwrap(),
                            replicas.parse().unwrap(),
                        )
                    }
                    pub fn logs(service: String, follow: impl Into<Option<String>>) {
                        super::super::super::services::logs(
                            service.parse().unwrap(),
                            follow.into().map(|v: String| v.parse().unwrap()),
                        )
                    }
                }
            }
            pub fn execute_cli(path_sep: String) {
                let mut command = clap::Command::new("tusks")
                    .version("1.0")
                    .about("Task runner");
                command = build_cli(command, Vec::new(), path_sep);
                let _matches = command.get_matches();
            }
            pub fn build_cli(
                mut command: clap::Command,
                path_prefix: Vec<String>,
                path_sep: String,
            ) -> clap::Command {
                let subcommand = {
                    let cmd_name = {
                        let mut parts = path_prefix.clone();
                        if !parts.is_empty() {
                            parts.push(path_sep.clone());
                        }
                        parts.push("up".to_string());
                        parts.join("")
                    };
                    let mut cmd = clap::Command::new(cmd_name);
                    cmd = cmd
                        .arg(
                            clap::Arg::new("detached")
                                .value_name("DETACHED")
                                .required(false)
                                .long("detached"),
                        );
                    cmd
                };
                command = command.subcommand(subcommand);
                let subcommand = {
                    let cmd_name = {
                        let mut parts = path_prefix.clone();
                        if !parts.is_empty() {
                            parts.push(path_sep.clone());
                        }
                        parts.push("down".to_string());
                        parts.join("")
                    };
                    let mut cmd = clap::Command::new(cmd_name);
                    cmd = cmd
                        .arg(
                            clap::Arg::new("force")
                                .value_name("FORCE")
                                .required(true)
                                .default_value("false")
                                .long("force"),
                        );
                    cmd
                };
                command = command.subcommand(subcommand);
                let mut child_prefix = path_prefix.clone();
                if !child_prefix.is_empty() {
                    child_prefix.push(path_sep.clone());
                }
                child_prefix.push("services".to_string());
                let subcommand = {
                    let cmd_name = {
                        let mut parts = child_prefix.clone();
                        if !parts.is_empty() {
                            parts.push(path_sep.clone());
                        }
                        parts.push("list".to_string());
                        parts.join("")
                    };
                    let mut cmd = clap::Command::new(cmd_name);
                    cmd
                };
                command = command.subcommand(subcommand);
                let subcommand = {
                    let cmd_name = {
                        let mut parts = child_prefix.clone();
                        if !parts.is_empty() {
                            parts.push(path_sep.clone());
                        }
                        parts.push("scale".to_string());
                        parts.join("")
                    };
                    let mut cmd = clap::Command::new(cmd_name);
                    cmd = cmd
                        .arg(
                            clap::Arg::new("service")
                                .value_name("SERVICE")
                                .required(true)
                                .long("service"),
                        );
                    cmd = cmd
                        .arg(
                            clap::Arg::new("replicas")
                                .value_name("REPLICAS")
                                .required(true)
                                .default_value("1")
                                .long("replicas"),
                        );
                    cmd
                };
                command = command.subcommand(subcommand);
                let subcommand = {
                    let cmd_name = {
                        let mut parts = child_prefix.clone();
                        if !parts.is_empty() {
                            parts.push(path_sep.clone());
                        }
                        parts.push("logs".to_string());
                        parts.join("")
                    };
                    let mut cmd = clap::Command::new(cmd_name);
                    cmd = cmd
                        .arg(
                            clap::Arg::new("service")
                                .value_name("SERVICE")
                                .required(true)
                                .long("service"),
                        );
                    cmd = cmd
                        .arg(
                            clap::Arg::new("follow")
                                .value_name("FOLLOW")
                                .required(false)
                                .long("follow"),
                        );
                    cmd
                };
                command = command.subcommand(subcommand);
                command
            }
        }
    }
}
mod tasks {
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
    pub mod __tusks_internal_module {
        use tusks::{TusksNode, Tusk, Argument, LinkNode};
        use std::collections::HashMap;
        pub fn get_tusks_tree() -> TusksNode {
            {
                let mut node = tusks::TusksNode {
                    module_path: <[_]>::into_vec(::alloc::boxed::box_new(["tasks"])),
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
                                            value: None,
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
                                            value: None,
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
                                            value: None,
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
                                        ::alloc::boxed::box_new(["tasks", "docker"]),
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
                                                            value: None,
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
                                                            value: None,
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
                                                            value: None,
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
                                        ::alloc::boxed::box_new(["tasks", "git"]),
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
                                                            value: None,
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
                                                            value: None,
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
                                                            value: None,
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
        pub mod mirror_module {
            pub fn init(name: String) {
                super::super::init(name.parse().unwrap())
            }
            pub fn count(times: String) {
                super::super::count(times.parse().unwrap())
            }
            pub fn optional(opt: impl Into<Option<String>>) {
                super::super::optional(opt.into().map(|v: String| v.parse().unwrap()))
            }
            pub mod docker {
                pub fn build(tag: String) {
                    super::super::super::docker::build(tag.parse().unwrap())
                }
                pub fn push(registry: String, image: String) {
                    super::super::super::docker::push(
                        registry.parse().unwrap(),
                        image.parse().unwrap(),
                    )
                }
            }
            pub mod git {
                pub fn commit(message: String) {
                    super::super::super::git::commit(message.parse().unwrap())
                }
                pub fn push(branch: String, force: bool) {
                    super::super::super::git::push(branch.parse().unwrap(), force)
                }
            }
        }
        pub fn execute_cli(path_sep: String) {
            let mut command = clap::Command::new("tusks")
                .version("1.0")
                .about("Task runner");
            command = build_cli(command, Vec::new(), path_sep);
            let _matches = command.get_matches();
        }
        pub fn build_cli(
            mut command: clap::Command,
            path_prefix: Vec<String>,
            path_sep: String,
        ) -> clap::Command {
            let subcommand = {
                let cmd_name = {
                    let mut parts = path_prefix.clone();
                    if !parts.is_empty() {
                        parts.push(path_sep.clone());
                    }
                    parts.push("init".to_string());
                    parts.join("")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("name")
                            .value_name("NAME")
                            .required(true)
                            .long("name"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let subcommand = {
                let cmd_name = {
                    let mut parts = path_prefix.clone();
                    if !parts.is_empty() {
                        parts.push(path_sep.clone());
                    }
                    parts.push("count".to_string());
                    parts.join("")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("times")
                            .value_name("TIMES")
                            .required(true)
                            .long("times"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let subcommand = {
                let cmd_name = {
                    let mut parts = path_prefix.clone();
                    if !parts.is_empty() {
                        parts.push(path_sep.clone());
                    }
                    parts.push("optional".to_string());
                    parts.join("")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("opt")
                            .value_name("OPT")
                            .required(false)
                            .long("opt"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let mut link_prefix = path_prefix.clone();
            if !link_prefix.is_empty() {
                link_prefix.push(path_sep.clone());
            }
            link_prefix.push("holla".to_string());
            command = super::holla::__tusks_internal_module::build_cli(
                command,
                link_prefix,
                path_sep.clone(),
            );
            let mut child_prefix = path_prefix.clone();
            if !child_prefix.is_empty() {
                child_prefix.push(path_sep.clone());
            }
            child_prefix.push("docker".to_string());
            let subcommand = {
                let cmd_name = {
                    let mut parts = child_prefix.clone();
                    if !parts.is_empty() {
                        parts.push(path_sep.clone());
                    }
                    parts.push("build".to_string());
                    parts.join("")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("tag")
                            .value_name("TAG")
                            .required(true)
                            .default_value("latest")
                            .long("tag"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let subcommand = {
                let cmd_name = {
                    let mut parts = child_prefix.clone();
                    if !parts.is_empty() {
                        parts.push(path_sep.clone());
                    }
                    parts.push("push".to_string());
                    parts.join("")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("registry")
                            .value_name("REGISTRY")
                            .required(true)
                            .default_value("latest")
                            .long("registry"),
                    );
                cmd = cmd
                    .arg(
                        clap::Arg::new("image")
                            .value_name("IMAGE")
                            .required(true)
                            .long("image"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let mut link_prefix = child_prefix.clone();
            if !link_prefix.is_empty() {
                link_prefix.push(path_sep.clone());
            }
            link_prefix.push("compose".to_string());
            command = super::docker::compose::__tusks_internal_module::build_cli(
                command,
                link_prefix,
                path_sep.clone(),
            );
            let mut child_prefix = path_prefix.clone();
            if !child_prefix.is_empty() {
                child_prefix.push(path_sep.clone());
            }
            child_prefix.push("git".to_string());
            let subcommand = {
                let cmd_name = {
                    let mut parts = child_prefix.clone();
                    if !parts.is_empty() {
                        parts.push(path_sep.clone());
                    }
                    parts.push("commit".to_string());
                    parts.join("")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("message")
                            .value_name("MESSAGE")
                            .required(true)
                            .long("message"),
                    );
                cmd
            };
            command = command.subcommand(subcommand);
            let subcommand = {
                let cmd_name = {
                    let mut parts = child_prefix.clone();
                    if !parts.is_empty() {
                        parts.push(path_sep.clone());
                    }
                    parts.push("push".to_string());
                    parts.join("")
                };
                let mut cmd = clap::Command::new(cmd_name);
                cmd = cmd
                    .arg(
                        clap::Arg::new("branch")
                            .value_name("BRANCH")
                            .required(true)
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
    }
}
fn main() {
    tasks::__tusks_internal_module::execute_cli(".".to_string());
}
