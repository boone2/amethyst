use super::button::Button;
use super::local_mouse_button::LocalMouseButton;
use super::local_virtual_key_code::LocalVirtualKeyCode;

use winit::{MouseButton, VirtualKeyCode};

/// Events generated by the input system
///
/// Type parameter T is the type assigned to your Actions for your
/// InputBundle or InputHandler.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InputEvent<T> {
    /// A key was pressed down, sent exactly once per key press.
    KeyPressed {
        #[serde(with = "LocalVirtualKeyCode")]
        key_code: VirtualKeyCode,
        scancode: u32,
    },
    /// A key was released, sent exactly once per key release.
    KeyReleased {
        #[serde(with = "LocalVirtualKeyCode")]
        key_code: VirtualKeyCode,
        scancode: u32,
    },
    /// A unicode character was received by the window.  Good for typing.
    KeyTyped(char),
    /// A mouse button was pressed down, sent exactly once per press.
    MouseButtonPressed(#[serde(with = "LocalMouseButton")] MouseButton),
    /// A mouse button was released, sent exactly once per release.
    MouseButtonReleased(#[serde(with = "LocalMouseButton")] MouseButton),
    /// A button was pressed.
    ButtonPressed(Button),
    /// A button was released.
    ButtonReleased(Button),
    /// The mouse pointer moved on screen
    CursorMoved { delta_x: f64, delta_y: f64 },
    /// The mouse device moved.  Use this for any use of the mouse that doesn't involve a standard mouse cursor.
    MouseMoved { delta_x: f64, delta_y: f64 },
    /// The associated action had one of its keys pressed.
    ActionPressed(T),
    /// The associated action had one of its keys released.
    ActionReleased(T),
}
