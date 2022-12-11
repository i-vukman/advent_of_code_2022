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

        pub fn sum_folders_with_max_size(&self, max_size: u32) -> u32 {
            let folder_size = if self.is_folder() { self.sum_all_files() } else { 0 };
            let folder_size = if folder_size <= max_size { folder_size } else { 0 };

            let children_folder_size_sum: u32 = self
                .children
                .iter()
                .map(|child| child.sum_folders_with_max_size(max_size))
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

            let relative_path = relative_path.trim_matches('/');

            //TODO: extract in separate method, move to loop!!
            if !relative_path.contains('/') {
                let node_name = relative_path;
                //TODO: check if we need this check here?
                for child in self.children.iter() {
                    if child.name == node_name {
                        return;
                    }
                }
                
                self.children.push(FileSystemNode::new(node_name.to_string(), size));
                return;
            }

            let path_iterator = relative_path.split('/');
            let mut consumed_chars = 0;
            
            //TODO: Could we care about only first element instead, and then create new relative path from it?
            for path in path_iterator {
                consumed_chars += path.len();
                let new_relative_path = &relative_path[consumed_chars..];
                for child in self.children.iter_mut() {
                    if child.is_folder() && path == child.name {
                        return child.add_node_to_relative_path(new_relative_path, size);
                    }
                }
                let mut new_node = FileSystemNode::new(path.to_string(), 0);
                new_node.add_node_to_relative_path(new_relative_path, size);
                return self.children.push(new_node);
            }
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
        println!("{:#?}", root);
        root.sum_folders_with_max_size(100000)
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
