# Rust Rest API 🚀

## 🏁 A simple Rest API using Rust and PostgreSQL

This is a fictional project for laboratory study written in the **Rust** :crab: programming language.

The project is a **Rest API** with **CRUD** functionalities that uses a PostgreSQL database.

_No web framework nor ORM is used_.

The data is stored in the public db schema in the `books` table.

### 1. 💡 Prerequisites

- [Docker](https://www.docker.com/products/docker-desktop/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Curl](https://curl.se/)
- Optional: [Postman](https://www.postman.com/) or [Insomnia](https://insomnia.rest/download)
- Optional: [Tableplus](https://tableplus.com/) or [DBeaver](https://dbeaver.io/)

### 2. 🏃 Running the application with Docker

The application and the database are dockerized :whale:.

To test this project using Docker, just open a terminal in the base directory of the project and run the command:

```bash
docker compose up -d --build
```

### 3. 🏗️ Build project manualy

:radioactive: Run the commands only if you want to build the project manually. If not, skip to next step! :radioactive:

I'm assuming you already have **Rust** and **ToolChain** installed on your workstation.

> Note: Define the database connection variable:

```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
export DATABASE_USERNAME=admin
export DATABASE_PASSWORD=admin
```

Enter the base directory of the project and run the command:

```bash
cargo build --release
```

This will build the app in **release** mode. After running the project:

```bash
cargo run
```

> Note: You can also use the shell script by calling `./run.sh` to build and run the application. This assumes that you have an instance of the PostgreSQL database running locally as required by the script.

### 4. 🧪 Testing the CRUD Endpoints

To test the endpoints, you can use **Postman**, the collections are in the [doc](doc/Rust-Rest-Api.postman_collection.json) folder,
or test via command line with **curl**.

| Method   | EndPoint  | Parameter      | Payload                                   |
| -------- | --------- | -------------- | ----------------------------------------- |
| `POST`   | `/books`  | _not required_ | `{"author":"Author", "title":"My Title"}` |
| `GET`    | `/books/` | ID             | _not required_                            |
| `PUT`    | `/books/` | ID             | `{"author":"Author", "title":"My Title"}` |
| `GET`    | `/books`  | _not required_ | _not required_                            |
| `DELETE` | `/books/` | ID             | _not required_                            |

> Note: The commands below use `curl`.

#### 4.1 📝 Creating a user

Command:

```bash
curl -i -H "Content-Type: application/json" -X POST http://127.0.0.1:8080/users -d '{"name":"User1", "email":"u1@xxx1.com"}'
curl -i -H "Content-Type: application/json" -X POST http://127.0.0.1:8080/users -d '{"name":"User2", "email":"u2@xxx2.com"}'
```

Expected answer:

```
User created
```

#### 4.2 📝 Checking created user with ID

Command:

```bash
curl -i -H "Content-Type: application/json" -X GET http://127.0.0.1:8080/users/1
```

Expected answer:

```json
{ "id": 1, "name": "User1", "email": "u1@xxx1.com" }
```

#### 4.3 📝 Updating user data

Command:

```bash
curl -i -H "Content-Type: application/json" -X PUT http://127.0.0.1:8080/users/1 -d '{"name":"User0", "email":"u0@xxx0.com"}'
```

Expected answer:

```
User updated
```

#### 4.4 📝 Checking all registered users

Command:

```bash
curl -i -H "Content-Type: application/json" -X GET http://127.0.0.1:8080/users
```

Expected answer:

```json
[
  { "id": 2, "name": "User2", "email": "u2@xxx2.com" },
  { "id": 1, "name": "User0", "email": "u0@xxx0.com" }
]
```

#### 4.5 📝 Deleting a user with ID

Command:

```bash
curl -i -H "Content-Type: application/json" -X DELETE http://127.0.0.1:8080/users/1
curl -i -H "Content-Type: application/json" -X DELETE http://127.0.0.1:8080/users/2
```

Expected answer:

```
User deleted
```

### 4. :heavy_multiplication_x: Stop docker

Stop and remove containers, networks, volumes used in this project:

```bash
docker compose down --volumes
```

**Enjoy!** :tropical_drink:
