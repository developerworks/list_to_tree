use std::collections::HashMap;

pub trait IntoTree: Sized {
    type Output;

    fn get_id(&self) -> i32;
    fn get_parent_id(&self) -> i32;
    fn convert(&self, children: Vec<Self::Output>) -> Self::Output;
}
#[allow(unused)]
fn take_all_children<T>(parent_id: i32, children: &mut HashMap<i32, Vec<&T>>) -> Vec<T::Output>
where
    T: IntoTree,
{
    children
        .remove(&parent_id)
        .unwrap_or_default()
        .iter()
        .map(|child| {
            let grandchildren = take_all_children(child.get_id(), children);
            child.convert(grandchildren)
        })
        .collect()
}

#[allow(unused)]
pub fn convert_to_tree<T>(top_nodes: &[T], children: &[T]) -> Vec<T::Output>
where
    T: IntoTree,
{
    let mut children_by_parent = HashMap::new();
    for sub in children {
        children_by_parent
            .entry(sub.get_parent_id())
            .or_insert_with(Vec::new)
            .push(sub);
    }

    top_nodes
        .iter()
        .map(|top_node| {
            let children = take_all_children(top_node.get_id(), &mut children_by_parent);
            top_node.convert(children)
        })
        .collect()
}
