There are two proto packages, `person` and `student`. Students are a specialized
kind of person, so the student library imports the person library.

Each proto package has its own corresponding Rust library under `lib/`. I've got
`lib/person` working just fine, however, `lib/student` panics when generating
code, because it can't resolve the protobuf `import` statement for the `person`
package.

I can partially get around this if I move the proto packages around and rename
them, but in the real-world code I don't have control over any file locations,
package declarations or import paths.
