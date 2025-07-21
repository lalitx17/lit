# LIT

It's literally git but it's lit.

This is a simple implementation of a version control system inspired by Git, designed to help you understand the core concepts of version control.

## Architecture

![Lit Architecture](assets/lit-architecture.png)

### Functionality

| Command  | Functionality                                               |
| -------- | ----------------------------------------------------------- |
| init     | Initialize a new lit repository                             |
| add      | Add file contents to the staging area                       |
| commit   | Record changes to the repository                            |
| branch   | List branches                                               |
| checkout | create branches(with -n flag) or restore working tree files |
| switch   | Switch branches                                             |
| log      | Show commit logs                                            |
| show     | Show various types of objects (commits, tree, blobs)        |
