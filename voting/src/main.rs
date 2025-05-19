use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Candidate(String);

struct Voting {
    votes: HashMap<Candidate, usize>,
}

impl Voting {
    pub fn new(can: &[Candidate]) -> Voting {
        Voting {
            votes: HashMap::from_iter(can.iter().map(|c| (c.clone(), 0))),
        }
    }
    pub fn vote(&mut self, c: Candidate) {
        match self.votes.get_mut(&c) {
            Some(c) => *c += 1,
            None => {}
        }
    }
}

fn main() {}
