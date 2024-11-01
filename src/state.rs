#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct ListState {
    /// The selected item. If `None`, no item is currently selected.
    pub selected: Option<(usize, Option<usize>)>,

    /// The total number of elements in the list. This is necessary to correctly
    /// handle item selection.
    pub(crate) num_elements: Vec<usize>,

    /// The expanded main elements in the list
    pub(crate) expanded: Vec<usize>,

    /// Indicates if the selection is circular. If true, calling `next` on the last
    /// element returns the first, and calling `previous` on the first returns the last.
    ///
    /// True by default.
    pub(crate) infinite_scrolling: bool,

    /// The state for the viewport. Keeps track which item to show
    /// first and how much it is truncated.
    pub(crate) view_state: ViewState,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub(crate) struct ViewState {
    /// The index of the first item displayed on the screen.
    pub(crate) offset: usize,

    /// The truncation in rows/columns of the first item displayed on the screen.
    pub(crate) first_truncated: u16,
}

impl Default for ListState {
    fn default() -> Self {
        Self {
            selected: None,
            expanded: vec![],
            num_elements: vec![],
            infinite_scrolling: true,
            view_state: ViewState::default(),
        }
    }
}

impl ListState {
    pub(crate) fn set_infinite_scrolling(&mut self, infinite_scrolling: bool) {
        self.infinite_scrolling = infinite_scrolling;
    }

    /// Returns the index of the currently selected item, if any.
    #[must_use]
    #[deprecated(since = "0.9.0", note = "Use ListState's selected field instead.")]
    pub fn selected(&self) -> Option<usize> {
        self.selected.map(|selected| selected.0)
    }

    /// Selects an item by its index.
    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index.map(|index| (index, None));
        if index.is_none() {
            self.view_state.offset = 0;
        }
    }

    /// Selects a child item by its index.
    pub fn select_child(&mut self, index: Option<(usize, Option<usize>)>) {
        self.selected = index;
        if index.is_none() {
            self.view_state.offset = 0;
        }
    }

    /// collapse all items in the list
    pub fn collapse_all(&mut self) {
        self.expanded.clear();
        // reset the seletion to the main entry
        if let Some(selected) = &mut self.selected {
            selected.1 = None;
        }
    }

    /// collapse the current selected item in the list
    pub fn collapse_selected(&mut self) {
        if let Some(selected) = &mut self.selected {
            // keep only entries where x != selected
            self.expanded.retain(|&x| x != selected.0);
            // reset the seletion to the main entry
            selected.1 = None;
        }
    }

    /// expand all items in the list
    pub fn expand_all(&mut self) {
        self.expanded.clear();
        for i in 0..self.num_elements.len() {
            self.expanded.push(i)
        }
    }

    /// expand the current selected item in the list
    pub fn expand_selected(&mut self) {
        if let Some(selected) = self.selected {
            if !self.is_expanded(selected.0) {
                self.expanded.push(selected.0);
            }
        }
    }

    pub fn get_selected_child(&self, index: usize) -> Option<usize> {
        self.selected.map(|selected| if selected.0 == index { selected.1 } else { None })?
    }

    pub fn is_selected(&self, index: usize) -> bool {
        self.selected.map(|selected| selected.0 == index).unwrap_or_default()
    }

    pub fn is_expanded(&self, index: usize) -> bool {
        self.expanded.iter().find(|&&x| x == index).is_some()
    }

    /// Selects the next element of the list. If circular is true,
    /// calling next on the last element selects the first.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tui_widget_list::ListState;
    ///
    /// let mut list_state = ListState::default();
    /// list_state.next();
    /// ```
    pub fn next(&mut self) {
        if self.num_elements.is_empty() {
            return;
        }

        fn next_main_item(index: usize, num_elements: usize, infinite_scrolling: bool) -> (usize, Option<usize>) {
            if index >= num_elements - 1 {
                if infinite_scrolling {
                    (0, None)
                } else {
                    (index, None)
                }
            } else {
                (index + 1, None)
            }
        }

        let i = match self.selected {
            Some((i, j)) => {
                match self.num_elements.get(i) {
                    Some(sub_items) => {
                        if self.is_expanded(i) {
                            if j.is_none() {
                                (i, Some(0))
                            } else {
                                let j = j.unwrap();
                                if j >= sub_items.saturating_sub(1) {
                                    next_main_item(i, self.num_elements.len(), self.infinite_scrolling)
                                } else {
                                    (i, Some(j + 1))
                                }
                            }
                        } else {
                            // not expanded
                            next_main_item(i, self.num_elements.len(), self.infinite_scrolling)
                        }
                    },
                    None => next_main_item(i, self.num_elements.len(), self.infinite_scrolling),
                }
            }
            None => (0, None),
        };
        self.select_child(Some(i));
    }

    /// Selects the previous element of the list. If circular is true,
    /// calling previous on the first element selects the last.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tui_widget_list::ListState;
    ///
    /// let mut list_state = ListState::default();
    /// list_state.previous();
    /// ```
    pub fn previous(&mut self) {
        if self.num_elements.is_empty() {
            return;
        }

        fn prev_main_item(index: usize, num_elements: &Vec<usize>, expanded: &Vec<usize>, infinite_scrolling: bool) -> (usize, Option<usize>) {
            if index == 0 && !infinite_scrolling { return (index, None); }
            let prev_index = if index == 0 { num_elements.len().saturating_sub(1) } else { index - 1 };
            if expanded.iter().find(|&&x| x == prev_index).is_some() {
            //if expanded.get(prev_index).is_some_and(|expanded|*expanded == prev_index) {
                let sub_element = match num_elements.get(prev_index) {
                    Some(len) => {
                        if *len == 0 { None }
                        else { Some(len -1) }
                    },
                    None => None,
                };
                (prev_index, sub_element)
            } else {
                (prev_index, None)
            }
        }

        let i = match self.selected {
            Some((i, j)) => {
                match self.num_elements.get(i) {
                    Some(_) => {
                        if let Some(j) = j {
                            if j == 0 {
                                // select main element
                                (i, None)
                            } else {
                                (i, Some(j - 1))
                            }
                        } else {
                            //select prev main item in list
                            prev_main_item(i, &self.num_elements, &self.expanded, self.infinite_scrolling)
                        }
                    },
                    None => prev_main_item(i, &self.num_elements, &self.expanded, self.infinite_scrolling),
                }
            }
            None => (0, None),
        };
        self.select_child(Some(i));
    }

    /// Updates the number of elements that are present in the list.
    pub(crate) fn set_num_elements(&mut self, num_elements: Vec<usize>) {
        self.expanded.retain(|&x| x < num_elements.len());
        self.num_elements = num_elements;
    }
}
