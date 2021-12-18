use itertools::Itertools;
use std::fmt::Display;

pub fn solve() -> (usize, usize) {
    let trees = parse_input(include_str!("../resources/day18.txt"));
    (solve1(&trees), solve2(&trees))
}

fn parse_input(raw: &str) -> Vec<Tree> {
    raw.lines().map(Tree::from_str).collect()
}

fn solve1(trees: &[Tree]) -> usize {
    sum_trees(trees).magnitude(0)
}

fn solve2(trees: &[Tree]) -> usize {
    let n = trees.len();
    (0..n)
        .cartesian_product(0..n)
        .filter(|(x, y)| x != y)
        .map(|(x, y)| (trees[x].clone() + &trees[y]).reduce().magnitude(0))
        .max()
        .unwrap()
}

fn sum_trees(trees: &[Tree]) -> Tree {
    let mut tree = trees[0].clone();
    for t in trees.iter().skip(1) {
        tree = (tree + t).reduce();
    }
    tree
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Tree {
    nodes: Vec<Option<Node>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Num(u8),
    Pair,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

impl Tree {
    fn new() -> Self {
        let nodes: Vec<Option<Node>> = std::iter::repeat(None).take(1 << 7).collect();
        Self { nodes }
    }

    fn from_str(raw: &str) -> Self {
        use Node::*;
        let mut nodes = Self::new().nodes;
        let mut idx = 0;
        for c in raw.chars() {
            match c {
                '[' => {
                    nodes[idx] = Some(Pair);
                    idx = idx * 2 + 1;
                }
                ']' => idx = (idx - 1) / 2,
                ',' => idx += 1,
                d if d.is_digit(10) => {
                    nodes[idx] = Some(Num(d.to_digit(10).unwrap() as u8));
                }
                _ => unreachable!("unknown char: {}", c),
            }
        }
        Self { nodes }
    }

    fn fmt_idx(&self, f: &mut std::fmt::Formatter<'_>, idx: usize) -> std::fmt::Result {
        match self.nodes[idx] {
            Some(Node::Num(n)) => n.fmt(f)?,
            Some(Node::Pair) => {
                f.write_str("[")?;
                self.fmt_idx(f, idx * 2 + 1)?;
                f.write_str(",")?;
                self.fmt_idx(f, idx * 2 + 2)?;
                f.write_str("]")?;
            }
            None => (),
        };
        Ok(())
    }

    fn reduce(mut self) -> Self {
        let mut stable = false;
        while !stable {
            stable = !(self.explode_pair() || self.split_pair());
        }
        self
    }

    fn depth(&self, idx: usize) -> usize {
        // the int_log feature is nightly only for now, it gives .log2()
        // so fallback to something simple
        let mut r = 0;
        let mut current = idx;
        while current > 0 {
            current = (current - 1) / 2;
            r += 1;
        }
        r
    }

    fn explode_pair(&mut self) -> bool {
        let p = self
            .nodes
            .iter()
            .enumerate()
            .find(|(idx, n)| n.is_some() && self.depth(*idx) > 4);
        match p {
            Some((idx, node)) => {
                let node_val = match node {
                    Some(Node::Num(n)) => n,
                    _ => unreachable!(),
                };
                if let Some((left_idx, x)) = self.left_num(idx) {
                    self.nodes[left_idx] = Some(Node::Num(x + node_val));
                }

                let node_val = match self.nodes[idx + 1] {
                    Some(Node::Num(n)) => n,
                    _ => unreachable!(),
                };

                if let Some((right_idx, x)) = self.right_num(idx + 1) {
                    self.nodes[right_idx] = Some(Node::Num(x + node_val));
                }
                self.nodes[idx] = None;
                self.nodes[idx + 1] = None;
                let parent_idx = (idx - 1) / 2;
                self.nodes[parent_idx] = Some(Node::Num(0));

                true
            }
            None => false,
        }
    }

    fn split_pair(&mut self) -> bool {
        let mut idxs = vec![0];
        let mut t = None;
        while let Some(idx) = idxs.pop() {
            match self.nodes[idx].unwrap() {
                Node::Num(k) if k > 9 => {
                    t = Some((idx, k));
                    break;
                }
                Node::Num(_) => (),
                Node::Pair => {
                    idxs.push(idx * 2 + 2);
                    idxs.push(idx * 2 + 1);
                }
            }
        }

        match t {
            Some((idx, x)) => {
                self.nodes[idx] = Some(Node::Pair);
                self.nodes[idx * 2 + 1] = Some(Node::Num(x / 2));
                self.nodes[idx * 2 + 2] = Some(Node::Num(x / 2 + x % 2));
                true
            }
            None => false,
        }
    }

    fn left_num(&self, node_idx: usize) -> Option<(usize, u8)> {
        self.sibling_num(node_idx, Direction::Up, 0)
    }

    fn right_num(&self, node_idx: usize) -> Option<(usize, u8)> {
        self.sibling_num(node_idx, Direction::Up, 1)
    }

    fn sibling_num(&self, node_idx: usize, dir: Direction, offset: usize) -> Option<(usize, u8)> {
        if node_idx == 0 {
            return None;
        }

        match dir {
            Direction::Down => {
                if let Some(Node::Num(n)) = self.nodes[node_idx] {
                    Some((node_idx, n))
                } else {
                    self.sibling_num(node_idx * 2 + 1 + offset, dir, offset)
                }
            }
            Direction::Up => {
                let parent_idx = (node_idx - 1) / 2;
                if node_idx == parent_idx * 2 + 1 + offset {
                    // need to go further up
                    self.sibling_num(parent_idx, dir, offset)
                } else {
                    self.sibling_num(parent_idx * 2 + 1 + offset, Direction::Down, 1 - offset)
                }
            }
        }
    }

    fn magnitude(&self, idx: usize) -> usize {
        match self.nodes[idx].unwrap() {
            Node::Num(k) => k.into(),
            Node::Pair => 3 * self.magnitude(idx * 2 + 1) + 2 * self.magnitude(idx * 2 + 2),
        }
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_idx(f, 0)?;
        Ok(())
    }
}

impl std::ops::Add<&Tree> for Tree {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        // 🤮
        // but apparently that's not the bottleneck 🤷
        let mut tree = Self::new();
        tree.nodes[0] = Some(Node::Pair);

        let mut i = 0;
        for depth in 0..=5 {
            let span = 2usize.pow(depth + 1);
            for j in 0..span / 2 {
                i += 1;
                let source_idx = 2usize.pow(depth) - 1 + j;
                tree.nodes[i] = self.nodes[source_idx];
            }
            for j in 0..span / 2 {
                i += 1;
                let source_idx = 2usize.pow(depth) - 1 + j;
                tree.nodes[i] = rhs.nodes[source_idx];
            }
        }
        tree
    }
}

impl std::ops::Add for Tree {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(&rhs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Tree {
        fn idx_of(&self, target: u8) -> Option<usize> {
            self.nodes
                .iter()
                .enumerate()
                .find(|(_, n)| match n {
                    Some(Node::Num(k)) if *k == target => true,
                    _ => false,
                })
                .map(|(idx, _)| idx)
        }
    }

    #[test]
    fn test_siblings() {
        let t = Tree::from_str("[[1,[2,[3,4]]],[5,[6,[7,[8,9]]]]]");
        assert_eq!(None, t.left_num(t.idx_of(1).unwrap()));
        assert_eq!(
            Some((t.idx_of(1).unwrap(), 1)),
            t.left_num(t.idx_of(2).unwrap())
        );
        assert_eq!(None, t.right_num(t.idx_of(9).unwrap()));
        assert_eq!(
            Some((t.idx_of(4).unwrap(), 4)),
            t.left_num(t.idx_of(5).unwrap())
        );
        assert_eq!(
            Some((t.idx_of(5).unwrap(), 5)),
            t.right_num(t.idx_of(4).unwrap())
        );
    }

    #[test]
    fn test_explode() {
        assert_eq!(
            Tree::from_str("[[[[0,9],2],3],4]"),
            Tree::from_str("[[[[[9,8],1],2],3],4]").reduce()
        );
        assert_eq!(
            Tree::from_str("[7,[6,[5,[7,0]]]]"),
            Tree::from_str("[7,[6,[5,[4,[3,2]]]]]").reduce()
        );

        let t1 = Tree::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        let mut t2 = Tree::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        t2.explode_pair();
        assert_eq!(t1, t2);
        assert_eq!(
            Tree::from_str("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"),
            Tree::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").reduce()
        );
    }

    #[test]
    fn test_add() {
        let t1 = Tree::from_str("[[1,2],[3,4]");
        let t2 = Tree::from_str("[1,2]") + Tree::from_str("[3,4]");
        assert_eq!(t1, t2);

        assert_eq!(
            Tree::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]"),
            Tree::from_str("[1,1]")
                + Tree::from_str("[2,2]")
                + Tree::from_str("[3,3]")
                + Tree::from_str("[4,4]")
        )
    }

    #[test]
    fn test_reduce() {
        assert_eq!(
            Tree::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
            Tree::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").reduce(),
        );
    }

    #[test]
    fn test_reduce_1() {
        let expected = Tree::from_str("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
        let t1 = Tree::from_str("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
        let t2 = Tree::from_str("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
        let actual = (t1 + t2).reduce();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_reduce_2() {
        let expected = Tree::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        let actual =
            (Tree::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]") + Tree::from_str("[1,1]")).reduce();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sum1() {
        let expected = Tree::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        let test_input = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]
";
        let actual = sum_trees(&parse_input(test_input));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sum2() {
        let test_input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
        assert_eq!(
            Tree::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
            sum_trees(&parse_input(test_input))
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(143, Tree::from_str("[[1,2],[[3,4],5]]").magnitude(0));
        assert_eq!(
            3488,
            Tree::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(0)
        );
    }

    const TEST_INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

    #[test]
    fn test_solve1() {
        assert_eq!(4140, solve1(&parse_input(TEST_INPUT)));
    }

    #[test]
    fn test_solve2() {
        assert_eq!(3993, solve2(&parse_input(TEST_INPUT)));
    }
}
