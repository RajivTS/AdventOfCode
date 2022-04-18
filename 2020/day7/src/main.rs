use multimap::MultiMap;
use std::fmt;
use itertools::Itertools;

type BagSpec<'a> = (&'a str, &'a str);
type Rules<'a> = MultiMap<BagSpec<'a>, (usize, BagSpec<'a>)>;

fn parse_rules(input: &str) -> Rules<'_> {
    let mut rules: Rules = Default::default();
    peg::parser! {
        pub(crate) grammar parser() for str {
            pub(crate) rule root(r: &mut Rules<'input>)
            = (line(r) "." whitespace()*)* ![_]

            rule line(r: &mut Rules<'input>)
            = spec:bag_spec() " contain " rules:rules() {
                if let Some(rules) = rules {
                    for rule in rules {
                        r.insert(spec, rule)
                    }
                }
            }

            rule bag_spec() -> BagSpec<'input>
            = adjective:name() " " color:name() " bag" "s"? { (adjective, color) }

            rule rules() -> Option<Vec<(usize, BagSpec<'input>)>>
            = rules:rule1()+ { Some(rules) }
            / "no other bags" { None }

            rule rule1() -> (usize, BagSpec<'input>)
            = r:rule0() ", "? { r }

            rule rule0() -> (usize, BagSpec<'input>)
            = count:num() " " spec:bag_spec() { (count,spec) }

            rule num() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule name() -> &'input str
            = s:$((!whitespace()[_])*)

            rule whitespace()
            = [ '\t' | '\r' | '\n' | ' ' ]
        }
    }
    parser::root(input, &mut rules).unwrap();
    rules
}

struct FormattedRules<'a>(Rules<'a>);

impl fmt::Display for FormattedRules<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, vv) in &self.0 {
            write!(f, "{} {} bags can contain ", k.0, k.1)?;
            if vv.is_empty() {
                write!(f, "no other bags")?;
            } else {
                for (i, v) in vv.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(
                        f,
                        "{} {} {} {}",
                        v.0,
                        v.1 .0,
                        v.1 .1,
                        if v.0 == 1 { "bag" } else { "bags" }
                    )?;
                }
            }
            writeln!(f, ".")?;
        }
        Ok(())
    }
}

fn reverse_graph<'a>(graph: &Rules<'a>) -> Rules<'a> {
    graph
        .iter_all()
        .map(|(&node, neighbors)| {
            neighbors
                .iter()
                .map(move |&(count, neighbor)| (neighbor, (count, node)))
        })
        .flatten()
        .collect()
}

fn walk_subgraph<'iter, 'elems: 'iter>(
    graph: &'iter Rules<'elems>,
    root: &(&'iter str, &'iter str),
) -> Box<dyn Iterator<Item = (&'elems str, &'elems str)> + 'iter> {
    Box::new(
        graph
        .get_vec(root)
        .into_iter()
        .flatten()
        .map(move |&(_,neighbor)| {
            std::iter::once(neighbor).chain(walk_subgraph(graph, &neighbor))
        })
        .flatten()
    )
}

fn bag_quantities<'iter, 'elems: 'iter>(
    graph: &'iter Rules<'elems>,
    root: &(&'iter str,&'iter str),
) -> Box<dyn Iterator<Item = usize> + 'iter> {
    Box::new(
        graph
        .get_vec(root)
        .into_iter()
        .flatten()
        .map(move |&(count, neighbor)| {
            std::iter::once(count).chain(bag_quantities(graph, &neighbor).map(move |x| x * count))
        })
        .flatten()
    )
}

fn main() {
    // Part 1
    let rules = parse_rules(include_str!("input.txt"));
    let rev_rules = reverse_graph(&rules);

    let target_bag = ("shiny", "gold");
    let answer = walk_subgraph(&rev_rules, &target_bag).unique().count();
    println!("{} colors can contain {:?} bags", answer, target_bag);

    // Part 2
    let answer: usize = bag_quantities(&rules, &target_bag).sum();
    println!("You must buy {} bags to fill a {:?} bag", answer, target_bag);
}
