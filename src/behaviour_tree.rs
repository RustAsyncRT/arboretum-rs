use crate::stack::Stack;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Status {
    Failure,
    Success,
}

pub struct BehaviourTree<'a, U, const N: usize, const H: usize> {
    layout_nodes: [LayoutNode<U>; N],
    runtime_datas: [RuntimeData; N],
    user_data: Option<U>,
    stack: Stack<usize, H>,
    trace: Option<&'a mut [Option<Status>]>,
}

impl<'a, U, const N: usize, const H: usize> BehaviourTree<'a, U, N, H> {
    pub fn new(layout_nodes: [LayoutNode<U>; N]) -> Self {
        Self {
            layout_nodes,
            runtime_datas: [RuntimeData::default(); N],
            user_data: None,
            stack: Stack::default(),
            trace: None,
        }
    }

    pub fn with_user_data(mut self, user_data: U) -> Self {
        self.user_data = Some(user_data);
        self
    }

    #[allow(dead_code)]
    pub fn with_trace(mut self, trace: &'a mut [Option<Status>; N]) -> Self {
        self.trace = Some(trace);
        self
    }

    pub fn execute(&mut self) -> Result<Status, ()> {
        self.runtime_datas = [RuntimeData::default(); N];

        self.stack.push(0)?;

        let mut res = Status::Success;

        while !self.stack.is_empty() {
            let current_index = *self.stack.peek()?;
            let current_layout = &self.layout_nodes[current_index];
            let current_rt = &mut self.runtime_datas[current_index];

            match current_layout {
                LayoutNode::Leaf { action, .. } => {
                    self.stack.pop()?;
                    res = action(self.user_data.as_mut());
                    if let Some(trace) = &mut self.trace {
                        trace[current_index] = Some(res);
                    }
                }
                LayoutNode::Sequence { first_child, .. } => {
                    if !current_rt.started {
                        current_rt.started = true;
                        current_rt.current_child = Some(*first_child);
                        res = Status::Success;
                    }

                    match res {
                        Status::Success => {
                            match current_rt.current_child {
                                None => {
                                    self.stack.pop()?;
                                    if let Some(trace) = &mut self.trace {
                                        trace[current_index] = Some(res);
                                    }
                                }
                                Some(current_child) => {
                                    self.stack.push(current_child)?;
                                    current_rt.current_child = self.layout_nodes[current_child].sibling();
                                }
                            }
                        }
                        Status::Failure => {
                            self.stack.pop()?;
                            res = Status::Failure;
                            if let Some(trace) = &mut self.trace {
                                trace[current_index] = Some(res);
                            }
                        }
                    }
                }
                LayoutNode::Fallback { first_child, .. } => {
                    if !current_rt.started {
                        current_rt.started = true;
                        current_rt.current_child = Some(*first_child);
                        res = Status::Failure;
                    }

                    match res {
                        Status::Failure => {
                            match current_rt.current_child {
                                None => {
                                    self.stack.pop()?;
                                    if let Some(trace) = &mut self.trace {
                                        trace[current_index] = Some(res);
                                    }
                                }
                                Some(current_child) => {
                                    self.stack.push(current_child)?;
                                    current_rt.current_child = self.layout_nodes[current_child].sibling();
                                }
                            }
                        }
                        Status::Success => {
                            self.stack.pop()?;
                            res = Status::Success;
                            if let Some(trace) = &mut self.trace {
                                trace[current_index] = Some(res);
                            }
                        }
                    }
                }
            }
        }

        Ok(res)
    }
}

pub type Action<U> = fn(user_data: Option<&mut U>) -> Status;

pub enum LayoutNode<U> {
    Sequence {
        sibling: Option<usize>,
        first_child: usize,
    },
    Fallback {
        sibling: Option<usize>,
        first_child: usize,
    },
    Leaf {
        sibling: Option<usize>,
        action: Action<U>,
    },
}

impl<U> LayoutNode<U> {
    pub fn new_sequence(first_child: usize) -> Self {
        LayoutNode::Sequence {
            sibling: None,
            first_child,
        }
    }

    pub fn new_fallback(first_child: usize) -> Self {
        LayoutNode::Fallback {
            sibling: None,
            first_child,
        }
    }

    pub fn new_leaf(action: Action<U>) -> Self {
        LayoutNode::Leaf {
            sibling: None,
            action,
        }
    }

    pub fn with_sibling(mut self, sibling: usize) -> Self {
        let value = sibling;
        match &mut self {
            LayoutNode::Sequence { sibling, .. } => *sibling = Some(value),
            LayoutNode::Fallback { sibling, .. } => *sibling = Some(value),
            LayoutNode::Leaf { sibling, .. } => *sibling = Some(value),
        }
        self
    }

    pub fn sibling(&self) -> Option<usize> {
        match &self {
            LayoutNode::Sequence { sibling, .. } => sibling.clone(),
            LayoutNode::Fallback { sibling, .. } => sibling.clone(),
            LayoutNode::Leaf { sibling, .. } => sibling.clone(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct RuntimeData {
    started: bool,
    current_child: Option<usize>,
}

impl Default for RuntimeData {
    fn default() -> Self {
        Self {
            started: false,
            current_child: None,
        }
    }
}
