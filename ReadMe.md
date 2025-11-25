# Bidiak Myhailo. Rust Idioms Homework.
===============================

## Homework task
Made a library that has an ImageConvert struct, which accepts bytes and image format, and converts it to the format that the user asks. To convert from one image format to another, you have to do an intermediate conversion to raw pixel data (RGB32).
 
Use typestate to track the format change, and enforce the format1 -> raw pixels -> format2 state change. Each state should have a get_and_reset() method at each format state, which returns the current image and the format, and resets the state to the default one (Raw pixels). 
 
The supported formats should be JPEG and PNG, but consider that it might expand in the future, and the user shouldn't rely on it. 
 
Use https://crates.io/crates/image library for converting between different types.

## Current library API
- `UserFormat`: public enum for supported formats (`Jpeg`, `Png`). More formats can be added later.
- `ImageConvert<State>`: typestate wrapper with these states:
  - `Loaded`: holds original bytes + declared `UserFormat`. Create with `ImageConvert::<Loaded>::new(bytes, format)`.
  - `Raw`: holds RGBA8 bytes plus stored `width`/`height`.
  - `Encoded<F>`: encoded bytes with target format marker.
- Transitions:
  - `ImageConvert<Loaded>::to_raw()` → decode to RGBA8, capturing `width`/`height`.
  - `ImageConvert<Raw>::to_format(target)` → encode to requested format using stored dimensions.
  - Each state has `get_and_reset()` returning the current bytes and format while resetting to an empty `Raw`.

## Testing the library
Run:
```
cargo run   # ensure examples/sample-jpg-files-sample-5.jpg exists
```

Run tests:
```
cargo test
```
Test contains conversion tests for JPEG <-> PNG using using files in the `examples\` folder as fixtures.

## Content from Rust Idioms
* **`Newtype pattern idiom`** - The Newtype pattern involves creating a new type that wraps an existing type. This is useful for adding type safety, encapsulating behavior, or implementing traits for types that you don't own.
* **`Typestates`** - Typestates is a sequence of states our type is able to be in, and to declare transitions (via functions) between these states. Doing so will allow compiler to cut off incorrect state transitions at compile time.
* **`The Three Core std::mem Idioms`**: std::mem::replace, std::mem::take, and std::mem::swap - These three functions are essential for managing ownership and state in Rust. They allow you to replace, take, or swap values while adhering to Rust's strict ownership rules.
* **`Bind Behavior, Not Data`** - This idiom emphasizes the importance of associating behavior (methods) with types rather than just data. By encapsulating behavior within types, you can create more robust and maintainable code.
* **`Lift unnecessary bounds`** - This idiom involves removing unnecessary trait bounds from generic types or functions. By lifting these bounds, you can make your code more flexible and reusable, allowing it to work with a wider range of types without imposing strict constraints.
* **`Abstracting over input type`** - This idiom focuses on creating abstractions that can handle various input types seamlessly. By designing functions or types that can work with different data structures or formats, you enhance code reusability and adaptability, making it easier to integrate with diverse systems or libraries.
* **`Generic in, type out`** - This idiom involves designing functions or types that accept generic input types but produce specific output types. By using generics for input, you can create flexible and reusable code that can handle various data types, while still providing concrete and well-defined output types for clarity and usability.
* **`Exhaustivity idiom`** - The Exhaustivity idiom involves ensuring that all possible cases or variants of a type are handled explicitly in your code. This is particularly important when working with enums or pattern matching in Rust, as it helps prevent runtime errors and ensures that your code behaves predictably for all possible inputs.
* **`Sealing traits idiom`** - The Sealing traits idiom involves creating traits that cannot be implemented outside of the defining module. This is typically achieved by using a private trait or a private type as a supertrait. Sealing traits is useful for controlling the implementation of certain behaviors and preventing external code from implementing traits in unintended ways, thereby maintaining invariants and encapsulation within your library or module.

