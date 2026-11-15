//! The flow from one Native thing as a picture: Mermaid text, and an HTML page that draws it in a
//! browser.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::model::{self, Model, Target};

/// Escapes the characters Mermaid would read as markup inside a `["..."]` node label.
fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '"' => out.push_str("#quot;"),
            '<' => out.push_str("#lt;"),
            '>' => out.push_str("#gt;"),
            _ => out.push(c),
        }
    }
    out
}

/// Escapes `text` for HTML text (the page title).
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// What one walk found: node texts in id order, edges as (from id, to id, first op).
struct Graph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize, Option<String>)>,
    /// A new node had to be dropped because `max_nodes` was reached.
    truncated: bool,
}

/// Breadth-first walk from `root` over entry-to-entry mentions, up to `max_depth` deep and at most
/// `max_nodes` nodes.
fn walk(model: &Model, root: &Target, max_depth: usize, max_nodes: usize) -> Graph {
    let mut graph = Graph { nodes: Vec::new(), edges: Vec::new(), truncated: false };
    if max_nodes == 0 {
        graph.truncated = model.outgoing(root).iter().any(|(to, _)| matches!(to, Target::Entry { .. }));
        return graph;
    }
    let mut seen: HashMap<Target, usize> = HashMap::new();
    let mut edge_seen: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(Target, usize)> = VecDeque::new();
    seen.insert(root.clone(), 0);
    graph.nodes.push(node_text(model, root));
    queue.push_back((root.clone(), 0));
    while let Some((node, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }
        let from = seen[&node];
        for (to, op) in model.outgoing(&node) {
            if !matches!(to, Target::Entry { .. }) {
                continue;
            }
            let index = match seen.get(to) {
                Some(&index) => index,
                None => {
                    if graph.nodes.len() >= max_nodes {
                        graph.truncated = true;
                        continue;
                    }
                    let index = graph.nodes.len();
                    seen.insert(to.clone(), index);
                    graph.nodes.push(node_text(model, to));
                    queue.push_back((to.clone(), depth + 1));
                    index
                }
            };
            if edge_seen.insert((from, index)) {
                graph.edges.push((from, index, op.clone()));
            }
        }
    }
    graph
}

/// Label of `target`, and its in-game hint on a second line when it has one.
fn node_text(model: &Model, target: &Target) -> String {
    let mut text = escape(&model.label(target));
    if let Target::Entry { table, index } = target {
        let entry = &model.atlas.tables[table].entries[*index];
        if let Some(hint) = model::entry_hint(table, entry) {
            text.push_str("<br/>");
            text.push_str(&escape(&model::short(&hint, 40)));
        }
    }
    text
}

/// The flow from `root` as Mermaid `flowchart TD` text.
#[allow(dead_code)]
pub fn mermaid(model: &Model, root: &Target, max_depth: usize, max_nodes: usize) -> String {
    let graph = walk(model, root, max_depth, max_nodes);
    let mut out = String::from("flowchart TD\n");
    for (index, text) in graph.nodes.iter().enumerate() {
        out.push_str(&format!("    n{index}[\"{text}\"]\n"));
    }
    for (from, to, op) in &graph.edges {
        match op {
            Some(op) => out.push_str(&format!("    n{from} -->|{op}| n{to}\n")),
            None => out.push_str(&format!("    n{from} --> n{to}\n")),
        }
    }
    if graph.truncated {
        out.push_str("%% truncated\n");
    }
    out
}

/// A standalone page that draws the flow with Mermaid loaded from its CDN.
#[allow(dead_code)]
pub fn html(model: &Model, root: &Target, max_depth: usize, max_nodes: usize) -> String {
    let title = html_escape(&model.label(root));
    let chart = mermaid(model, root, max_depth, max_nodes);
    format!(
        "<!doctype html>\n\
         <html lang=\"en\">\n\
         <head>\n\
         <meta charset=\"utf-8\">\n\
         <title>{title}</title>\n\
         </head>\n\
         <body>\n\
         <pre class=\"mermaid\">\n\
         {chart}</pre>\n\
         <script src=\"https://cdn.jsdelivr.net/npm/mermaid@10.9.1/dist/mermaid.min.js\"></script>\n\
         <script>mermaid.initialize({{startOnLoad:true}});</script>\n\
         </body>\n\
         </html>\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model() -> Model {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_refs.json");
        Model::new(uw_atlas::Atlas::from_path(path).expect("fixture"))
    }

    fn root() -> Target {
        Target::Entry { table: "scripts".into(), index: 0 }
    }

    #[test]
    fn walk_follows_entry_mentions() {
        let model = model();
        let chart = mermaid(&model, &root(), 2, 10);
        assert!(chart.starts_with("flowchart TD"), "{chart}");
        assert_eq!(chart.lines().filter(|line| line.contains('[')).count(), 4, "{chart}");
        assert_eq!(chart.lines().filter(|line| line.contains("-->")).count(), 4, "{chart}");
        assert!(chart.contains(r#"n0["script_game_start"]"#), "{chart}");
        assert!(chart.contains(r#"n1["menu_camp#1<br/>A"]"#), "{chart}");
        assert!(chart.contains("n0 -->|jump_to_menu| n1"), "{chart}");
        assert!(chart.contains("n3 -->|assign| n2"), "{chart}");
        assert!(!chart.contains("%% truncated"), "{chart}");
    }

    #[test]
    fn node_hint_is_escaped() {
        let text = r#"{"schema":1,"kit_hash":null,"ops":[],"hooks":[],"ranges":[],
            "tables":{"menus":{"module":"module_game_menus","var":"game_menus",
            "id_file":"ID_menus","id_prefix":"menu_","ref_tag":"mnu","entries":[
            {"index":0,"name":"x","id":"menu_x","dup":null,"line":1,
             "value":["x",0,"say \"hi\"",{"list":[]}]}]}}}"#;
        let model = Model::new(uw_atlas::Atlas::from_str(text).expect("inline atlas"));
        let chart = mermaid(&model, &Target::Entry { table: "menus".into(), index: 0 }, 1, 10);
        assert!(chart.contains(r#"n0["menu_x<br/>say #quot;hi#quot;"]"#), "{chart}");
    }

    #[test]
    fn max_nodes_cuts_the_walk() {
        let model = model();
        let chart = mermaid(&model, &root(), 2, 1);
        assert_eq!(chart.lines().filter(|line| line.contains('[')).count(), 1, "{chart}");
        assert!(chart.ends_with("%% truncated\n"), "{chart}");
    }

    #[test]
    fn html_page_holds_chart_and_cdn() {
        let model = model();
        let page = html(&model, &root(), 2, 10);
        assert!(page.starts_with("<!doctype html>"), "{page}");
        assert!(page.contains("<meta charset=\"utf-8\">"), "{page}");
        assert!(page.contains("<title>script_game_start</title>"), "{page}");
        assert!(page.contains(&mermaid(&model, &root(), 2, 10)), "{page}");
        assert!(page.contains("https://cdn.jsdelivr.net/npm/mermaid@10.9.1/dist/mermaid.min.js"), "{page}");
        assert!(page.contains("mermaid.initialize({startOnLoad:true})"), "{page}");
    }
}
