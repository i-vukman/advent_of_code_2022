pub mod day_07 {

    #[derive(Debug)]
    pub struct FileSystemNode {
        name: String,
        size: u32,
        children: Vec<FileSystemNode>
    }

    impl FileSystemNode {
        fn new(name: String, size: u32) -> FileSystemNode {
            FileSystemNode { name, size, children: Vec::new() }
        }

        fn is_folder(&self) -> bool {
            return self.size == 0;
        }

        pub fn sum_folders_with_size_less_than(&self, size_limit: u32) -> u32 {
            let folder_size = if self.is_folder() { self.sum_all_files() } else { 0 };
            let folder_size = if folder_size <= size_limit { folder_size } else { 0 };

            let children_folder_size_sum: u32 = self
                .children
                .iter()
                .map(|child| child.sum_folders_with_size_less_than(size_limit))
                .sum();
            
            folder_size + children_folder_size_sum
        }

        fn sum_all_files(&self) -> u32 {
            if self.children.is_empty() {
                return self.size;
            }

            self.children
                .iter()
                .map(|child| child.sum_all_files())
                .sum()
        }

        pub fn add_folder_to_relative_path(&mut self, rleative_path: &str) {
            self.add_node_to_relative_path(rleative_path, 0);
        }

        pub fn add_file_to_relative_path(&mut self, relative_path: &str, size: u32) {
            self.add_node_to_relative_path(relative_path, size);
        }
        
        fn add_node_to_relative_path(&mut self, relative_path: &str, size: u32) {
            if !self.is_folder() {
                panic!("Can't add nodes to file!");
            }

            if relative_path.is_empty() {
                panic!("Can't add node to empty relative path!");
            }

            let relative_path = relative_path.trim_matches('/');
            let is_node_name = !relative_path.contains('/');
            
            if is_node_name {
                let node_name = relative_path;
                if self.has_child_with_name(node_name) {
                    return;
                }
                
                self.children.push(FileSystemNode::new(node_name.to_string(), size));
                return;
            }
            
            let folder_name = relative_path.split('/').next().expect("Iterator_can't be empty");
            let new_relative_path = &relative_path[folder_name.len() + 1..];

            let existing_folder = self.children.iter_mut().find(|child| child.is_folder() && child.name == folder_name);

            match existing_folder {
                Some(folder) => folder.add_node_to_relative_path(new_relative_path, size),
                None => {
                    let mut new_folder = FileSystemNode::new(folder_name.to_string(), 0);
                    new_folder.add_node_to_relative_path(new_relative_path, size);
                    self.children.push(new_folder);
                }
            }
        }

        fn has_child_with_name(&self, name: &str) -> bool {
          for child in self.children.iter() {
            if child.name == name {
                return true;
            }
          }
          false
        }
    }

    pub fn solve(input: &str) -> u32 {
        let mut current_path = String::from("");
        let mut root = FileSystemNode::new("/".to_string(), 0);

        //TODO: extract to build_hierarchy
        input
            .lines()
            .for_each(|line| {
                match line {
                    _l if line.starts_with("$ cd") => {
                        let path = line.split(' ').last().expect("Missing cd param");
                        match path {
                            ".." => {
                                while current_path.pop().expect(&format!("{current_path} should not be empty")) != '/' { }
                            },
                            "/" => current_path = "/".to_string(),
                            _ => current_path.push_str(&format!("/{}", path))
                        } 
                    },
                    _l if line.starts_with("$ ls") => (),
                    l if line.starts_with("dir") => { 
                        let dir_name = l.split(' ').last().expect("Missing dir name in output");
                        root.add_node_to_relative_path(&format!("{}/{}", current_path, dir_name), 0);
                    }
                    _ => { 
                        let mut split = line.split(' ');
                        let size = split.next().expect("Missing size for listed file");
                        let name = split.next().expect("Missing name for listed file");
                        let relative_path = &format!("{}/{}", current_path, name);
                        root.add_file_to_relative_path(relative_path, size.parse().expect(&format!("Invalid file size {size}")));
                    }
                }
            });
        root.sum_folders_with_size_less_than(100000)
    }
  }

  #[cfg(test)]
  mod tests {
    use std::fs;

      #[test]
      fn test_part_01() {
          let input = fs::read_to_string("input/day_07.txt").unwrap_or(String::from(""));
          assert_eq!(super::day_07::solve(&input), 1648397);
      }

      #[test]
      fn test_sample() {
          let input = fs::read_to_string("input/day_07_sample.txt").unwrap_or(String::from(""));
          assert_eq!(super::day_07::solve(&input), 95437);
      }
  }
