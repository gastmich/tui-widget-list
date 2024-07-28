use ratatui::widgets::Widget;

use crate::ScrollAxis;

/// This trait should be implemented for items that are intended to be used within a `List` widget.
pub trait PreRender: Widget {
    /// This method is called before rendering the widget.
    ///
    /// # Arguments
    ///
    /// - `self`: Captured by value, allowing modification within the pre-render hook.
    /// - `context`: Rendering context providing additional information like selection
    ///    status, cross-axis size, scroll direction and the widgets index in the list.
    ///
    /// # Returns
    ///
    /// - The (modified) widget.
    /// - The widget's main axis size, used for layouting.
    ///
    /// # Example
    ///
    ///```ignore
    /// use ratatui::prelude::*;
    /// use tui_widget_list::{PreRenderContext, PreRender};
    ///
    /// impl PreRender for MyWidget {
    ///     fn pre_render(self, context: &PreRenderContext) -> (Self, u16) {
    ///         // Modify the widget based on the selection state
    ///         if context.is_selected {
    ///             self.style = self.style.reversed();
    ///         }
    ///
    ///         // Example: set main axis size to 1
    ///         let main_axis_size = 1;
    ///
    ///         (self, main_axis_size)
    ///     }
    /// }
    /// ```
    fn pre_render(&mut self, context: &PreRenderContext) -> u16;
}

/// The context provided during rendering.
///
/// It provides a set of information that can be used from [`PreRender::pre_render`].
#[derive(Debug, Clone)]
pub struct PreRenderContext {
    /// Indicates whether the widget is selected.
    pub is_selected: bool,

    /// The cross axis size of the widget.
    pub cross_axis_size: u16,

    /// The list's scroll axis:
    /// - `vertical` (default)
    /// - `horizontal`
    pub scroll_axis: ScrollAxis,

    /// The index of the widget in the list.
    pub index: usize,
}

impl PreRenderContext {
    pub(crate) fn new(
        is_selected: bool,
        cross_axis_size: u16,
        scroll_axis: ScrollAxis,
        index: usize,
    ) -> Self {
        Self {
            is_selected,
            cross_axis_size,
            scroll_axis,
            index,
        }
    }
}
