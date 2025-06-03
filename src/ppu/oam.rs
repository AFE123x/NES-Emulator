/// Represents a single sprite entry in the NES's Object Attribute Memory (OAM).
///
/// Each sprite is defined by four bytes:
/// - Y position (relative to scanlines)
/// - Tile index number
/// - Attribute byte (palette, flipping, priority)
/// - X position (horizontal position on screen)
///
/// This structure encapsulates a single 4-byte sprite record.
#[derive(Debug)]
pub struct Oam {
    y_position: u16,
    index_number: u8,
    attribute: u8,
    x_position: u8,
}

impl Clone for Oam {
    /// Creates a clone of this OAM entry.
    fn clone(&self) -> Self {
        Self {
            y_position: self.y_position,
            index_number: self.index_number,
            attribute: self.attribute,
            x_position: self.x_position,
        }
    }
}

impl Oam {
    /// Creates a new default OAM entry with all fields set to zero.
    pub fn new() -> Self {
        Self {
            y_position: 0,
            index_number: 0,
            attribute: 0,
            x_position: 0,
        }
    }

    /// Returns the Y position of the sprite.
    ///
    /// This value determines where vertically on the screen the sprite is drawn.
    pub fn get_y_position(&self) -> u16 {
        self.y_position
    }

    /// Returns the X position of the sprite.
    ///
    /// This value determines the horizontal position on the screen.
    pub fn get_x_position(&self) -> u8 {
        self.x_position
    }

    /// Returns the attribute byte of the sprite.
    ///
    /// The attribute byte encodes palette selection, flipping (horizontal/vertical),
    /// and sprite priority (in front or behind background).
    pub fn get_attribute(&self) -> u8 {
        self.attribute
    }

    /// Returns the tile index number of the sprite.
    ///
    /// This is used to look up tile graphics in pattern memory.
    pub fn get_index_number(&self) -> u8 {
        self.index_number
    }

    /// Sets a byte of the OAM entry by index.
    ///
    /// The `address` should be in range `[0, 3]`, corresponding to:
    /// - `0`: Y position
    /// - `1`: Tile index
    /// - `2`: Attributes
    /// - `3`: X position
    ///
    /// # Panics
    /// Panics if `address % 4` is not in range `[0, 3]`, which should be impossible.
    pub fn set_byte(&mut self, address: u8, data: u8) {
        match address % 4 {
            0 => {
                self.y_position = data as u16;
            }
            1 => {
                self.index_number = data;
            }
            2 => {
                self.attribute = data;
            }
            3 => {
                self.x_position = data;
            }
            _ => {
                panic!("erm, what the sigma");
            }
        };
    }

    /// Gets a byte of the OAM entry by index.
    ///
    /// This allows treating the OAM entry as a 4-byte array for sequential reading.
    ///
    /// # Panics
    /// Panics if `address % 4` is out of range `[0, 3]`.
    pub fn get_byte(&mut self, address: u8) -> u8 {
        match address % 4 {
            0 => self.y_position as u8,
            1 => self.index_number,
            2 => self.attribute,
            3 => self.x_position,
            _ => {
                panic!("erm, what the sigma");
            }
        }
    }
}