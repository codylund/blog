# Blog
## Overview
A simple blog solution which utilizes GitHub for storing and managing blog content.

## Organization
### /server [TBI]
Will have the single responsibility of serving up the latest client-side web application. Will not store any of the actual blog content (I don't want to build out a web API to send the new content nor do I wan't to poke the remote VM everytime I want to share new content).

### /client
Contains the client-side program which is built in Rust and compiled into WASM. Will query the latest blog entries from this repository via the GitHub REST API.

### /blog
Contains the blog entries. Each is stored in a seperate file. 