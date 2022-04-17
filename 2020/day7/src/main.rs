use multimap::MultiMap;
use std::fmt;

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

fn main() {
    let rules = parse_rules(include_str!("input.txt"));
    print!("{}", FormattedRules(rules));
}
