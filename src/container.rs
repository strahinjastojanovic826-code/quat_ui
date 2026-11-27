use crate::state::UiState;
use crate::widget::Widget;
use crate::Quat;

/// A container widget capable of holding child widgets (Widget Tree).
pub struct Container {
    pub id: String,
    pub state: UiState,
    pub children: Vec<Box<dyn Widget>>,
    pub scroll_y: f32,
    pub max_scroll_y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeMeta {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub first_child: Option<NodeId>,
    pub next_sibling: Option<NodeId>,
}

pub struct NodeArena {
    pub nodes: Vec<Box<dyn Widget>>,
    pub meta: Vec<NodeMeta>,
    pub states: crate::tree_buffer::QuatTreeBuffer,
}

impl Container {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            state: UiState::default(),
            children: Vec::new(),
            scroll_y: 0.0,
            max_scroll_y: 0.0,
        }
    }

    /// Adds a child widget to this container.
    pub fn add_child(&mut self, widget: impl Widget + 'static) {
        self.children.push(Box::new(widget));
    }

    /// Returns the number of children in this container.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Propagates `event` across children using the quaternary state machine.
    /// Returns `true` if at least one child transitioned to a new state.
    pub fn dispatch_event(&mut self, event: &crate::event::UiEvent) -> bool {
        let mut any_changed = false;

        for child in &mut self.children {
            let prev_state = child.state();
            let next_quat = event.next_state(prev_state.quat);

            if next_quat != prev_state.quat
                && crate::event::UiEvent::is_valid_transition(prev_state.quat, next_quat)
            {
                let mut next_state = prev_state;
                next_state.quat = next_quat;
                child.set_state(next_state);
                any_changed = true;
            }
        }

        any_changed
    }
}

impl NodeArena {
    pub fn new(initial_capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(initial_capacity),
            meta: Vec::with_capacity(initial_capacity),
            states: crate::tree_buffer::QuatTreeBuffer::new(initial_capacity.max(1)),
        }
    }

    fn ensure_state_capacity(&mut self, index: usize) {
        if index >= self.states.capacity_nodes {
            let new_cap = (index + 1).next_power_of_two();
            let mut next_buf = crate::tree_buffer::QuatTreeBuffer::new(new_cap);
            for i in 0..self.states.capacity_nodes {
                if let Some(q) = self.states.get_node_state(i) {
                    let _ = next_buf.set_node_state(i, q);
                }
            }
            self.states = next_buf;
        }
    }

    /// Adds a root or child node and links it into the parent/sibling chain.
    pub fn add_node<W: Widget + 'static>(
        &mut self,
        parent: Option<NodeId>,
        widget: W,
    ) -> Result<NodeId, &'static str> {
        let idx = self.nodes.len();
        let id = NodeId(idx);
        let initial_quat = widget.state().quat;

        self.ensure_state_capacity(idx);
        self.states.set_node_state(idx, initial_quat)?;

        self.nodes.push(Box::new(widget));
        self.meta.push(NodeMeta {
            id,
            parent,
            first_child: None,
            next_sibling: None,
        });

        if let Some(p_id) = parent {
            if p_id.0 >= self.meta.len() {
                return Err("Parent NodeId out of bounds");
            }
            match self.meta[p_id.0].first_child {
                None => self.meta[p_id.0].first_child = Some(id),
                Some(mut curr) => {
                    while let Some(next) = self.meta[curr.0].next_sibling {
                        curr = next;
                    }
                    self.meta[curr.0].next_sibling = Some(id);
                }
            }
        }

        Ok(id)
    }

    pub fn set_node_quat(&mut self, id: NodeId, quat: Quat) -> Result<(), &'static str> {
        if id.0 >= self.nodes.len() {
            return Err("NodeId out of bounds");
        }
        self.states.set_node_state(id.0, quat)?;
        let mut st = self.nodes[id.0].state();
        st.quat = quat;
        self.nodes[id.0].set_state(st);
        Ok(())
    }

    pub fn get_node_quat(&self, id: NodeId) -> Option<Quat> {
        self.states.get_node_state(id.0)
    }

    /// Iterates over direct children of `parent` using the sibling chain.
    pub fn children_of(&self, parent: NodeId) -> Vec<NodeId> {
        let mut out = Vec::new();
        if parent.0 >= self.meta.len() {
            return out;
        }
        let mut curr = self.meta[parent.0].first_child;
        while let Some(c_id) = curr {
            out.push(c_id);
            curr = self.meta[c_id.0].next_sibling;
        }
        out
    }
}

impl Widget for Container {
    fn render(&self) {
        println!(
            "[QuatUI Container] Rendering container '{}' [State: {:?}] with {} children:",
            self.id, self.state.quat, self.children.len()
        );
        for (i, child) in self.children.iter().enumerate() {
            print!("    [Child {}] ", i);
            child.render();
        }
    }

    fn state(&self) -> UiState {
        self.state
    }

    fn set_state(&mut self, state: UiState) {
        self.state = state;
    }
}