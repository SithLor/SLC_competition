pub struct RGBA<T> {
    /// Red component
    r: T,
    /// Green component
    g: T,
    /// Blue component
    b: T,
    /// Alpha component
    a: T,
}
impl RGBA<u8> {
    /// Create a new RGB color
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGBA { r, g, b, a: 100 }
    }
    /// Create a new RGB color with alpha
    pub fn new_with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        if a>100 {
            panic!("Alpha value should be less than 100")
        }
        RGBA { r, g, b, a }
    }
    pub fn to_string(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
    pub fn get_r(&self) -> u8 {
        self.r
    }
    pub fn get_g(&self) -> u8 {
        self.g
    }
    pub fn get_b(&self) -> u8 {
        self.b
    }
    pub fn get_a(&self) -> u8 {
        self.a
    }
    pub fn set_r(&mut self, r: u8) {
        self.r = r;
    }
    pub fn set_g(&mut self, g: u8) {
        self.g = g;
    }
    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }
    pub fn set_a(&mut self, a: u8) {
        self.a = a;
    }

}
impl RGBA<u16> {
    /// Create a new RGB color
    pub fn new(r:u16, g: u16, b: u16) -> Self {
        RGBA { r, g, b, a: 100 }
    }
    /// Create a new RGB color with alpha
    pub fn new_with_alpha(r: u16, g:u16, b: u16, a: u16) -> Self {
        if a>100 {
            panic!("Alpha value should be less than 100")
        }
        RGBA { r, g, b, a }
    }
    pub fn to_string(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
    pub fn get_r(&self) -> u16 {
        self.r
    }
    pub fn get_g(&self) -> u16 {
        self.g
    }
    pub fn get_b(&self) -> u16 {
        self.b
    }
    pub fn get_a(&self) -> u16 {
        self.a
    }
    pub fn set_r(&mut self, r: u16) {
        self.r = r;
    }
    pub fn set_g(&mut self, g: u16) {
        self.g = g;
    }
    pub fn set_b(&mut self, b: u16) {
        self.b = b;
    }
    pub fn set_a(&mut self, a: u16) {
        self.a = a;
    }
}
impl RGBA<u32> {
    /// Create a new RGB color
    pub fn new(r:u32, g: u32, b: u32) -> Self {
        RGBA { r, g, b, a: 100 }
    }
    /// Create a new RGB color with alpha
    pub fn new_with_alpha(r: u32, g:u32, b: u32, a: u32) -> Self {
        if a>100 {
            panic!("Alpha value should be less than 100")
        }
        RGBA { r, g, b, a }
    }
    pub fn to_string(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
    pub fn get_r(&self) -> u32 {
        self.r
    }
    pub fn get_g(&self) -> u32 {
        self.g
    }
    pub fn get_b(&self) -> u32 {
        self.b
    }
    pub fn get_a(&self) -> u32 {
        self.a
    }
    pub fn set_r(&mut self, r: u32) {
        self.r = r;
    }
    pub fn set_g(&mut self, g: u32) {
        self.g = g;
    }
    pub fn set_b(&mut self, b: u32) {
        self.b = b;
    }
    pub fn set_a(&mut self, a: u32) {
        self.a = a;
    }
}
