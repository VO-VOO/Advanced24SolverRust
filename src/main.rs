mod solver;

use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use serde::Serialize;

const WORKER_COUNT: usize = 8;

#[derive(Serialize)]
struct Metadata {
    target: u32,
    tolerance: f64,
    worker_count: usize,
    combination_count: usize,
}

#[derive(Serialize)]
struct Record {
    cards: [u8; 4],
    cards_key: String,
    solved: bool,
    expression: String,
}

#[derive(Serialize)]
struct Payload {
    metadata: Metadata,
    results: Vec<Record>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let combinations = all_combinations();
    let worker_count = configured_worker_count();

    let pool = ThreadPoolBuilder::new().num_threads(worker_count).build()?;
    let mut records = pool.install(|| {
        combinations
            .par_iter()
            .map(|cards| {
                let expression =
                    solver::solve_cards(*cards).unwrap_or_else(|| String::from("无解"));
                Record {
                    cards: *cards,
                    cards_key: cards_key(cards),
                    solved: expression != "无解",
                    expression,
                }
            })
            .collect::<Vec<_>>()
    });

    records.sort_unstable_by_key(|record| record.cards);

    let payload = Payload {
        metadata: Metadata {
            target: 24,
            tolerance: solver::EPS,
            worker_count,
            combination_count: records.len(),
        },
        results: records,
    };

    let output_path = project_root().join("results.json");
    fs::write(&output_path, serde_json::to_string_pretty(&payload)?)?;

    println!(
        "完成 {} 种组合计算，结果已写入 {}",
        combinations.len(),
        output_path.file_name().and_then(|name| name.to_str()).unwrap_or("results.json")
    );
    println!("总耗时: {:.6} 秒", start.elapsed().as_secs_f64());

    Ok(())
}

fn configured_worker_count() -> usize {
    std::env::var("A24_THREADS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(WORKER_COUNT)
}

fn all_combinations() -> Vec<[u8; 4]> {
    let mut combinations = Vec::with_capacity(1820);
    for first in 1..=13 {
        for second in first..=13 {
            for third in second..=13 {
                for fourth in third..=13 {
                    combinations.push([first, second, third, fourth]);
                }
            }
        }
    }
    combinations
}

fn cards_key(cards: &[u8; 4]) -> String {
    format!("{},{},{},{}", cards[0], cards[1], cards[2], cards[3])
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[cfg(test)]
mod tests {
    use super::{WORKER_COUNT, all_combinations, cards_key, configured_worker_count};

    #[test]
    fn generates_expected_combination_count() {
        assert_eq!(all_combinations().len(), 1820);
    }

    #[test]
    fn formats_cards_key() {
        assert_eq!(cards_key(&[1, 1, 7, 7]), "1,1,7,7");
    }

    #[test]
    fn worker_count_matches_requirement() {
        assert_eq!(WORKER_COUNT, 8);
    }

    #[test]
    fn invalid_env_keeps_default_worker_count() {
        unsafe {
            std::env::set_var("A24_THREADS", "0");
        }
        assert_eq!(configured_worker_count(), WORKER_COUNT);
        unsafe {
            std::env::remove_var("A24_THREADS");
        }
    }
}
