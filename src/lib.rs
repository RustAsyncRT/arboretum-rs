mod behaviour_tree;
mod stack;

pub use behaviour_tree::{BehaviourTree, Status, Action};

#[cfg(test)]
mod tests {
    use crate::behaviour_tree::{BehaviourTree, LayoutNode, Status};

    fn create_layout() -> [LayoutNode<[Status; 16]>; 28] {
        [
            LayoutNode::new_fallback(1),
            LayoutNode::new_sequence(2).with_sibling(22),
            LayoutNode::new_fallback(3).with_sibling(15),
            LayoutNode::new_fallback(4).with_sibling(6),
            LayoutNode::new_leaf(|user_data: Option<&mut [Status; 16]>| { user_data.unwrap()[0] }).with_sibling(5),
            LayoutNode::new_leaf(|user_data| { user_data.unwrap()[1] }),
            LayoutNode::new_leaf(|user_data: Option<&mut [Status; 16]>| { user_data.unwrap()[2] }).with_sibling(7),
            LayoutNode::new_fallback(8),
            LayoutNode::new_sequence(9).with_sibling(13),
            LayoutNode::new_sequence(10).with_sibling(12),
            LayoutNode::new_leaf(|user_data: Option<&mut [Status; 16]>| { user_data.unwrap()[3] }).with_sibling(11),
            LayoutNode::new_leaf(|user_data| { user_data.unwrap()[4] }),
            LayoutNode::new_leaf(|user_data| { user_data.unwrap()[5] }),
            LayoutNode::new_leaf(|user_data: Option<&mut [Status; 16]>| { user_data.unwrap()[6] }).with_sibling(14),
            LayoutNode::new_leaf(|user_data| { user_data.unwrap()[7] }),
            LayoutNode::new_fallback(16),
            LayoutNode::new_sequence(17).with_sibling(21),
            LayoutNode::new_sequence(18).with_sibling(20),
            LayoutNode::new_leaf(|user_data: Option<&mut [Status; 16]>| { user_data.unwrap()[8] }).with_sibling(19),
            LayoutNode::new_leaf(|user_data| { user_data.unwrap()[9] }),
            LayoutNode::new_leaf(|user_data| { user_data.unwrap()[10] }),
            LayoutNode::new_leaf(|user_data: Option<&mut [Status; 16]>| { user_data.unwrap()[11] }),
            LayoutNode::new_leaf(|user_data: Option<&mut [Status; 16]>| { user_data.unwrap()[12] }).with_sibling(23),
            LayoutNode::new_sequence(24),
            LayoutNode::new_fallback(25).with_sibling(27),
            LayoutNode::new_leaf(|user_data: Option<&mut [Status; 16]>| { user_data.unwrap()[13] }).with_sibling(26),
            LayoutNode::new_leaf(|user_data| { user_data.unwrap()[14] }),
            LayoutNode::new_leaf(|user_data| { user_data.unwrap()[15] }),
        ]
    }

    #[test]
    fn all_success() {
        let test_leaves_status = [Status::Success; 16];

        let layout = create_layout();

        let mut bht = BehaviourTree::<[Status; 16], 28, 7>::new(layout)
            .with_user_data(test_leaves_status);
        let res = bht.execute();

        assert_eq!(res, Ok(Status::Success))
    }

    #[test]
    fn all_failure() {
        let test_leaves_status = [Status::Failure; 16];

        let layout = create_layout();

        let mut bht = BehaviourTree::<[Status; 16], 28, 7>::new(layout)
            .with_user_data(test_leaves_status);
        let res = bht.execute();

        assert_eq!(res, Ok(Status::Failure))
    }

    #[test]
    fn success_1() {
        let mut test_leaves_status = [Status::Failure; 16];
        test_leaves_status[12] = Status::Success;

        let layout = create_layout();

        let mut bht = BehaviourTree::<[Status; 16], 28, 7>::new(layout)
            .with_user_data(test_leaves_status);
        let res = bht.execute();

        assert_eq!(res, Ok(Status::Success))
    }

    #[test]
    fn success_2() {
        let mut test_leaves_status = [Status::Success; 16];
        for i in 0..4 {
            test_leaves_status[i + 12] = Status::Failure;
        }

        let layout = create_layout();

        let mut bht = BehaviourTree::<[Status; 16], 28, 7>::new(layout)
            .with_user_data(test_leaves_status);
        let res = bht.execute();

        assert_eq!(res, Ok(Status::Success))
    }

    #[test]
    fn success_3() {
        let test_leaves_status = [
            Status::Failure, Status::Failure, Status::Failure, Status::Failure, Status::Failure, Status::Failure,
            Status::Failure, Status::Failure, Status::Success, Status::Success, Status::Success, Status::Success,
            Status::Failure, Status::Success, Status::Failure, Status::Success,
        ];

        let layout = create_layout();

        let mut bht = BehaviourTree::<[Status; 16], 28, 7>::new(layout)
            .with_user_data(test_leaves_status);
        let res = bht.execute();

        assert_eq!(res, Ok(Status::Success))
    }

    #[test]
    fn failure_1() {
        let test_leaves_status = [
            Status::Success, Status::Success, Status::Success, Status::Success, Status::Success, Status::Success,
            Status::Success, Status::Success, Status::Failure, Status::Failure, Status::Failure, Status::Failure,
            Status::Failure, Status::Success, Status::Success, Status::Failure,
        ];

        let layout = create_layout();

        let mut bht = BehaviourTree::<[Status; 16], 28, 7>::new(layout)
            .with_user_data(test_leaves_status);
        let res = bht.execute();

        assert_eq!(res, Ok(Status::Failure))
    }

    #[test]
    fn failure_2() {
        let test_leaves_status = [
            Status::Success, Status::Success, Status::Success, Status::Success, Status::Success, Status::Success,
            Status::Success, Status::Success, Status::Failure, Status::Success, Status::Success, Status::Failure,
            Status::Failure, Status::Success, Status::Success, Status::Failure,
        ];

        let layout = create_layout();

        let mut bht = BehaviourTree::<[Status; 16], 28, 7>::new(layout)
            .with_user_data(test_leaves_status);
        let res = bht.execute();

        assert_eq!(res, Ok(Status::Failure))
    }

    #[test]
    fn failure_3() {
        let test_leaves_status = [
            Status::Failure, Status::Success, Status::Failure, Status::Success, Status::Success, Status::Success,
            Status::Success, Status::Success, Status::Failure, Status::Success, Status::Success, Status::Failure,
            Status::Failure, Status::Success, Status::Success, Status::Failure,
        ];

        let layout = create_layout();

        let mut bht = BehaviourTree::<[Status; 16], 28, 7>::new(layout)
            .with_user_data(test_leaves_status);
        let res = bht.execute();

        assert_eq!(res, Ok(Status::Failure))
    }
}
