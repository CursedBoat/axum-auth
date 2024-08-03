# Axum Auth
A collection of authentication processes for Axum.

---

# Todo
Below you can find which authentication processes are implemented, and what <br />
is planned for the future.

### Only the very baseline authentication features will be provided. For practical use, please modify the code and add your own features.

- Session Auth ✔
- JWT Auth ❌
- OAuth2 ❌
- Passkeys ❌

# Session Auth
I have implemented session auth using ``tower_http`` by creating a custom function. <br />
On login/register, the server generates and writes a session id to the database, and appends it to the client's cookies. <br />

By default, the session expires in 2 days, and it gets deleted from the database when a user tries to access the protected route. <br />

In an actual usecase, it is advised to refresh and generate a new session everytime the user accesses a protected route, so that the user only gets logged out when they are inactive.

# License
None, do whatever you want with the code, no need to credit me.

# Note
Rename ``database.sqlite.example`` to ``database.sqlite`` & run ``cargo sqlx prepare`` if you want to test the repository.