fn main() {
    struct Thing {
        x: u32,
        y: u32,
    }

    let thing = Thing { x: 0, y: 0 };

    match thing {
        Thing { x, y: z } => {}
    }
}
