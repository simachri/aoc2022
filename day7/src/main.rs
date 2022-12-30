enum Cmd {
    Exit,
    CdUp,
    CdInto(String),
    Ls,
}

struct ExpandDirResult<'a> {
    next_cmd: Option<Cmd>,
    cli_output_iterator: Box<dyn Iterator<Item = String> + 'a>,
    expanded_dir: FSDir,
}

struct CliCmdConsumptionResult<'a> {
    next_cmd: Option<Cmd>,
    cli_output_iterator: Box<dyn Iterator<Item = String> + 'a>,
    expanded_dir: FSDir,
}

#[derive(Debug, PartialEq, Clone)]
enum FSNode {
    File(FSFile),
    Directory(FSDir),
}

#[derive(Debug, PartialEq, Clone)]
struct FSFile {
    name: String,
    size: u32,
}

#[derive(Debug, PartialEq, Clone)]
struct FSDir {
    name: String,
    children: Vec<FSNode>,
    size: u32,
}

impl<'a> FSDir {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            children: Vec::new(),
            size: 0,
        }
    }

    fn expand_from_cli_output(
        mut self,
        mut cli_output_iterator: Box<dyn Iterator<Item = String> + 'a>,
    ) -> ExpandDirResult {
        loop {
            let cli_output = cli_output_iterator
                .next()
                .unwrap_or_else(|| String::from("exit"));

            if is_command(&cli_output) {
                return ExpandDirResult {
                    next_cmd: Some(get_command(&cli_output)),
                    cli_output_iterator,
                    expanded_dir: self,
                };
            }

            let mut output_parts = cli_output.split_whitespace();
            match output_parts.next().unwrap() {
                "dir" => {
                    let dir_name = output_parts.next().unwrap();
                    self.children.push(FSNode::Directory(FSDir {
                        name: String::from(dir_name),
                        children: Vec::new(),
                        size: 0,
                    }))
                }

                file => {
                    let fsize = file.parse::<u32>().unwrap();
                    let fname = output_parts.next().unwrap();
                    self.children.push(FSNode::File(FSFile {
                        name: String::from(fname),
                        size: fsize,
                    }));

                    self.size += fsize;
                }
            };
        }
    }
}

fn get_command(cmd: &str) -> Cmd {
    if cmd == "exit" {
        return Cmd::Exit;
    }

    let mut cmd_args = cmd.split_whitespace().skip(1);
    match cmd_args.next().unwrap() {
        "cd" => match cmd_args.next().unwrap() {
            ".." => Cmd::CdUp,
            dir => Cmd::CdInto(String::from(dir)),
        },
        "ls" => Cmd::Ls,
        _ => panic!("Unknown command"),
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let fstree = parse_fstree_from_cli_output(input);
    let day1_result = calculate_day_1(&fstree);
    let day2_result = calculate_day_2(&fstree);

    println!("Result of day 1: {}", day1_result);
    println!("Result of day 2: {}", day2_result);
}

fn calculate_day_1(fstree: &FSDir) -> u32 {
    return aggregate_dir_size_with_max_size_of(fstree, 100_000);
}

fn calculate_day_2(fstree: &FSDir) -> u32 {
    let disk_size = 70_000_000;
    let required_space = 30_000_000;
    let used_space = fstree.size;
    let space_to_be_freed: u32 = (((disk_size - required_space) as i32 - used_space as i32))
        .abs()
        .try_into()
        .unwrap();

    return get_size_of_dir_to_delete(fstree, space_to_be_freed);
}

fn aggregate_dir_size_with_max_size_of(fstree: &FSDir, max_size: u32) -> u32 {
    let mut total_size = 0;

    if fstree.size <= max_size {
        total_size += fstree.size;
    }

    for child in fstree.children.iter() {
        match child {
            FSNode::File(..) => (),
            FSNode::Directory(dir) => {
                let dir_size = aggregate_dir_size_with_max_size_of(dir, max_size);
                total_size += dir_size;
            }
        }
    }

    return total_size;
}

fn get_size_of_dir_to_delete(fstree: &FSDir, required_size: u32) -> u32 {
    let mut smallest_sufficient_dir_size = u32::MAX;

    if fstree.size >= required_size {
        smallest_sufficient_dir_size = fstree.size;
    }

    for child in fstree.children.iter() {
        match child {
            FSNode::File(..) => (),
            FSNode::Directory(dir) => {
                let dir_size = get_size_of_dir_to_delete(dir, required_size);
                if dir_size >= required_size && dir_size < smallest_sufficient_dir_size {
                    smallest_sufficient_dir_size = dir_size;
                }
            }
        }
    }

    return smallest_sufficient_dir_size;
}

fn parse_fstree_from_cli_output(input: &str) -> FSDir {
    // First command is always '$ cd <root>'.
    let root = input
        .split_once("\n")
        .unwrap()
        .0
        .to_string()
        .split_whitespace()
        .nth(2)
        .unwrap()
        .to_string();

    let fstree = FSDir::new(&root);

    let cli_output_iterator = Box::new(input.lines().skip(1).map(|line| line.to_string()));

    let result = consume_cli_output_cmds(cli_output_iterator, fstree);

    return result.expanded_dir;
}

fn consume_cli_output_cmds<'a>(
    mut cli_output_iterator: Box<dyn Iterator<Item = String> + 'a>,
    mut cwd: FSDir,
) -> CliCmdConsumptionResult<'a> {
    let mut received_next_cmd: Option<Cmd> = None;
    let mut next_cmd: Cmd;

    loop {
        if !received_next_cmd.is_none() {
            next_cmd = received_next_cmd.unwrap();
        } else {
            let cli_output = cli_output_iterator
                .next()
                .unwrap_or_else(|| String::from("exit"));

            if !is_command(&cli_output) {
                continue;
            }

            next_cmd = get_command(&cli_output);
        }

        match next_cmd {
            Cmd::Exit => {
                return CliCmdConsumptionResult {
                    next_cmd: Some(Cmd::Exit),
                    cli_output_iterator: Box::new(cli_output_iterator),
                    expanded_dir: cwd,
                };
            }

            Cmd::CdUp => {
                return CliCmdConsumptionResult {
                    // The next command shall be read from the CLI input from the calling function.
                    next_cmd: None,
                    cli_output_iterator: Box::new(cli_output_iterator),
                    expanded_dir: cwd,
                };
            }

            Cmd::CdInto(target_dir) => {
                let next_dir: &mut FSNode = cwd
                    .children
                    .iter_mut()
                    .find(|node| match node {
                        FSNode::Directory(dir) => dir.name == target_dir,
                        _ => false,
                    })
                    .unwrap();

                match next_dir {
                    FSNode::Directory(next_dir) => {
                        let result = consume_cli_output_cmds(cli_output_iterator, next_dir.clone());
                        cli_output_iterator = result.cli_output_iterator;
                        received_next_cmd = result.next_cmd;
                        *next_dir = result.expanded_dir;

                        cwd.size += next_dir.size;
                    }
                    _ => panic!("Expected directory"),
                }
            }

            Cmd::Ls => {
                let expanded_cwd = cwd.clone();
                let expansion_result =
                    expanded_cwd.expand_from_cli_output(Box::new(cli_output_iterator));
                cli_output_iterator = expansion_result.cli_output_iterator;
                received_next_cmd = expansion_result.next_cmd;
                cwd = expansion_result.expanded_dir;
            }
        }
    }
}

