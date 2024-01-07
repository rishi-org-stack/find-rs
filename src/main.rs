use std::{
    collections::VecDeque,
    env::{self, args},
    path::Path,
};

enum SearchAlgo {
    Dfs,
    Bfs,
}
struct Config {
    search_algo: SearchAlgo,
}

impl Config {
    fn load() -> Config {
        let search_algo = env::var("FIND_SEARCH_ALGO").unwrap_or("dfs".to_string());

        let algo = match search_algo.as_str() {
            "bfs" => SearchAlgo::Bfs,
            _ => SearchAlgo::Dfs,
        };
        Config { search_algo: algo }
    }
}

fn find_dfs(dir: &str, to_find: &str) {
    let mut dir_q: VecDeque<String> = VecDeque::new();
    dir_q.push_back(dir.to_string());
    loop {
        if dir_q.is_empty() {
            break;
        }

        let dir_path_str: String = dir_q.pop_back().unwrap();

        let dir_path = Path::new(dir_path_str.as_str());

        dir_path.read_dir().unwrap().into_iter().for_each(|f| {
            let path = f.unwrap().path();
            if path.is_dir() {
                dir_q.push_back(path.to_str().unwrap().to_string());
            } else {
                let file_name = path.to_str().unwrap();
                let files: Vec<&str> = file_name.split("/").collect();
                if files.last().unwrap().eq(&to_find.to_string()) {
                    println!("path: {}", path.to_str().unwrap())
                }
            }
        })
    }
}

fn find_bfs(dir: &str, to_find: &str) {
    let mut dir_q: VecDeque<String> = VecDeque::new();
    dir_q.push_back(dir.to_string());
    loop {
        if dir_q.is_empty() {
            break;
        }

        let dir_path_str: String = dir_q.pop_front().unwrap();

        let dir_path = Path::new(dir_path_str.as_str());

        dir_path.read_dir().unwrap().into_iter().for_each(|f| {
            let path = f.unwrap().path();
            if path.is_dir() {
                dir_q.push_back(path.to_str().unwrap().to_string());
            } else {
                let file_name = path.to_str().unwrap();
                let files: Vec<&str> = file_name.split("/").collect();
                if files.last().unwrap().eq(&to_find.to_string()) {
                    println!("path: {}", path.to_str().unwrap())
                }
            }
        })
    }
}

fn main() {
    let arguments: Vec<String> = args().map(String::from).collect();
    let args_size = arguments.len();
    if args_size < 2 {
        println!("please add which file to find");
        return;
    }

    let conf = Config::load();

    let dir_and_find_tuple = match args_size {
        2 => {
            let to_find = arguments[1].as_str();
            (".", to_find)
        }
        3 => {
            let dir: &str = arguments[1].as_str();
            let to_find: &str = arguments[2].as_str();
            (dir, to_find)
        }
        _ => {
            println!("only to args required");
            return;
        }
    };
    match conf.search_algo {
        SearchAlgo::Bfs => {
            find_bfs(dir_and_find_tuple.0, dir_and_find_tuple.1);
        }
        _ => {
            find_dfs(dir_and_find_tuple.0, dir_and_find_tuple.1);
        }
    }
}
