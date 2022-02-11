use crate::prelude::*;

mod menus;
mod hud;
mod tooltips;
mod inventory;

#[derive(Component)]
pub struct TopUINode;


fn setup (
    asset_server: ResMut<AssetServer>,
    mut commands: Commands,
) {
    let font: Handle<Font> = asset_server.load("fonts/dos.ttf");
    commands.insert_resource(font);
}


pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App)
    {
        app
            .add_startup_system(setup)
            .add_plugin(menus::MenuPlugin)
            .add_plugin(hud::HudPlugin)
            .add_plugin(inventory::InventoryPlugin)
            .add_plugin(tooltips::TooltipsPlugin);
        
    }
}