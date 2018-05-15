pub struct Rectangle<T, S> {
    pub x: T,
    pub y: T,
    pub width: S,
    pub height: S,
}

impl<T: PartialEq, S:PartialEq> Rectangle<T, S> {
    pub fn is_square(&self) -> bool {
        self.width == self.height &&
          self.x == self.y
    }

    pub fn update_y(&mut self, h:S) -> () {
        self.height = h;
        ()
    }
}
