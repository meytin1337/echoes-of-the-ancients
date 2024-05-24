use bevy::prelude::*;

#[derive(Event)]
pub struct MouseRightClickEvent(pub Vec2);

pub struct InputHandlingPlugin;
impl Plugin for InputHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input)
            .add_event::<MouseRightClickEvent>();
    }
}
fn handle_input(
    mut mouse_right_click_event_writer: EventWriter<MouseRightClickEvent>,
    button_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window>,
) {
    if button_input.pressed(MouseButton::Left) {
        if let Some(position) = window_query.single().cursor_position() {
            mouse_right_click_event_writer.send(MouseRightClickEvent(position));
        }
    }
}
