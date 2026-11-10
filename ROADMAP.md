# Development Roadmap

## Phase 1 — Database Core

- [X] Implement in-memory key-value storage.
- [X] Implement basic database operations.

## Phase 2 — Command Engine

- [X] Implement command parsing.
- [X] Implement command execution.
- [X] Handle invalid commands and arguments.

## Phase 3 — Communication Protocol

- [ ] Implement request parsing.
- [ ] Implement response serialization.
- [ ] Handle multiple command arguments.
- [ ] Handle protocol errors.

## Phase 4 — TCP Server

- [ ] Configure server address and port.
- [ ] Accept client connections.
- [ ] Process incoming commands.
- [ ] Send responses to clients.
- [ ] Support multiple concurrent clients.
- [ ] Share database state between connections.

## Phase 5 — CLI Client

- [ ] Connect to Redisnt server.
- [ ] Send commands and receive responses.
- [ ] Display formatted responses.
- [ ] Implement interactive REPL.
- [ ] Implement connection termination.

## Phase 6 — Testing & Documentation

- [ ] Add tests.
- [ ] Document supported commands.
