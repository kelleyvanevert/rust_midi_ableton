use im::Vector;
use std::ops::{Add, Mul};

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    pub data: Vector<u8>,
}

#[allow(unused)]
impl Pattern {
    pub fn one() -> Pattern {
        Self {
            data: Vector::from(vec![1u8]),
        }
    }

    pub fn zero() -> Pattern {
        Self {
            data: Vector::from(vec![0u8]),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn rotate(&self, mut by: i32) -> Pattern {
        let mut data = self.data.clone();
        while by != 0 {
            match by.signum() {
                1 => {
                    if let Some(b) = data.pop_front() {
                        data.push_back(b);
                    }
                    by -= 1;
                }
                -1 => {
                    if let Some(b) = data.pop_back() {
                        data.push_front(b);
                    }
                    by += 1;
                }
                _ => {}
            }
        }

        Self { data }
    }

    pub fn fill(&self, beats: usize) -> Pattern {
        if self.data.len() > beats {
            return Self {
                data: self.data.take(beats),
            };
        }

        let mut data = self.data.clone();
        for i in 0..(beats - self.data.len()) {
            data.push_back(*self.data.get(i % self.data.len()).unwrap());
        }

        Self { data }
    }

    pub fn fill_zeroes(&self, beats: usize) -> Pattern {
        if self.data.len() > beats {
            return Self {
                data: self.data.take(beats),
            };
        }

        let mut data = self.data.clone();
        for i in 0..(beats - self.data.len()) {
            data.push_back(0);
        }

        Self { data }
    }
}

impl From<Vec<u8>> for Pattern {
    fn from(vec: Vec<u8>) -> Self {
        Self {
            data: Vector::from(vec),
        }
    }
}

impl From<&str> for Pattern {
    fn from(str: &str) -> Self {
        let data = str
            .chars()
            .into_iter()
            .map(|char| if char == 'X' { 1u8 } else { 0u8 })
            .collect::<Vector<u8>>();

        Self { data }
    }
}

impl Add for Pattern {
    type Output = Pattern;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.data.clone();
        data.append(rhs.data.clone());

        Self { data }
    }
}

impl Mul<usize> for Pattern {
    type Output = Pattern;

    fn mul(self, rhs: usize) -> Self::Output {
        let mut data = Vector::new();
        for _ in 0..rhs {
            data.append(self.data.clone());
        }

        Pattern { data }
    }
}

impl IntoIterator for Pattern {
    type Item = u8;
    type IntoIter = PatternIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        PatternIntoIterator {
            pattern: self,
            index: 0,
        }
    }
}

#[derive(Clone)]
pub struct PatternIntoIterator {
    pattern: Pattern,
    index: usize,
}

impl Iterator for PatternIntoIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pattern.data.get(self.index) {
            None => None,
            Some(&byte) => {
                self.index += 1;
                Some(byte)
            }
        }
    }
}

// https://observablehq.com/@toja/euclidean-rhythms
pub fn euclid(mut pulses: usize, steps: usize) -> Pattern {
    if pulses > steps {
        pulses = steps;
    }

    let mut a = Pattern::one();
    let mut b = Pattern::zero();
    let mut k = pulses;
    let mut m = steps - pulses;

    loop {
        if k <= m {
            a = a + b.clone();
            m -= k;
        } else {
            (a, b) = (a.clone() + b, a);
            (k, m) = (m, k - m);
        }

        if m <= 1 || k <= 1 {
            break;
        }
    }

    (a * k) + (b * m)
}

#[test]
fn test_euclid() {
    assert_eq!(euclid(2, 3), Pattern::from("X.X"));
    assert_eq!(euclid(3, 4), Pattern::from("X.XX"));
    assert_eq!(euclid(3, 5), Pattern::from("X.X.X"));
    assert_eq!(euclid(3, 8), Pattern::from("X..X..X."));
    assert_eq!(euclid(5, 12), Pattern::from("X..X.X..X.X."));
    assert_eq!(euclid(13, 24), Pattern::from("X.XX.X.X.X.X.XX.X.X.X.X."));
}

#[test]
fn test_rotate() {
    assert_eq!(euclid(3, 4).rotate(1), Pattern::from(".XXX"));
    assert_eq!(euclid(3, 5).rotate(2), Pattern::from("X.XX."));
    assert_eq!(euclid(3, 5).rotate(-2), Pattern::from(".XX.X"));
}

#[test]
fn test_fill() {
    assert_eq!(euclid(3, 8).fill(5), Pattern::from("X..X."));
    assert_eq!(euclid(3, 8).fill(10), Pattern::from("X..X..X.X."));
    assert_eq!(euclid(3, 8).fill(15), Pattern::from("X..X..X.X..X..X"));
    assert_eq!(euclid(3, 8).fill(16), Pattern::from("X..X..X.X..X..X."));
    assert_eq!(euclid(3, 8).fill(20), Pattern::from("X..X..X.X..X..X.X..X"));
}
