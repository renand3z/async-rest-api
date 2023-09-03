# Todo REST API Example

This is a sample CRUD REST API for managing Todo items, implemented in Rust using Actix web, the fastest web framework.

## Features

- REST API with JSON responses
- Supports all CRUD operations:
  - GET todos
  - POST create todo
  - GET single todo
  - PUT update todo
  - DELETE todo
- Uses Actix web async framework
- In-memory storage of Todos (no database)

## Usage

### Get all todos

```
GET /todos
```

### Create a new todo

```
POST /todos 
{
  "text": "My new todo item" 
}
```

### Get a todo

```
GET /todos/1
```

### Update a todo

```
PUT /todos/1
{
  "completed": true
} 
```

### Delete a todo

```
DELETE /todos/1
```

## Running Locally

```
cargo run
```

Starts server on [http://localhost:8080 ↗](http://localhost:8080)

## Implementation

- `main.rs` - Sets up Actix App and /todos route handler
- `routes.rs` - CRUD handlers for each HTTP method 
- `todo.rs` - Todo struct representing item 
- Uses Actix web, serde JSON  
- Async request handling with Rust futures