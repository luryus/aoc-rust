use std::io;

struct Dir {
    name: String,
    inodes: Vec<Inode>,
}

impl Dir {
    fn find_dir(&mut self, path: &[String]) -> Option<&mut Dir> {
        if path.is_empty() {
            return Some(self);
        }
        self.inodes
            .iter_mut()
            .find_map(|i| {
                if let Inode::Dir(subdir) = i {
                    if subdir.name == path[0] {
                        return Some(subdir);
                    }
                }
                None
            })
            .and_then(|d| d.find_dir(&path[1..]))
    }

    fn size(&self) -> usize {
        self.inodes.iter().map(|i| i.size()).sum()
    }
}

struct File {
    size: usize,
}

enum Inode {
    Dir(Dir),
    File(File),
}

impl Inode {
    fn size(&self) -> usize {
        match self {
            Inode::Dir(d) => d.size(),
            Inode::File(f) => f.size,
        }
    }
}

fn part1(mut dir_sizes: Vec<usize>) -> usize {
    dir_sizes.sort();
    dir_sizes.into_iter().filter(|d| *d <= 100000).sum()
}

fn part2(mut dir_sizes: Vec<usize>) -> usize {
    // root is last
    let available_space = 70000000 - dir_sizes.last().unwrap();
    let needed_space = 30000000;
    dir_sizes.sort();
    dir_sizes
        .into_iter()
        .find(|e| e + available_space > needed_space)
        .unwrap()
}

fn parse_input(input: Vec<String>) -> Dir {
    let mut current_path = vec![];
    let mut root = Dir {
        name: "".to_string(),
        inodes: vec![],
    };

    for l in input.into_iter().skip(1) {
        if l == "$ ls" {
            continue;
        } else if let Some(target) = l.strip_prefix("$ cd ") {
            if target == ".." {
                current_path.pop();
            } else {
                current_path.push(target.to_string());
            }
        } else {
            let current_dir = root
                .find_dir(&current_path)
                .expect("Directory not found for current path");
            if let Some(dn) = l.strip_prefix("dir ") {
                let dn = dn.to_string();
                current_dir.inodes.push(Inode::Dir(Dir {
                    name: dn,
                    inodes: vec![],
                }));
            } else {
                let (size, _) = l.split_once(' ').expect("Not a valid file line");
                let size = size.parse().expect("Parsing size failed");
                let file = File { size };
                current_dir.inodes.push(Inode::File(file));
            }
        }
    }

    root
}

fn walk_dirs(root: &Dir) -> Vec<usize> {
    let mut entries: Vec<_> = root
        .inodes
        .iter()
        .filter_map(|i| if let Inode::Dir(d) = i { Some(d) } else { None })
        .flat_map(walk_dirs)
        .collect();
    entries.push(root.size());
    entries
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let root = parse_input(input);
    let dir_sizes = walk_dirs(&root);

    let p1 = part1(dir_sizes.clone());
    println!("Part 1: {}", p1);

    let p2 = part2(dir_sizes);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(7)).unwrap();

        let root = parse_input(input);
        let dir_sizes = walk_dirs(&root);

        let p1 = part1(dir_sizes.clone());
        assert_eq!(p1, 1443806);

        let p2 = part2(dir_sizes);
        assert_eq!(p2, 942298);
    }
}
