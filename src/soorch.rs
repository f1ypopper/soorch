use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
type TokenCount = HashMap<String, usize>;
type Index = HashMap<PathBuf, TokenCount>;

fn tokenize(text: String) -> Vec<String> {
    let mut chars = text.chars().peekable();
    let mut tokens = Vec::new();
    while chars.peek().is_some() {
        if chars.next_if_eq(&'<').is_some() {
            chars.borrow_mut().take_while(|&c| c != '>').for_each(drop);
        } else if chars.peek().unwrap().is_alphanumeric() {
            let token = chars
                .borrow_mut()
                .take_while(|c| c.is_alphanumeric())
                .collect::<String>();
            tokens.push(token);
        } else {
            chars.next();
        }
    }
    tokens
}

fn calculate_token_count(text: String) -> TokenCount {
    //Calculates Term Count for a Text File
    let mut token_count = TokenCount::new();
    let mut chars = text.chars().peekable();

    while chars.peek().is_some() {
        if chars.next_if_eq(&'<').is_some() {
            chars.borrow_mut().take_while(|&c| c != '>').for_each(drop);
        } else if chars.peek().unwrap().is_alphanumeric() {
            let token = chars
                .borrow_mut()
                .take_while(|c| c.is_alphanumeric())
                .collect::<String>();

            token_count
                .entry(token)
                .and_modify(|freq| *freq += 1)
                .or_insert(1);
        } else {
            chars.next();
        }
    }
    token_count
}

pub fn create_index(dir_path: &String) -> io::Result<Index> {
    let mut file_paths = Vec::<PathBuf>::new();
    let index = Arc::new(Mutex::new(Index::new()));
    let mut threads = vec![];
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        if !entry.path().is_dir() {
            file_paths.push(entry.path());
        }
    }
    for file_path in file_paths {
        let index = Arc::clone(&index);
        let handle = thread::spawn(move || -> Result<(), io::Error> {
            let text = fs::read_to_string(&file_path)?;
            let tf = calculate_token_count(text);
            index.lock().unwrap().insert(file_path, tf);
            Ok(())
        });
        threads.push(handle);
    }
    for handle in threads {
        let _ = handle.join();
    }
    let unwrapped_index = Arc::try_unwrap(index).unwrap().into_inner().unwrap();
    Ok(unwrapped_index)
}

pub fn write_index(index: &Index, index_path: &String) -> io::Result<()> {
    fs::write(index_path, serde_json::to_string_pretty(index).unwrap())?;
    Ok(())
}

fn calculate_term_frequency(term: &String, tc: &TokenCount) -> f32 {
    (tc.get(term).unwrap_or(&0) / tc.values().sum::<usize>()) as f32
}

fn calculate_inverse_document_frequency(term: &String, index: &Index) -> f32 {
    let mut nd = index.iter().filter(|(_, v)| v.get(term).is_some()).count();
    nd = if nd < 1 { 1 } else { nd }; // Add one if nd is 0 (avoid div by 0)
    ((index.len() / nd) as f32).log10()
}

pub fn search_and_rank<'a>(phrase: String, index: &'a Index) -> Vec<(&'a str, f32)> {
    let terms = tokenize(phrase);
    let mut doc_rankings = Vec::<(&str, f32)>::new();
    for (doc_name, tc) in index {
        let mut rank = 0 as f32;
        for term in &terms {
            rank += calculate_term_frequency(term, &tc)
                * calculate_inverse_document_frequency(term, index);
        }
        doc_rankings.push((doc_name.to_str().unwrap(), rank))
    }
    doc_rankings.sort_by(|a, b| a.partial_cmp(b).unwrap());
    doc_rankings.reverse();
    doc_rankings
}

pub fn run_server(path: &String, address: &String) {
    unimplemented!()
}

