/// A component is the main building block of a UI set,
/// They are composed of smaller elements.
/// An example would be a UI Popup, or a HUD Health / Mana Pool
/// Each Component is rendered to their own image buffer
pub trait UIComponent {
    fn render(&self);

    /// Called every frame to check if we should re-render
    fn rerender(&self) -> bool;
}
