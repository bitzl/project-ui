
struct Value {
    label: String,
    value: String
}


struct Manifest {
    context: String,
    id: String,
    label: String,
    metadata: Vec<Value>,
    sequeneces: Vec<Sequence>,
}

struct Sequence {
    canvases: Vec<Canvas>,
}

struct Canvas {
    id: String,
    t: String,
    height: u32,
    width: u32,
    images: Vec<Image>,
}

struct Image {
    id: String,
    t: String,
    resource: Resource,
}