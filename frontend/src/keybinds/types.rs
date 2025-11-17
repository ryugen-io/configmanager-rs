use serde::Deserialize;

#[derive(Deserialize)]
pub struct Keybinds {
    pub menu: MenuKeybinds,
    pub file_list: FileListKeybinds,
    pub container_list: ContainerListKeybinds,
    pub global: GlobalKeybinds,
}

#[derive(Deserialize)]
pub struct MenuKeybinds {
    pub navigate_down: String,
    pub navigate_down_alt: String,
    pub navigate_up: String,
    pub navigate_up_alt: String,
    pub select: String,
}

#[derive(Deserialize)]
pub struct FileListKeybinds {
    pub navigate_down: String,
    pub navigate_down_alt: String,
    pub navigate_up: String,
    pub navigate_up_alt: String,
    pub select: String,
    pub back_to_menu: String,
    pub go_to_editor: String,
}

#[derive(Deserialize)]
pub struct ContainerListKeybinds {
    pub navigate_down: String,
    pub navigate_down_alt: String,
    pub navigate_up: String,
    pub navigate_up_alt: String,
    pub start_container: String,
    pub stop_container: String,
    pub restart_container: String,
    pub back_to_menu: String,
}

#[derive(Deserialize)]
pub struct GlobalKeybinds {
    pub save: String,
    pub back_to_files: String,
    pub cycle_theme: String,
}
