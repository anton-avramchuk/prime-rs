//! Чистая логика TreeSelect: предки выбранных узлов и подписи выбранного.

use prime_tree::{Flat, TreeNode};
use std::collections::HashSet;

/// Ключи предков всех узлов с ключами из `keys` (их нужно раскрыть, чтобы выбранные узлы были видны).
pub fn ancestors_of(options: &[TreeNode], keys: &[String]) -> HashSet<String> {
    fn walk(nodes: &[TreeNode], keys: &[String], path: &mut Vec<String>, out: &mut HashSet<String>) {
        for n in nodes {
            if keys.contains(&n.key) {
                out.extend(path.iter().cloned());
            }
            path.push(n.key.clone());
            walk(&n.children, keys, path, out);
            path.pop();
        }
    }
    let mut out = HashSet::new();
    walk(options, keys, &mut Vec::new(), &mut out);
    out
}

/// Пары `(ключ, подпись)` выбранных узлов в порядке выбора; ключи, которых нет в дереве, пропускаются.
pub fn labels_of(flat: &[Flat], keys: &[String]) -> Vec<(String, String)> {
    keys.iter().filter_map(|k| flat.iter().find(|f| &f.key == k).map(|f| (f.key.clone(), f.label.clone()))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use prime_tree::flatten;

    fn n(key: &str, label: &str) -> TreeNode {
        TreeNode::new(key).label(label)
    }

    fn tree() -> Vec<TreeNode> {
        vec![n("a", "Alpha").children(vec![n("a1", "Apple").children(vec![n("a11", "Red")]), n("a2", "Avocado")]), n("b", "Beta")]
    }

    fn keys(k: &[&str]) -> Vec<String> {
        k.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn ancestors() {
        let set = |k: &[&str]| k.iter().map(|s| (*s).to_owned()).collect::<HashSet<_>>();
        assert_eq!(ancestors_of(&tree(), &keys(&["a11"])), set(&["a", "a1"]));
        assert_eq!(ancestors_of(&tree(), &keys(&["a2", "b"])), set(&["a"]));
        assert_eq!(ancestors_of(&tree(), &keys(&["a"])), set(&[]), "у корневого узла предков нет");
        assert!(ancestors_of(&tree(), &keys(&["zzz"])).is_empty());
        assert!(ancestors_of(&tree(), &[]).is_empty());
    }

    #[test]
    fn labels_follow_selection_order() {
        let flat = flatten(&tree());
        let l = labels_of(&flat, &keys(&["b", "a11", "ghost", "a"]));
        assert_eq!(l, [("b".into(), "Beta".into()), ("a11".into(), "Red".into()), ("a".into(), "Alpha".into())]);
        assert!(labels_of(&flat, &[]).is_empty());
    }
}