fn is_command(first_cli_token: &str) -> bool {
    return first_cli_token.chars().nth(0).unwrap() == '$' || first_cli_token == "exit";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fstree_singlelevel_from_cli_output() {
        let input = r#"$ cd /
$ ls
42 foo
73 bar
"#;

        let want = FSDir {
            name: "/".to_string(),
            children: vec![
                FSNode::File(FSFile {
                    name: "foo".to_string(),
                    size: 42,
                }),
                FSNode::File(FSFile {
                    name: "bar".to_string(),
                    size: 73,
                }),
            ],
            size: 115,
        };

        let got = parse_fstree_from_cli_output(input);

        assert_eq!(want, got);
    }

    #[test]
    fn test_parse_fstree_multilevel_from_cli_output() {
        let input = include_str!("../test.txt");

        let want = FSDir {
            name: "/".to_string(),
            children: vec![
                FSNode::Directory(FSDir {
                    name: "a".to_string(),
                    children: vec![
                        FSNode::Directory(FSDir {
                            name: "e".to_string(),
                            children: vec![FSNode::File(FSFile {
                                name: "i".to_string(),
                                size: 584,
                            })],
                            size: 584,
                        }),
                        FSNode::File(FSFile {
                            name: "f".to_string(),
                            size: 29116,
                        }),
                        FSNode::File(FSFile {
                            name: "g".to_string(),
                            size: 2557,
                        }),
                        FSNode::File(FSFile {
                            name: "h.lst".to_string(),
                            size: 62596,
                        }),
                    ],
                    size: 94853,
                }),
                FSNode::File(FSFile {
                    name: "b.txt".to_string(),
                    size: 14848514,
                }),
                FSNode::File(FSFile {
                    name: "c.dat".to_string(),
                    size: 8504156,
                }),
                FSNode::Directory(FSDir {
                    name: "d".to_string(),
                    children: vec![
                        FSNode::File(FSFile {
                            name: "j".to_string(),
                            size: 4060174,
                        }),
                        FSNode::File(FSFile {
                            name: "d.log".to_string(),
                            size: 8033020,
                        }),
                        FSNode::File(FSFile {
                            name: "d.ext".to_string(),
                            size: 5626152,
                        }),
                        FSNode::File(FSFile {
                            name: "k".to_string(),
                            size: 7214296,
                        }),
                    ],
                    size: 24933642,
                }),
            ],
            size: 48381165,
        };

        let got = parse_fstree_from_cli_output(input);

        assert_eq!(want, got);
    }

    #[test]
    fn test_day1() {
        let fstree = parse_fstree_from_cli_output(include_str!("../test.txt"));

        assert_eq!(95437, calculate_day_1(&fstree));
    }

    #[test]
    fn test_day2() {
        let fstree = parse_fstree_from_cli_output(include_str!("../test.txt"));

        assert_eq!(24933642, calculate_day_2(&fstree));
    }
}
