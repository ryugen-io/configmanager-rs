use crate::api::ContainerInfo;

pub struct ContainerListState {
    pub containers: Vec<ContainerInfo>,
    pub selected_index: usize,
}

impl ContainerListState {
    pub fn new() -> Self {
        Self {
            containers: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn next(&mut self) {
        if !self.containers.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.containers.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.containers.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.containers.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn _selected(&self) -> Option<&ContainerInfo> {
        self.containers.get(self.selected_index)
    }

    pub fn set_containers(&mut self, containers: Vec<ContainerInfo>) {
        self.containers = containers;
        self.selected_index = 0;
    }
}
