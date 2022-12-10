pub mod day_07 {
    use std::ops::{AddAssign, Add};

    #[derive(Debug)]
    pub struct FileSystemNode {
        name: String,
        size: u32,
        //TODO improvement: Could be changed to Set? PartialEq would be name + size
        children: Vec<FileSystemNode>
    }

    impl FileSystemNode {
        fn new(name: String, size: u32) -> FileSystemNode {
            FileSystemNode { name, size, children: Vec::new() }
        }

        fn is_folder(&self) -> bool {
            return self.size == 0;
        }
        
        fn add_file_to_relative_path(&mut self, relative_path: &str, size: u32) {
            if !self.is_folder() {
                panic!("Can't add file to file");
            }

            //TODO: traverse hierarchy until file is found!
            self.add_file(relative_path, size);
        }

        fn add_file(&mut self, file_name: &str, size: u32) {
            if !self.is_folder() {
                panic!("Can't add file to file");
            }

            if file_name.contains("/") {
                panic!("Invalid file name {file_name}");
            }

            self.children.push(FileSystemNode::new(file_name.to_string(), size));
        }
        
        fn add_folder_to_relative_path(&mut self, relative_path: &str) {
            if !self.is_folder() {
                panic!("Can't add folder to file");
            }

            let relative_path = relative_path.trim_matches('/');

            if !relative_path.contains('/') {
                self.add_folder(relative_path);
                return;
            }

            let path_iterator = relative_path.split('/');
            let mut consumed_chars = 0;
            
            for path in path_iterator {
                consumed_chars += path.len() + 1;
                for child in self.children.iter_mut() {
                    if child.is_folder() && path == child.name {
                        return child.add_folder_to_relative_path(&relative_path[consumed_chars..]);
                    }
                }
                self.add_folder(relative_path);
            }
        }

        fn add_folder(&mut self, folder_name: &str) {
            if !self.is_folder() {
                panic!("Can't add folder to file");
            }

            self.children.push(FileSystemNode::new(folder_name.to_string(), 0));
        }
    }

    pub fn solve(input: &str) -> FileSystemNode {
        let mut current_path = String::from("");
        let mut root = FileSystemNode::new("".to_string(), 0);

        input
            .lines()
            .for_each(|line| {
                match line {
                    l if line.starts_with("$ cd") => {
                        let path = line.split(' ').last().expect("Missing cd param");
                        match path {
                            ".." => {
                                let mut chars = current_path.chars();
                                while chars.next_back().unwrap() != '/' {}
                            },
                            "/" => current_path = "/".to_string(),
                            _ => current_path.push_str(&format!("/{}/", path))
                        } 
                    },
                    l if line.starts_with("$ ls") => (),
                    l if line.starts_with("dir") => { 
                        let dir_name = l.split(' ').last().expect("Missing dir name in output");
                        root.add_folder_to_relative_path(&format!("{}{}", current_path, dir_name));
                    }
                    _ => { 
                        //let mut split = line.split(' ');
                        //let size = split.next().expect("Missing size for listed file");
                        //let name = split.next().expect("Missing name for listed file");
                        //root.add_file_to_relative_path(name, size.parse().expect(&format!("Invalid file size {size}")));
                    }
                }
            });
        
        root
    }
  }

  #[cfg(test)]
  mod tests {
    use std::fs;

      #[test]
      fn test_part_01() {
          let input = fs::read_to_string("input/day_07_sample.txt").unwrap_or(String::from(""));
          let file_hierarchy = super::day_07::solve(&input);
          println!("{:#?}", file_hierarchy);
          assert_eq!(2, 3);
      }
  }
