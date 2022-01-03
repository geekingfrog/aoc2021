use std::collections::{BTreeMap, BTreeSet};

pub fn solve() -> (usize, usize) {
    let graph = Graph::from_str(include_str!("../resources/day12.txt"));
    (solve1(&graph), solve2(&graph))
}

fn solve1(graph: &Graph) -> usize {
    graph.count_path_to_end(false)
}

fn solve2(graph: &Graph) -> usize {
    graph.count_path_to_end(true)
}

#[derive(Debug, Copy, Clone)]
enum Cave {
    Small,
    Big,
}

#[derive(Debug)]
struct Graph {
    n: usize,
    // adjacency matrix, there are merely 20 or so entries
    // so even a sparse graph doesn't consume much mem
    connections: Vec<bool>,

    // useful for debugging
    #[allow(dead_code)]
    mappings: BTreeMap<String, usize>,

    caves: Vec<Cave>,
}

impl Graph {
    fn from_str(raw: &str) -> Self {
        let pairs: Vec<(&str, &str)> = raw
            .split_terminator('\n')
            .map(|l| {
                let pair: [&str; 2] = l.split('-').collect::<Vec<_>>().try_into().unwrap();
                (pair[0], pair[1])
            })
            .collect();
        let n = raw
            .split_terminator('\n')
            .flat_map(|l| l.split('-'))
            .collect::<BTreeSet<_>>()
            .len();
        let mut connections = vec![false; n * n];
        let mut caves = vec![Cave::Small; n];

        let mut mappings: BTreeMap<&str, usize> =
            vec![("start", 0), ("end", n - 1)].into_iter().collect();

        let mut counter = 0;
        for (orig, end) in pairs {
            let orig_counter: usize = match mappings.get(orig) {
                Some(&c) => c,
                None => {
                    counter += 1;
                    mappings.insert(orig, counter);
                    counter
                }
            };
            if orig.chars().next().unwrap().is_uppercase() {
                caves[orig_counter] = Cave::Big;
            }

            let end_counter: usize = match mappings.get(end) {
                Some(&c) => c,
                None => {
                    counter += 1;
                    mappings.insert(end, counter);
                    counter
                }
            };
            if end.chars().next().unwrap().is_uppercase() {
                caves[end_counter] = Cave::Big;
            }
            connections[orig_counter * n + end_counter] = true;
            connections[end_counter * n + orig_counter] = true;
        }

        let mappings = mappings
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        Self {
            n,
            mappings,
            connections,
            caves,
        }
    }

    fn count_path_to_end(&self, can_return: bool) -> usize {
        let mut seen = vec![false; self.n];
        seen[0] = true;
        let mut paths = RoseTree::new(0);
        self.dfs(0, &mut seen, can_return, &mut paths);
        paths.count_paths(&(self.n - 1))
    }

    fn dfs(
        &self,
        from: usize,
        seen: &mut [bool],
        can_return: bool,
        paths: &mut RoseTree<usize>,
    ) {
        for i in self.connections[(self.n * from)..(self.n * (from + 1))]
            .iter()
            .enumerate()
            .filter_map(|(i, &has_edge)| if has_edge { Some(i) } else { None })
        {
            if seen[i] {
                continue;
            }

            if i == self.n - 1 {
                paths.push(i);
                continue;
            }

            let n = paths.push(i);
            let mut paths = &mut paths.1[n];
            match self.caves[i] {
                Cave::Small => {
                    if can_return {
                        // don't mark node as seen, and flip the switch
                        self.dfs(i, seen, false, &mut paths);
                    }
                    seen[i] = true;
                    self.dfs(i, seen, can_return, &mut paths);
                    seen[i] = false;
                }
                Cave::Big => self.dfs(i, seen, can_return, &mut paths),
            }
        }
    }
}

#[derive(Debug)]
struct RoseTree<T>(T, Vec<RoseTree<T>>);

impl<T> RoseTree<T> {
    fn new(root: T) -> Self {
        RoseTree(root, vec![])
    }

    fn push(&mut self, node: T) -> usize
    where
        T: PartialEq,
    {
        match self.1.iter().enumerate().find(|(_idx, t)| t.0 == node) {
            Some((i, _)) => i,
            None => {
                self.1.push(Self::new(node));
                self.1.len() - 1
            }
        }
    }

    fn count_paths(&self, target: &T) -> usize
    where
        T: Eq,
    {
        if &self.0 == target {
            1
        } else {
            self.1.iter().map(|n| n.count_paths(target)).sum()
        }
    }

    // fn dedup(&mut self)
    //     where T: Eq + std::hash::Hash + Copy
    // {
    //     let mut seen = HashMap::new();
    //     for c in &self.1 {
    //         let e = seen.entry(&c.0).or_insert_with(Vec::new);
    //         e.extend(&c.1);
    //     }
    //     let children = seen.into_iter().map(|x| x.1).collect();
    //     *self = RoseTree(self.0, children);
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    const TEST_INPUT2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

    const TEST_INPUT3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

    #[test]
    fn test_solve1() {
        assert_eq!(10, solve1(&Graph::from_str(TEST_INPUT1)));
        assert_eq!(19, solve1(&Graph::from_str(TEST_INPUT2)));
        assert_eq!(226, solve1(&Graph::from_str(TEST_INPUT3)));
    }

    #[test]
    fn test_solve2() {
        assert_eq!(36, solve2(&Graph::from_str(TEST_INPUT1)));
        assert_eq!(103, solve2(&Graph::from_str(TEST_INPUT2)));
        assert_eq!(3509, solve2(&Graph::from_str(TEST_INPUT3)));
    }
}
