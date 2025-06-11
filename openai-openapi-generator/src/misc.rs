use std::collections::HashSet;
use std::hash;

pub(crate) fn is_unique<I>(iter: I) -> bool
where
    I: IntoIterator,
    I::Item: Eq + hash::Hash,
{
    let mut items = HashSet::new();
    iter.into_iter().all(|item| items.insert(item))
}
