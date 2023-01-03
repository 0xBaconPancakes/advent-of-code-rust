use std::{collections::HashMap, cell::RefCell, borrow::Borrow, str::Lines};

trait Sized {
    fn size(&self) -> u32;
}

struct Dir {
    name: String,
    files: HashMap<String, RefCell<File>>,
    dirs: HashMap<String, RefCell<Dir>>,
}

impl Dir {
    fn sum_small_sizes(&self) -> u32 {
        let subdirs_size = self.dirs.iter().map(|(_, d)| d.borrow().sum_small_sizes()).sum::<u32>();
        let size = self.size();
        if size < 100000 {
            size + subdirs_size
        } else {
            0 + subdirs_size
        }
    }

    fn smallest_subdir(&self, size_greater_than: u32) -> u32 {
        let mut smallest = u32::MAX;
        if self.size() > size_greater_than {
            smallest = self.size();
        }
        for (_, d) in self.dirs.iter() {
            let size = d.borrow().size();
            if size > size_greater_than {
                let sub_smallest = d.borrow().smallest_subdir(size_greater_than);
                if smallest == 0 || sub_smallest < smallest {
                    smallest = sub_smallest;
                }
            }
        }
        smallest
    }
}

impl Sized for Dir {
    fn size(&self) -> u32 {
        let files_size: u32 = self.files.iter().map(|(n, f)| f.borrow().size()).sum();
        let dirs_size: u32 = self.dirs.iter().map(|(n, d)| d.borrow().size()).sum();
        files_size + dirs_size
    }
}

struct File {
    name: String,
    size: u32,
}

impl Sized for File {
    fn size(&self) -> u32 {
        self.size
    }
}

fn parse_input(lines: &mut std::str::Lines<'_>, name: String) -> Dir {
    let mut root = Dir {
        name: name,
        files: HashMap::new(),
        dirs: HashMap::new(),
    };
    while let Some(next_line) = lines.next() {
        if next_line == "$ cd .." {
            return root;
        } else if next_line == "$ ls" {
            continue;
        } else if next_line.starts_with("$ cd ") {
            let dir_name = next_line[5..].to_string();
            let dir = parse_input(lines, dir_name);
            root.dirs.insert(dir.name.clone(), RefCell::new(dir));
        } else if next_line.starts_with("dir") {
            continue;
        } else {
            let mut parts = next_line.split_whitespace();
            let size = parts.next().unwrap().parse::<u32>().unwrap();
            let name = parts.next().unwrap().to_string();
            root.files.insert(name.clone(), RefCell::new(File {
                name: name,
                size: size,
            }));
        }
    }
    root
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines: Lines = input.lines();
    lines.next();
    let root = parse_input(&mut lines, String::from("/"));
    Some(root.sum_small_sizes())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines: Lines = input.lines();
    lines.next();
    let root = parse_input(&mut lines, String::from("/"));
    let size_root = root.size();
    let space_to_free = size_root - 40000000;
    Some(root.smallest_subdir(space_to_free))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
