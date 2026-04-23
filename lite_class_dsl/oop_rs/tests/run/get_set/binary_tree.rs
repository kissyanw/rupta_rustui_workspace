use oop_rs::class;

#[class]
type Node = class<
    {
        let mut value: usize = 0;
        let mut left: Option<CRc<Node>>;
        let mut right: Option<CRc<Node>>;
        let mut parent: Option<CWeak<Node>>;

        pub fn new(value: usize) -> Self {
            Self { value }
        }
    },
>;

#[class]
type Tree = class<
    {
        let mut root: Option<CRc<Node>>;

        pub fn new() -> Self {
            Self {}
        }

        pub fn insert(&self, value: usize) {
            let node = Node::new(value);
            if let Some(root) = self.get().root() {
                self.insert_node(root, node);
            } else {
                self.set().root(Some(node));
            }
        }

        pub fn insert_node(&self, current: CRc<Node>, new: CRc<Node>) {
            if new.get().value() < current.get().value() {
                if let Some(left) = current.get().left() {
                    self.insert_node(left, new);
                } else {
                    current.set().left(Some(new.clone()));
                    new.set().parent(Some(current));
                }
            } else {
                if let Some(right) = current.get().right() {
                    self.insert_node(right, new);
                } else {
                    current.set().right(Some(new.clone()));
                    new.set().parent(Some(current));
                }
            }
        }

        pub fn pre_order_traverse(&self) -> Vec<usize> {
            let mut result = Vec::new();
            if let Some(root) = self.get().root() {
                self.pre_order_traverse_node(root, &mut result);
            }
            result
        }

        fn pre_order_traverse_node(&self, node: CRc<Node>, result: &mut Vec<usize>) {
            result.push(node.get().value());
            if let Some(left) = node.get().left() {
                self.pre_order_traverse_node(left, result);
            }
            if let Some(right) = node.get().right() {
                self.pre_order_traverse_node(right, result);
            }
        }

        pub fn in_order_traverse(&self) -> Vec<usize> {
            let mut result = Vec::new();
            if let Some(root) = self.get().root() {
                self.in_order_traverse_node(root, &mut result);
            }
            result
        }

        fn in_order_traverse_node(&self, node: CRc<Node>, result: &mut Vec<usize>) {
            if let Some(left) = node.get().left() {
                self.in_order_traverse_node(left, result);
            }
            result.push(node.get().value());
            if let Some(right) = node.get().right() {
                self.in_order_traverse_node(right, result);
            }
        }

        pub fn post_order_traverse(&self) -> Vec<usize> {
            let mut result = Vec::new();
            if let Some(root) = self.get().root() {
                self.post_order_traverse_node(root, &mut result);
            }
            result
        }

        fn post_order_traverse_node(&self, node: CRc<Node>, result: &mut Vec<usize>) {
            if let Some(left) = node.get().left() {
                self.post_order_traverse_node(left, result);
            }
            if let Some(right) = node.get().right() {
                self.post_order_traverse_node(right, result);
            }
            result.push(node.get().value());
        }
    },
>;

#[test]
fn binary_tree() {
    let tree = Tree::new();
    let values = [5, 2, 8, 4, 7, 1, 3, 6, 9, 10];
    for value in values {
        tree.insert(value);
    }

    let pre_order = tree.pre_order_traverse();
    assert_eq!(pre_order, [5, 2, 1, 4, 3, 8, 7, 6, 9, 10]);

    let in_order = tree.in_order_traverse();
    assert_eq!(in_order, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    let post_order = tree.post_order_traverse();
    assert_eq!(post_order, [1, 3, 4, 2, 6, 7, 10, 9, 8, 5]);
}
