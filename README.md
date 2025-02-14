# AiroAPI
For a self-hosted caching API server, for Aerodatabox. Check out examples in the docker-compose.yml file.

## Usage

Build docker image:

```docker build -t airoapi:latest .```

...or pull from ghcr.io:

```docker pull ghcr.io/tuxy/airoapi:latest```

...or run the sample docker-compose.yml file for a small deployment:

```docker compose up -d```

Which will be hosted on port 80, with easy scalability