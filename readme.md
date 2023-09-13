# idle_game

## NOTE This is a copy from an obsidian notebook I started to just get my ideas down. This is not finished and will, most likely, be completely changed. If you are contributing to this project feel free to open a PR to add, remove, or change any part of this.

Synopsis: Create an opensource idle game that is just an api endpoint, so that anyone can use any language to play it.

Needs:

- #infrastructure
  - whole "game server" is just a docker container, can be ran on anything.
- #code
  - probably should be written in something fast, #rust?
    - https://docs.rs/num-bigint/0.4.4/num_bigint/index.html
  - monolith
  - authentication
    - nothing special
    - simple user / pass stored in some simple db
    - when a user is created generate an api key and store it as well.
    - when a user is created or logs in return the api key
    - just check for api key in all requests
  - endpoints
    - / (get)
      - a simple home screen (terminal based) that explains whats up
    - /login (post)
      - take user / pass return pre generated api key if successful
    - /signup (post)
      - take a user / pass and generate api key and create db entry
    - /summary (get)
      - shows tasks completed, rewards, unlocks, items, etc
    - /tasks (get)
      - shows available tasks, updated when new tasks are unlocked
    - /task/{task} (get)
      - get what needs to be done to complete task
    - /task/{task} (put)
      - return the work, server checks it, rewards if correct, returns error if incorrect.
    - /store (get)
      - shows items in the store
    - /store/{item} (get)
      - shows item description
    - /store/{item} (post)
      - attempt to purchase item
  - #gameplay
    - Player has to write their own "front end" to interact with game.
    - When a player completes a task, they are given a reward
    - Tasks are repeatable infinitely
    - Rewards from previous tasks are required to complete future tasks
    - How to keep track of players? Single player per server, (no)? Login and passback auth key?
    - login adds complexity, and will require a db (SQLite)
    - Rate limit players? Make rate limit increases a reward?
    - if player continuously hits against rate limit, time out player
    - Keep track of player resources on server? (in db)
    - Otherwise would have to pass some immutable "wallet" around...
    - Store
      - Player can spend resources in the store
      - Store items effect how player interacts with game
      - Big numbers
      - Tasks
        - start easy and then add more complexity, based around things you would normally do as a programmer.
        - first group of tasks, each unlocked individually
          - add two numbers (rewards 1 bit per completion)
          - add two floats (requires 8k bits to unlock, rewards 4 bits)
          - concatenate two strings (requires 16k, rewards 8 bits)
          - split two strings (requires 32k, rewards 16 bits)
          - multiply all numbers in array (requires 64k, rewards 32 bits)
          - multiply all numbers in array, then divide by total array elements (requires 128k, rewards 64 bits)
        - second group of tasks, purchased from shop for 512k
          - add two numbers in json object (requires 64k, rewards 32 bits)
          - add numbers in arrays in json, return result in json (requires 64k, rewards 32 bits
          - take data from array of jsons and combine them, no conflicts (requires 64k, rewards 32 bits)
          - take data from array of jsons and combine them, with conflicts (requires 64k, rewards 32 bits)
          - validate json against a schema, receive schema from get endpoint (requires 64k, rewards 32 bits)
        - third group of tasks - ???
