//Module pour la gestion de la scène (gestion des objets et des lumières)

struct Scene {
    image_size: (u32, u32),
    background_color: String,
    camera: Camera,
    light: Light,
    shapes: Vec<Shape>,
}