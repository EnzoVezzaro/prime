# Ground Truth Sample Queries

Repo: httpx

Query: "AuthService.login"
Intent: Lookup
Expected entities:
- AuthService.login method
- AuthService class

Query: "who calls AuthService.login"
Intent: Callers
Expected entities:
- UserController.authenticate
- SessionManager.refresh

Query: "AuthService"
Intent: Search
Expected top 5 entities:
...
