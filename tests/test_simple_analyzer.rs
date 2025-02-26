use std::collections::HashMap;

use arxiv::Arxiv;
use rand::seq::SliceRandom;
use rand::Rng;
use sciffer_rs::analyzers::simple::SimpleArixvTrendingAnalyzerBuilder;
use sciffer_rs::analyzers::TrendingAnalyzer;
use sciffer_rs::extracters::topic::TopicData;

fn generate_random_string(_: usize) -> String {
    let keywords = vec![
        "machine learning",
        "image recogniton",
        "artificial intelligence",
        "program debugging",
        "large language models",
    ];
    keywords
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}

fn generate_dummy_arxiv() -> Arxiv {
    Arxiv {
        id: format!("{}{}", generate_random_string(4), generate_random_string(4)), // Random Arxiv ID
        updated: format!(
            "2025-02-14T{:02}:00:00Z",
            rand::thread_rng().gen_range(0..24)
        ), // Random hour for update
        published: format!(
            "2025-02-01T{:02}:00:00Z",
            rand::thread_rng().gen_range(0..24)
        ), // Random hour for publish
        title: generate_random_string(20),   // Random paper title
        summary: generate_random_string(50), // Random summary
        authors: (0..rand::thread_rng().gen_range(1..4)) // Random number of authors (1-3)
            .map(|_| generate_random_string(10)) // Random author names
            .collect(),
        pdf_url: format!("https://arxiv.org/pdf/{}.pdf", generate_random_string(8)), // Random PDF URL
        comment: if rand::random::<bool>() {
            Some(generate_random_string(30)) // Random comment (optional)
        } else {
            None
        },
    }
}

fn generate_dummy_topicdata() -> TopicData {
    TopicData {
        title: generate_random_string(15), // Random topic title
        solved_problem: (0..rand::thread_rng().gen_range(1..4)) // Random number of problems (1-3)
            .map(|_| generate_random_string(25)) // Random solved problems
            .collect(),
        research_field: (0..rand::thread_rng().gen_range(1..3)) // Random number of fields (1-2)
            .map(|_| generate_random_string(12)) // Random research fields
            .collect(),
        techniques_used: (0..rand::thread_rng().gen_range(1..4)) // Random number of techniques (1-3)
            .map(|_| generate_random_string(15)) // Random techniques used
            .collect(),
    }
}

fn trending_problems_round(cnt: i32) {
    let cnt = 5;
    let mut data = vec![];
    (0..cnt).into_iter().for_each(|_| {
        data.push((generate_dummy_arxiv(), generate_dummy_topicdata()));
    });

    let mut cnt: HashMap<String, i32> = HashMap::new();
    data.iter().for_each(|(_, x)| {
        x.solved_problem.iter().for_each(|s| {
            let entry = cnt.entry(s.to_string()).or_insert(0);
            *entry += 1
        });
    });

    let mut oracle = vec![];
    for (k, v) in cnt {
        oracle.push((k, v));
    }
    oracle.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));

    let analyzer = SimpleArixvTrendingAnalyzerBuilder::default()
        .build()
        .unwrap();
    let res = analyzer.top_k(&data, |x| x.research_field.clone());

    // println!("{:#?}", res);

    for (x, y) in oracle.iter().zip(&res) {
        assert!(x.0 == y.0, "{:?}, {:?}", oracle, res)
    }
}

#[test]
fn test_trending_problems() {
    let epoch = 20;
    (0..epoch)
        .into_iter()
        .for_each(|_| trending_problems_round(epoch));
}
