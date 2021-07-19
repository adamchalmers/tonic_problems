There are two proto libraries, `person` and `student`. Students are a specialized kind of person, so the student library imports the person library.

This works fine if student and person are in the same directory, but in real life, I don't have control over the location of either file. Furthermore, the `student` libraries isn't importing `person.proto`, it's importing `some/host/path/to/person.proto`. How can I tell Tonic to find `some/host/path/to/person.proto` in the right place?