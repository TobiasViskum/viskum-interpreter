pub struct CFGHelper<'a> {
    pub get_next_node_id: &'a Box<dyn Fn() -> usize>,
    pub get_current_node_id: &'a Box<dyn Fn() -> usize>,
    pub scope: usize,
}

impl<'a> CFGHelper<'a> {
    pub fn new(
        get_next_node_id: &'a Box<dyn Fn() -> usize>,
        get_current_node_id: &'a Box<dyn Fn() -> usize>,
        scope: usize
    ) -> Self {
        Self {
            get_next_node_id,
            get_current_node_id,
            scope,
        }
    }

    pub fn increase_scope(&mut self) {
        self.scope += 1;
    }

    pub fn decrease_scope(&mut self) {
        self.scope -= 1;
    }
}

#[derive(Debug)]
pub struct GotoNodeIds {
    pub break_node_ids: Vec<usize>,
    pub continue_node_ids: Vec<usize>,
}

impl GotoNodeIds {
    pub fn new() -> Self {
        Self {
            break_node_ids: vec![],
            continue_node_ids: vec![],
        }
    }

    pub fn extend(&mut self, other: GotoNodeIds) {
        self.break_node_ids.extend(other.break_node_ids);
        self.continue_node_ids.extend(other.continue_node_ids);
    }
}
