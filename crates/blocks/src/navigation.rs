use crate::Block;
use std::{collections::HashMap, usize};

///Represent navigation state between blocks
pub struct BlockNavigation {
    current_block_id: Option<usize>,
    ///current focused block ID
    history: Vec<usize>,
    ///Block jump history
    history_position: usize,
    /// history position (for forward/back navigation)
    bookmarks: HashMap<String, usize>,
}

impl BlockNavigation {
    pub fn new() -> Self {
        Self {
            current_block_id: None,
            history: Vec::new(),
            history_position: 0,
            bookmarks: HashMap::new(),
        }
    }

    ///Set the current block and update navigation history
    pub fn set_current_block(&mut self, block_id: usize) {
        ///Don't add duplicate history entries
        if Some(block_id) != self.current_block_id {
            if self.history_position < self.history.len() {
                self.history.truncate(self.history_position);
            }

            //Add the current block to history if it exists
            if let Some(current) = self.current_block_id {
                self.history.push(current);
                self.history_position += 1;
            }

            self.current_block_id = Some(block_id);
        }
    }

    ///Navigation back in history
    pub fn go_back(&mut self) -> Option<usize> {
        if self.history_position > 0 {
            self.history_position -= 1;
            let block_id = self.history[self.history_position];
            self.current_block_id = Some(block_id);
            Some(block_id)
        } else {
            None
        }
    }

    //Navigate forward in history
    pub fn go_forward(&mut self) -> Option<usize> {
        if self.history_position < self.history.len() {
            let block_id = self.history[self.history_position];
            self.history_position += 1;
            self.current_block_id = Some(block_id);
            Some(block_id)
        } else {
            None
        }
    }

    ///Bookmark a block with a name
    pub fn bookmark(&mut self, name: &str, block_id: usize) {
        self.bookmarks.insert(name.to_string(), block_id);
    }

    ///Go to a bookmarked block
    pub fn go_to_bookmark(&mut self, name: &str) -> Option<usize> {
        if let Some(&block_id) = self.bookmarks.get(name) {
            self.set_current_block(block_id);
            Some(block_id)
        } else {
            None
        }
    }

    ///Get a current block ID
    pub fn current_block_id(&self) -> Option<usize> {
        self.current_block_id
    }

    ///Get list of bookmarks
    pub fn get_bookmarks(&self) -> &HashMap<String, usize> {
        &self.bookmarks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation() {
        let mut nav = BlockNavigation::new();

        //Test setting current block
        nav.set_current_block(1);
        assert_eq!(nav.current_block_id(), Some(1));

        nav.set_current_block(2);
        assert_eq!(nav.current_block_id(), Some(2));

        nav.set_current_block(3);
        assert_eq!(nav.current_block_id(), Some(3));

        assert_eq!(nav.go_back(), Some(2));
        assert_eq!(nav.current_block_id(), Some(2));

        assert_eq!(nav.go_forward(), Some(3));
        assert_eq!(nav.current_block_id(), Some(3));

        nav.bookmark("test", 2);
        assert_eq!(nav.go_to_bookmark("test"), Some(2));
        assert_eq!(nav.current_block_id(), Some(2));
    }
}
