use std::fs;
use regex::RegexBuilder;

fn main() {
    let directory = "/home/parrot/Documents/Hacking-Database";
    let files: Vec<_> = fs::read_dir(directory).unwrap().map(|e| e.unwrap()).filter(|f| f.file_type().unwrap().is_file()).collect();
    let names: Vec<_> = files.iter().map(|f| String::from(f.file_name().clone().into_string().unwrap().split(".").next().unwrap())).collect();
    let rp = r"[.,;\n]?";
    let regex: Vec<_> = names.iter().map(|name| (name,RegexBuilder::new(&format!("^{rp}{0}{rp}$",name.escape_unicode())).case_insensitive(true).build().unwrap())).collect();

    for (index,file) in files.iter().enumerate() {
        println!("{}",index);
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
                for (name,reg) in &regex {
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
