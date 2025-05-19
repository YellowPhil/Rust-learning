use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Candidate(String);

struct Voting {
    votes: HashMap<Candidate, usize>,
}

impl Voting {
    pub fn new(_candidates: &[Candidate]) {
        let mut voting = Voting{
            votes: HashMap::with_capacity(_candidates.len())
        };

        for candidate in _candidates {
            voting.votes.insert(candidate.clone(), 0);
        }
    }
}

fn main() {
    let candidates = vec![
        Candidate("Alice".to_string()),
        Candidate("Bob".to_string()),
        Candidate("Charlie".to_string()),
    ];
    let voting = Voting::new(&candidates);
}