# Integration Testing

## Summary

This part of the repo contains all the setup for the TMAG5273 CI Testing Platform. The rough architecture of the platform can be seen below:

![Diagram](../docs/continuous%20integration/Continuous%20Testing%20Pipeline.png)

## How to Run (Raspberry Pi OS)

1. Follow the instructions outlined by Docker [here](https://docs.docker.com/engine/install/raspberry-pi-os/). If your using 64-bit Raspberry Pi OS Follow [these](https://docs.docker.com/engine/install/debian/) instead.
2. Setup your github actions runner token
3. run `docker-compose up -d`
4. Access portainer at `<ip-address>:9000`
5. Monitor your runner :)

## Useful Links

- [GitHub Actions Docker Runner](https://github.com/beikeni/github-runner-dockerfile/tree/main)
- [Dockerising GitHub Actions Article](https://baccini-al.medium.com/how-to-containerize-a-github-actions-self-hosted-runner-5994cc08b9fb)
