
type IteratorSource<Item, IntoIter> = Box<dyn IntoIterator<Item = Item, IntoIter = IntoIter>>;
pub struct IteratorBuilder<Item, IntoIter> {
    container: IteratorSource<Item, IntoIter>,
    make_iter: dyn Fn(&IteratorSource<Item, IntoIter>),
}
