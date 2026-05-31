use super::types::CellType;

const ALL_PIECES: [CellType; 7] = [
    CellType::I,
    CellType::O,
    CellType::T,
    CellType::S,
    CellType::Z,
    CellType::J,
    CellType::L,
];

#[derive(Clone)]
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }
}

pub struct Bag {
    queue: Vec<CellType>,
    rng: Lcg,
}

impl Bag {
    pub fn new(seed: u64) -> Self {
        let mut bag = Bag {
            queue: Vec::new(),
            rng: Lcg(seed),
        };
        bag.refill();
        bag
    }

    /// Reconstruct a Bag from serialised state (remaining queue + RNG seed).
    pub fn from_parts(queue: Vec<CellType>, rng_seed: u64) -> Self {
        Bag {
            queue,
            rng: Lcg(rng_seed),
        }
    }

    pub fn remaining(&self) -> &[CellType] {
        &self.queue
    }

    pub fn rng_seed(&self) -> u64 {
        self.rng.0
    }

    fn refill(&mut self) {
        let mut next_bag = ALL_PIECES.to_vec();
        for i in (1..7).rev() {
            let j = (self.rng.next() as usize) % (i + 1);
            next_bag.swap(i, j);
        }
        self.queue.extend(next_bag);
    }

    pub fn next(&mut self) -> CellType {
        if self.queue.len() <= 7 {
            self.refill();
        }
        self.queue.remove(0)
    }

    pub fn peek(&self, n: usize) -> Vec<CellType> {
        self.queue.iter().take(n).copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_pieces() {
        let mut bag = Bag::new(42);
        for _ in 0..14 {
            let p = bag.next();
            assert!(ALL_PIECES.contains(&p), "unexpected piece {p:?}");
        }
    }

    #[test]
    fn test_peek_does_not_advance() {
        let bag = Bag::new(42);
        let a = bag.peek(3);
        let b = bag.peek(3);
        assert_eq!(a, b);
    }

    #[test]
    fn test_each_bag_has_all_7() {
        let mut bag = Bag::new(42);
        for _ in 0..5 {
            let mut seen = [false; 7];
            for _ in 0..7 {
                let p = bag.next();
                let idx = ALL_PIECES.iter().position(|&x| x == p).unwrap();
                seen[idx] = true;
            }
            assert!(seen.iter().all(|&x| x), "bag missing pieces");
        }
    }

    #[test]
    fn test_deterministic_seed() {
        let mut a = Bag::new(42);
        let mut b = Bag::new(42);
        for _ in 0..21 {
            assert_eq!(a.next(), b.next());
        }
    }

    #[test]
    fn test_different_seed_different_sequence() {
        let mut a = Bag::new(42);
        let mut b = Bag::new(99);
        let mut same = true;
        for _ in 0..14 {
            if a.next() != b.next() {
                same = false;
                break;
            }
        }
        assert!(!same, "seeds 42 and 99 should differ");
    }

    #[test]
    fn test_peek_respects_n() {
        let bag = Bag::new(42);
        assert_eq!(bag.peek(0).len(), 0);
        assert_eq!(bag.peek(5).len(), 5);
        // We refill at len <= 7, so initial peek can go up to 7 easily
        let many = bag.peek(20);
        assert!(many.len() <= 14);
    }
}
