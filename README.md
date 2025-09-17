# How Many Contributors

This is the source code of the [howmanycontributors.com](howmanycontributors.com) website.

The purpose of this website is to recursively find the approximate number of contributors of a project, its dependencies, their dependencies etc...

This was a small project to learn Dioxus for the frontend and Axum for the backend, as well as web socket connections for real-time frontend update.

## Repository organization

This repository follow the structure:

```txt
├── crates
│   ├── backend             # The backend service, including the scrapper, DB and cache
│   ├── frontend            # The Dioxus frontend
│   ├── github_scrapper     # Used to fetch dependencies & contributors, can be run as a standalone.
│   └── http_health_checker # Minimalist health checker for Docker HTTP services
├── database                # Database setup scripts
├── manifests               # Configurations & DevOps
│   ├── grafana             # Grafana install scripts & dashboard templates
│   ├── prometheus          # Prometheus configuration
│   └── redis               # Redis configuration
└── scripts                 # Helpers & Runners
    └── git_hooks           # Local development git hooks
```

The backend and frontend services contain a `Dockerfile.debug` for development purposes with hot-reloading, and a `Dockerfile.release` use to create the final optimized image supposed to run in production. Build them at the git root directory.

For development purposes, use the [docker-compose.yml](./docker-compose.yml)

### Services

The frontend service is just a miniserve web server which job is to deliver the Dioxus WASM payload.

The backend service fetches contributors and dependencies from GitHub, stores them for a day in Redis to act as a cache, and also store them in Postgres to act as a cache for dependencies, and as a database to keep a history of contributors and dependencies per repository.

Everything run within Docker.

```txt
+----------+             +----------+                 
| Internet | <-exposed-> | Frontend |                 
+----------+             +----------+                 
     |                +----------+                +---------+                 
     ----exposed----> | Backend  | <------------> | GitHub  |                 
                      +----------+                +---------+                 
                         ^  ^  ^                   
                         |  |  |                   
              |----------|  |  |------------|      
              v             v               v      
       +------------+  +----------+   +----------+ 
       | Prometheus |  |  Redis   |   | Postgres | 
       +------------+  +----------+   +----------+ 
              |             |               |      
              |-------------|---------------|      
                            v                      
                       +----------+                
                       | Grafana  |                
                       +----------+                
```

## Quick Start

### Full service

To run this project locally, clone this repository and run docker compose:

```cmd
git clone https://github.com/BaptisteRoseau/howmanycontributors.com.git
cd howmanycontributors.com.git
docker compose up
```

/!\ Currently Dioxus has compilation issues /!\

This will run the debug build, enabling hot reloading to work on both the frontend and the backend. Wait for the build to complete, then run [scripts/open_firefox.debug.sh](scripts/open_firefox.debug.sh) (or open manually your browser if not Firefox on the links in the script).

For the services credentials, refer to [docker-compose.yml](./docker-compose.yml).

### Scrapper Only

You can also build and run the GitHub scrapper independently, for example:

```cmd
$ cargo run --bin github_scrapper
Usage: target/debug/github_scrapper <url>
```

### Development

#### Git Hooks

While developing the project, it is strongly advised to set up local git hooks. This can be achieved by running the [scripts/git_hooks/setup.sh](scripts/git_hooks/setup.sh) script. Afterwards, each commit will:

1. link the code (clippy)
1. test the frontend
1. test the backend with coverage
1. check for license compliance (cargo deny)
1. check for known vulnerabilities (cargo audit)

You can disable this behavior by using:

```cmd
chmod -x .git/hooks/pre-commit
```

### Release

To build the release docker images, run [scripts/build_release.sh](scripts/build_release.sh).
