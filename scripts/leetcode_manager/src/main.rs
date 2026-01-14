use regex::Regex;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
struct Solution {
    id: String,
    title: String,
    docs: Vec<(String, String)>, // (Label, Path)
    lang: String,
    // Metadata from file content could be added here
    difficulty: String,
    time: String,
    space: String,
}

impl Solution {
    fn slug_to_title(slug: &str) -> String {
        slug.split('-')
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn main() {
    let solutions_dir = Path::new("../../solutions");
    let readme_path = Path::new("../../README.md");

    if !solutions_dir.exists() {
        eprintln!("Solutions directory not found at {:?}", solutions_dir);
        return;
    }

    let solutions = collect_solutions(solutions_dir);
    let table = generate_table(&solutions);
    update_readme(readme_path, &table);
}

fn extract_metadata(path: &Path) -> (String, String, String) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return ("-".to_string(), "-".to_string(), "-".to_string()),
    };

    let diff_re = Regex::new(r#"difficulty: "([^"]+)""#).unwrap();
    let time_re = Regex::new(r#"time: "([^"]+)""#).unwrap();
    let space_re = Regex::new(r#"space: "([^"]+)""#).unwrap();

    let difficulty = diff_re.captures(&content)
        .map(|c| c[1].to_string())
        .unwrap_or_else(|| "-".to_string());
    
    let time = time_re.captures(&content)
        .map(|c| c[1].to_string())
        .unwrap_or_else(|| "-".to_string());

    let space = space_re.captures(&content)
        .map(|c| c[1].to_string())
        .unwrap_or_else(|| "-".to_string());

    (difficulty, time, space)
}

fn collect_solutions(root: &Path) -> Vec<Solution> {
    let mut solutions_map: BTreeMap<String, Solution> = BTreeMap::new();
    let mut languages_map: BTreeMap<String, HashSet<String>> = BTreeMap::new();
    
    // Regex for ID-Slug.ext (e.g. 0001-two-sum.rs, 0001-two-sum-es.md)
    let re = Regex::new(r"(\d{4})-(.+)\.(\w+)").unwrap();

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                if let Some(caps) = re.captures(filename) {
                    let id = caps[1].to_string();
                    let slug = caps[2].to_string();
                    let ext = caps[3].to_string();

                    let relative_path = path.strip_prefix("../..").unwrap_or(path).to_string_lossy().to_string();

                    if ext == "md" {
                         let label = if slug.ends_with("-es") {
                             format!("{} - ES", Solution::slug_to_title(slug.trim_end_matches("-es")))
                         } else if slug.ends_with("-en") {
                             format!("{} - EN", Solution::slug_to_title(slug.trim_end_matches("-en")))
                         } else {
                             // Fallback for generic MD names
                             Solution::slug_to_title(&slug)
                         };

                        let (diff, time, space) = extract_metadata(path);

                        solutions_map.entry(id.clone())
                            .and_modify(|s| {
                                s.docs.push((label.clone(), relative_path.clone()));
                                if s.difficulty == "-" && diff != "-" { s.difficulty = diff.clone(); }
                                if s.time == "-" && time != "-" { s.time = time.clone(); }
                                if s.space == "-" && space != "-" { s.space = space.clone(); }
                            })
                            .or_insert(Solution {
                                id: id.clone(),
                                title: Solution::slug_to_title(&slug), // Temporary title
                                docs: vec![(label, relative_path)],
                                lang: String::new(),
                                difficulty: diff,
                                time: time,
                                space: space,
                            });
                        continue;
                    }

                    // Rudimentary language mapping
                    let lang = match ext.as_str() {
                        "rs" => "Rust",
                        "c" => "C",
                        "cpp" => "C++",
                        "py" => "Python",
                        "java" => "Java",
                        _ => &ext,
                    }.to_string();

                    let lang_link = format!("[{}]({})", lang, relative_path);
                    languages_map.entry(id.clone()).or_default().insert(lang_link);

                    solutions_map.entry(id.clone()).or_insert(Solution {
                        id: id.clone(),
                        title: Solution::slug_to_title(&slug),
                        docs: vec![], 
                        lang: String::new(), 
                        difficulty: "-".to_string(),
                        time: "-".to_string(),
                        space: "-".to_string(),
                    });
                }
            }
        }
    }

    // Convert map to vec and sort/inject languages
    solutions_map.into_values().map(|mut s| {
        if let Some(langs) = languages_map.get(&s.id) {
             let mut sorted_langs: Vec<_> = langs.iter().cloned().collect();
             sorted_langs.sort();
             s.lang = sorted_langs.join(", ");
        }
        // Ensure Docs are sorted nicely (e.g. EN then ES or alphabetical)
        s.docs.sort();
        // Fallback title if no docs found (shouldn't happen with proper structure)
        if s.docs.is_empty() {
            // Keep existing s.title
        } else {
            // If we have docs, the title column will be constructed from them.
            // We don't really need s.title for display in that case.
        }
        s
    }).collect()
}

fn generate_table(solutions: &[Solution]) -> String {
    let mut table = String::new();
    // Header is already in README, we generate the rows.
    // Actually the user wants the script to populate the table.
    // The marker wraps the WHOLE table, so we regenerate header too to be safe/consistent?
    // User requested "Script to generate... table".
    // My marker was: 
    // <!-- SOLUTIONS_TABLE_START -->
    // | ID | ...
    // <!-- SOLUTIONS_TABLE_END -->
    // So I should replace everything between markers.

    table.push_str("| ID | Title | Difficulty | Time | Space | Languages |\n");
    table.push_str("| -- | ----- | ---------- | ---- | ----- | --------- |\n");

    for s in solutions {
        let title_cell = if s.docs.is_empty() {
             s.title.clone()
        } else {
             s.docs.iter()
                 .map(|(label, path)| format!("[{}]({})", label, path))
                 .collect::<Vec<_>>()
                 .join("<br>")
        };

        table.push_str(&format!("| {} | {} | {} | {} | {} | {} |\n", 
            s.id, title_cell, s.difficulty, s.time, s.space, s.lang));
    }

    table
}

fn update_readme(path: &Path, table_content: &str) {
    let content = fs::read_to_string(path).expect("Could not read README.md");
    let start_marker = "<!-- SOLUTIONS_TABLE_START -->";
    let end_marker = "<!-- SOLUTIONS_TABLE_END -->";

    if let Some(start_idx) = content.find(start_marker) {
        if let Some(end_idx) = content.find(end_marker) {
            let mut new_content = String::with_capacity(content.len());
            new_content.push_str(&content[..start_idx + start_marker.len()]);
            new_content.push('\n');
            new_content.push_str(table_content);
            new_content.push_str(&content[end_idx..]);
            
            fs::write(path, new_content).expect("Could not write README.md");
            println!("README.md updated successfully.");
        } else {
            eprintln!("End marker not found in README.md");
        }
    } else {
        eprintln!("Start marker not found in README.md");
    }
}
