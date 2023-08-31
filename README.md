
# Rust REST API Example

This is a sample REST API written in Rust using Actix, the fastest web framework.

It implements two simple endpoints:

- GET / - Returns "Hello World!"
- GET /{id} - Returns the id passed in the path parameter

## Usage
Get main endpoint
```
GET /
```
Response:
```
"Hello World!"
```
Get item by id
```
GET /12345
```
Response:
```
"Item ID: 12345"
```
Replace 12345 with any integer id.

## This API uses:

- Rust - Programming language
- Actix Web - Rust web framework