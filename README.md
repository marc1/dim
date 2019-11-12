# dim

##Features
- Unit scalar multiplication
- Unit cross multiplication
- Unit addition and subtraction
- Unit formatted printing
- Vector addition and subtraction
- Vector dot product
- Vector cross product
- Vector angle to another vectore
- Vector formatted printing

##Usage

*file: src/main.rs*
```rust
	use dim::vec_3::*;

fn main() {
	println!("Hello dim!");

    let v1 = Vec3::create(1.0, -2.0, 3.0); // Create a Vec3 instance from concrete f64s.
	let v2 = Vec3 { x: Unit::I(1.0), y: Unit::J(-2.0), z: Unit::K(3.0) }; // Or from Units!

    println!("v1: {}", v1);
	println!("v2: {}", v2);
}
```

**Out**
```
Hello dim!
v1: 1.000i -2.000j 3.000k
v2: 1.000i -2.000j 3.000k
```

------------


You can also perform any of the available vector operations.

*file: src/main.rs*
```rust
use dim::vec_3::*;

fn main() {
	println!("Hello dim!");

    let v1 = Vec3::create(1.0, -2.0, 3.0); // Create a Vec3 instance from concrete f64s.
	let v2 = Vec3::create(4.0, 5.0, -6.0); // Or from Units!

    println!("v1 x v2: {}", v1 * v2); // Perform the cross product.
}
```

**Out**
```
Hello dim!
v1 x v2: -3.000i 18.000j 13.000k
```

