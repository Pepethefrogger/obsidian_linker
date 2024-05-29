use std::{cmp::min, fs::{self, DirEntry}, sync::Arc, thread::{self, JoinHandle, Thread}};
use regex::{Regex, RegexBuilder};

fn main() {
    let threads = 10
    ;
    let directory = "/home/parrot/Documents/Hacking-Database";
    let files: Vec<_> = fs::read_dir(directory).unwrap().map(|e| e.unwrap()).filter(|f| f.file_type().unwrap().is_file()).collect();
    let names: Vec<_> = files.iter().map(|f| String::from(f.file_name().clone().into_string().unwrap().split(".").next().unwrap())).collect();
    let rp = r"[.,;\n]?";
    let regex: Vec<_> = names.iter().map(|name| (name.clone(),RegexBuilder::new(&format!("^{rp}{0}{rp}$",name.escape_unicode())).case_insensitive(true).build().unwrap())).collect();

    let count = files.iter().count();
    let mut t: Vec<JoinHandle<()>> = vec![];
    let file_arc = Arc::new(files);
    let regex_arc = Arc::new(regex);
    for i in 0..threads {
        let num = count.div_ceil(threads);
        let index_low = i*num;
        let index_high = min(count, (i+1)*num);
        let file_ref = file_arc.clone();
        let regex_ref = regex_arc.clone();
        t.push(thread::spawn(move || process(&file_ref[index_low..index_high],&regex_ref[..])));
    };

    for thread in t {
        let _ = thread.join().unwrap();
    };
}

fn process(files: &[DirEntry],regex: &[(String, Regex)]) {
    for file in files.iter() {
        let text = fs::read_to_string(file.path()).unwrap();
        let mut words: Vec<_> = text.split(" ").map(|s| String::from(s)).collect();
        let mut single_nested = false;
        let mut triple_nested = false;

        for i in 0..words.len() {
            let word = &words[i];
            if word.contains("```") {
                triple_nested = !triple_nested;
                continue;
            }
            if word.contains("`") {
                if word.match_indices("`").count()%2==0 {
                    continue;
                };
                single_nested = !single_nested;
                continue;
            }
            if !single_nested && !triple_nested {
                for (name,reg) in regex {
                    if reg.is_match(&words[i]) {
                        words[i] = reg.replace(&words[i],&format!("[[{name}]]")).to_string();
                        break;
                    }
                }
            }
        }
        let _ = fs::write(file.path(), words.join(" "));
    }
}
