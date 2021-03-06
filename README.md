# DevPNG

[![devpng](https://img.shields.io/crates/v/devpng.svg)](https://crates.io/crates/devpng)
[![Documentation](https://docs.rs/devpng/badge.svg)](https://docs.rs/devpng)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Note
----

For the moment, this crate is in construction.

If you need something, you can tell me:

- Rust Programming Language Community discord: @Engineer#7038
- mail: pascal.chen@devling.xyz

If you encounter some issues, tell me.

Examples
--------

## Want to modify an image ?
```Rust
use devpng::prelude::PNG;

fn main() -> Result<(), String> {
    // Load.
    let mut buf = std::fs::read("img.png")
        .expect("Couldn't read the file.");
    let mut png = PNG::load(&mut buf)?;

    // Access image.
    let img = png.image();
    
    // Modify image.
    for x in img.iter_mut() {
        *x = !*x;
    }
    
    // Store.
    png.store("img.png")?;
    Ok(())
}
```

## Want to create an image ?
```Rust
use devpng::prelude::{ColourType, Image, PNG, Point};

fn main() -> Result<(), String> {
    // Create.
    let mut data = vec![255; 800 * 200];
    let img = Image::new(&mut data[..])
        .set_ncol(800) // 200
        .set_nrow(200)
        .set_depth(8)
        .set_colour(ColourType::RGBA);
    let mut buf = Vec::new();

    let mut png = PNG::from_image(&mut buf, &img);
    let mut img = png.image();

    for i in 0..50 {
        let center = Point { x: 100, y: 100 };
        let radius = 80 - i as i32;
        let colour = &[0, (255 - i * 5) as u8, (255 - i * 5) as u8, 255];
        img.plot_circle(center, radius, colour);
    }

    // Store.
    png.store("img.png")?;
    Ok(())
}
```

![alt text](https://github.com/TimeEngineer/devpng/blob/master/img/img.png "img")

## Want low level access ?
```Rust
use devpng::prelude::DataStreamMut;

fn main() -> Result<(), String> {
    // Load.
    let mut buf = std::fs::read("img.png")
        .expect("Couldn't read the file.");
    let mut datastream = DataStreamMut::from(&mut buf)?;

    // Access image.
    let mut cache = datastream.idat()?;
    let img = cache.image();
   
    // Modify image.
    for x in img.iter_mut() {
        *x = !*x;
    }
    
    // Store.
    let png = datastream.rebuild(&mut Some(&mut cache));
    std::fs::write("img.png", png)
        .expect("Couldn't write the file.");
    Ok(())
}
```

Images
------

![alt text](https://github.com/TimeEngineer/devpng/blob/master/img/example0.png "0")
![alt text](https://github.com/TimeEngineer/devpng/blob/master/img/example1.png "1")
![alt text](https://github.com/TimeEngineer/devpng/blob/master/img/example2.png "2")
![alt text](https://github.com/TimeEngineer/devpng/blob/master/img/example3.png "3")
![alt text](https://github.com/TimeEngineer/devpng/blob/master/img/example4.png "4")
![alt text](https://github.com/TimeEngineer/devpng/blob/master/img/example5.png "5")

Documentation
-------------

See [RustDoc Documentation](https://docs.rs/devpng).

Installation
------------

Add following lines to your `Cargo.toml`:

```toml
[dependencies]
devpng = "0"
```

References
----------

- DEFLATE: [RFC-1951](https://tools.ietf.org/html/rfc1951)
- PNG: [RFC-2083](https://tools.ietf.org/html/rfc2083)
- W3C: https://www.w3.org/TR/PNG/