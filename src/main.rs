use std::fs;


/// Define possible Data types
#[derive(Debug, Clone)]
enum Data {
    /// File with name and size
    File(String, usize),
    /// Directory with name and multiple data
    Directory(String, Vec<Data>)
}
impl Data {
    /// Get the name of the file or directory
    fn get_name(&self) -> String {
        match self {
            Self::File(n, _) => n.to_string(),
            Self::Directory(n, _) => n.to_string(),
        }
    }
    /// Add Data on a specified position
    /// 
    /// # Arguments
    /// * `position` - Iterator over the directory hierarchy to where to add the data
    /// * `data` - The data to add
    fn add<I>(&mut self, mut position: I, data: Self) -> bool 
        where
            I: Iterator<Item = String> + Clone
    {
        match self {
            Self::File(_, _) => false,
            Self::Directory(_, datas) => {
                let pos = position.next();
                if pos.is_none() {
                    datas.push(data);
                    return true
                }
        
                for d in datas {
                    if d.get_name() == *pos.as_ref().unwrap() && d.add(position.clone(), data.clone()) {
                        return true
                    }
                }
                false
            }
        }
    }
    /// Get the size of the file or containing files recursively
    fn get_size(&self) -> usize {
        match self {
            Self::File(_, size) => *size,
            Self::Directory(_, data) => data.iter().map(|el| el.get_size()).sum()
        }
    }
    /// Find all directories which size is at most X and sum them
    /// 
    /// # Arguments
    /// * `x` - The maximal size to look for
    fn find_all_directories_at_most_x_and_sum(&self, x: usize) -> usize {
        let mut sum = 0;
        sum + match self {
            Self::Directory(_, data) => {
                if self.get_size() <= x {
                    sum += self.get_size();
                }
                for d in data {
                    sum += d.find_all_directories_at_most_x_and_sum(x);
                }
                sum
            },
            Self::File(_, _) => 0
        }
    }
    /// Find the smallest directory to delete which is bigger then space_needed
    /// 
    /// # Arguments
    /// * `space_needed` - The space needed to free
    fn find_directory_to_delete(&self, space_needed: usize) -> usize {
        match self {
            Self::Directory(_, data) => {
                let mut possible = 0;
                if self.get_size() >= space_needed {
                    possible = self.get_size();
                }
                for d in data {
                    let tmp = d.find_directory_to_delete(space_needed);
                    if tmp >= space_needed && (possible == 0 || possible > tmp) {
                        possible = tmp;
                    }
                }
                possible
            },
            Self::File(_, _) => 0
        }
    }
}


/// A Filesystem containing a root Data (Hopefully a directory)
#[derive(Debug)]
struct Filesystem {
    root: Data
}

impl Filesystem {
    /// Create a file system as described in the terminal output
    /// 
    /// # Arguments
    /// * `terminal` - The terminal output puzzle input
    fn create_from_terminal_output(terminal: &str) -> Self {
        let mut current_dirs: Vec<String> = Vec::new();

        let mut data: Option<Data> = None;

        let mut in_ls_cmd = false;

        for line in terminal.split('\n') {
            if line.starts_with("$ ") {
                in_ls_cmd = false;

                let parts = line.split(' ').collect::<Vec<&str>>();
                match parts[1] {
                    "cd" => {
                        if data.is_none() {
                            data = Some(Data::Directory(parts[2].to_string(), Vec::new()));
                        }
                        if parts[2] == ".." {
                            current_dirs.pop();
                        }
                        else {
                            current_dirs.push(parts[2].to_string());
                        }
                    },
                    "ls" => {
                        in_ls_cmd = true;
                    }
                    _ => {panic!("Unknown command");}
                }
            }
            else if in_ls_cmd {
                if line.starts_with("dir") {
                    let parts = line.split(' ').collect::<Vec<&str>>();
                    let d = Data::Directory(parts[1].to_string(), Vec::new());
                    data.as_mut().unwrap().add(current_dirs.iter().skip(1).cloned(), d);
                }
                else {
                    let parts = line.split(' ').collect::<Vec<&str>>();
                    let d = Data::File(parts[1].to_string(), parts[0].parse().unwrap());
                    data.as_mut().unwrap().add(current_dirs.iter().skip(1).cloned(), d);
                }
            }
            else {
                panic!("Unexpected line");
            }
        }
        
        Self { root: data.unwrap() }
    }
    /// Find all directories in the root which size is at most X and sum them
    /// 
    /// # Arguments
    /// * `x` - The maximal size to look for
    fn find_all_directories_at_most_x_and_sum(&self, x: usize) -> usize {
        self.root.find_all_directories_at_most_x_and_sum(x)
    }
    /// Find the smallest directory in the root to delete which is bigger then space_needed
    /// 
    /// # Arguments
    /// * `space_needed` - The space needed to free
    fn find_directory_to_delete(&self, space_needed: usize) -> usize {
        self.root.find_directory_to_delete(space_needed)
    }
}


fn main() {

    let input = fs::read_to_string("input").unwrap_or_else(|_| panic!("Unable to read input"));

    let fs = Filesystem::create_from_terminal_output(&input);
    println!("The total size is {}", fs.find_all_directories_at_most_x_and_sum(100000));

    let space_needed = 30000000 - (70000000 - fs.root.get_size());
    println!("The directory to delete has a size of {}", fs.find_directory_to_delete(space_needed));
}


#[cfg(test)]
mod tests {
    use crate::Filesystem;

    #[test]
    fn check_against_example() {
        let fs = Filesystem::create_from_terminal_output("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k");
        assert_eq!(fs.find_all_directories_at_most_x_and_sum(100000), 95437);

        let space_needed = 30000000 - (70000000 - fs.root.get_size());
        assert_eq!(space_needed, 8381165);
        assert_eq!(fs.find_directory_to_delete(space_needed), 24933642);
    }
}