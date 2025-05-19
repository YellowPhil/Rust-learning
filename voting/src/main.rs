use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Candidate(String);

#[derive(Debug)]
struct Voting {
    is_finished: bool,
    winner: Candidate,
    votes: HashMap<Candidate, usize>,
}

impl Voting {
    pub fn new(can: &[Candidate]) -> Voting {
        Voting {
            is_finished: false,
            winner: Candidate("".to_string()),
            votes: HashMap::from_iter(can.iter().map(|c| (c.clone(), 0))),
        }
    }
    pub fn vote(&mut self, c: &Candidate) {
        match self.votes.get_mut(c) {
            Some(count) => *count += 1,
            None => {}
        }
    }
    pub fn finish_vote(&mut self) {
        self.is_finished = true;
        self.winner = self.votes.iter().max_by(|c1, c2| c1.1.cmp(c2.1)).unwrap().0.clone()
    }
    pub fn get_winner(&self) -> Candidate {
        self.winner.clone()
    }
}

fn main() {
    let candidates = vec![
        Candidate("Alice".to_string()),
        Candidate("Bob".to_string()),
        Candidate("Charlie".to_string()),
    ];
    let mut voting = Voting::new(&candidates);

    for _ in 1..1000 {
        voting.vote(&candidates[rand::random_range(0..candidates.len())])
    }

    voting.finish_vote();

    println!("{:?}", voting.get_winner().0);
}
