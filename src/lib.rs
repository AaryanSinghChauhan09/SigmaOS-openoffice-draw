#[derive(Debug, Clone, PartialEq)]
pub enum ShapePrimitive {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
    Line { x1: f64, y1: f64, x2: f64, y2: f64 },
}

pub struct DrawLayer {
    pub name: String,
    pub shapes: Vec<ShapePrimitive>,
}

impl DrawLayer {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), shapes: Vec::new() }
    }

    pub fn add_shape(&mut self, shape: ShapePrimitive) {
        self.shapes.push(shape);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_shapes() {
        // Arrange
        let mut layer = DrawLayer::new("Layer 1");
        // Act
        layer.add_shape(ShapePrimitive::Rectangle { width: 100.0, height: 50.0 });
        // Assert
        assert_eq!(layer.shapes.len(), 1);
    }
}
