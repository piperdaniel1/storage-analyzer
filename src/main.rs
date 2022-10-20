use std::env;
use std::fs;

#[derive(Debug)]
struct Config {
    custom_path: String,
}

#[derive(Debug)]
struct File {
    size: u64,
    path: String,
}

// Represents a directory in the file tree
#[derive(Debug)]
struct TreeNode {
    // Path to the directory
    path: String,

    // Path to the directory above this one
    upward_path: String,

    // Size that all files from this node down take up
    size: u64,

    // Signifies whether this node is the root node of the tree
    is_root: bool,

    // List of directory paths that are contained in this directory
    dir_list: Vec<TreeNode>,

    // List of files in that directory
    file_list: Vec<File>,
}

fn get_ind(input_vec: &Vec<String>, target: String) -> i32 {
    for i in 0..input_vec.len() {
        if input_vec[i] == target {
            return i as i32;
        }
    }

    return -1;
}

fn parse_args(args: Vec<String>) -> Config {
    let mut run_config = Config {custom_path: String::from("/")};

    let path_ind = get_ind(&args, String::from("--path"));
    if path_ind != -1 {
        if path_ind >= args.len() as i32 - 1 {
            println!("Argument error, provided --path flag without path.")
        } else {
            run_config.custom_path = String::clone(&args[path_ind as usize +1]);
        }
    }

    println!("{args:?}");

    run_config
}

fn index_file_tree(curr_path: String, old_path: String) -> TreeNode {
    let mut curr_node = TreeNode {
        path: String::from(&curr_path),
        upward_path: old_path,
        size: 0,
        is_root: false,
        dir_list: Vec::new(),
        file_list: Vec::new()
    };

    if curr_node.upward_path == String::from("") {
        curr_node.is_root = true;
    }

    let paths = match fs::read_dir(&curr_path) {
        Ok(paths) => paths,
        Err(_) => panic!("Problem reading directory!"),
    };

    for path in paths.into_iter() {
        let path = match path {
            Ok(path) => path,
            Err(error) => {
                println!("Unable to read entry due to {error}");
                continue;
            },
        };

        let path_str = match path.path().into_os_string().into_string() {
            Ok(path_str) => path_str,
            Err(error) => {
                println!("Unable to read path due to {error:?}");
                continue;
            }
        };

        let metadata = match path.metadata() {
            Ok(metadata) => metadata,
            Err(error) => {
                println!("Unable to read metadata due to {error}");
                continue;
            }
        };

        if metadata.is_dir() {
            curr_node.dir_list.push(index_file_tree(path_str, String::clone(&curr_path)));
        } else {
            curr_node.file_list.push(File {path: path_str, size: metadata.len()});
        }
    }

    for dir in &curr_node.dir_list {
        curr_node.size += dir.size;
    }

    for file in &curr_node.file_list {
        curr_node.size += file.size;
    }

    return curr_node;
}

fn pp_bytes(bytes: u64) -> String {
    let base_str = String::from(bytes.to_string());

    return match bytes {
        0..=999 => base_str + " B",
        1000..=999999 => (bytes/1000).to_string() + " KB",
        1000000..=999999999 => (bytes/1000000).to_string() + " MB",
        1000000000..=999999999999 => (bytes/1000000000).to_string() + " GB",
        1000000000000..=999999999999999 => (bytes/1000000000).to_string() + " TB",
        _ => String::from(bytes.to_string()),
    }
}

fn visualize_tree(root_node: &mut TreeNode) {
    cls();

    if !root_node.is_root {
        println!("==== {} ====", root_node.path);
    } else {
        println!("==== {} (root) ====", root_node.path);
    }

    println!("Directory size: {}", pp_bytes(root_node.size));

    root_node.dir_list.sort_by(|a, b| b.size.cmp(&a.size)); 

    println!("\nSubdirectories:");
    for dir in &root_node.dir_list {
        let dir_name: Vec<&str> = dir.path.split("/").collect();
        let dir_name = dir_name[dir_name.len()-1];

        println!(" - {}, size: {}", dir_name, pp_bytes(dir.size));
    }

    println!("\nFiles:");
    for file in &root_node.file_list {
        let file_name: Vec<&str> = file.path.split("/").collect();
        let file_name = file_name[file_name.len()-1];
        println!(" - {}, size: {}", file_name, pp_bytes(file.size));
    }
}

fn cls() {
    print!("{esc}c", esc = 27 as char);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let run_config = parse_args(args);
    println!("{run_config:?}");

    let mut root_node = index_file_tree(run_config.custom_path, String::from(""));

    visualize_tree(&mut root_node);

    /*let paths = fs::read_dir(run_config.custom_path);
    let paths = match paths {
        Ok(paths) => paths,
        Err(_) => panic!("Problem reading directory!"),
    };

    for path in paths.into_iter() {
        let path = match path {
            Ok(path) => path,
            Err(error) => {
                println!("Unable to read entry due to {error}");
                continue;
            },
        };

        let path_str = match path.path().into_os_string().into_string() {
            Ok(path_str) => path_str,
            Err(error) => {
                println!("Unable to read path due to {error:?}");
                continue;
            }
        };

        println!("\nEntry:\n{}", path_str);
        let metadata = match path.metadata() {
            Ok(metadata) => metadata,
            Err(error) => {
                println!("Unable to read metadata due to {error}");
                continue;
            }
        };

        
        println!("{:?}", metadata.len());
    }*/
}
