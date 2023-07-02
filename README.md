# SETUP

- `brew install sqlite`
- `cargo install diesel_cli --no-default-features --features "sqlite-bundled"`
- `cargo install --path .`
- `diesel setup`

# Generate Migration

- `diesel migration generate create_users`
- edit `migrations\...\up.sql`

- check using curl
  - Get user
    ```shell
      curl "http://localhost:8080/getusers"
    ```
  - Post newuser
    ```shell
      curl "http://localhost:8080/users" \
          -X POST \
          -d "{\r\n  \"name\": \"Fey\",\r\n  \"address\": \"145 Av Stovia\"\r\n}" \
          -H "content-type: application/json"
    ```
