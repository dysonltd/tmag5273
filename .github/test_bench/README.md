# Integration Testing

## Summary

This part of the repo contains all the setup for the TMAG5273 CI Testing Platform. The rough architecture of the platform can be seen below:

![Diagram](../../docs/ci%20pipeline%20diagram.drawio.svg)

## How to Run (Raspberry Pi OS)

1. Follow the instructions outlined by Docker [here](https://docs.docker.com/engine/install/raspberry-pi-os/). If your using 64-bit Raspberry Pi OS Follow [these](https://docs.docker.com/engine/install/debian/) instead.
2. Setup your github actions runner token
3. Add the Token to your`.env` file
4. Enable I2C using `raspi-config`
5. run `docker-compose up -d`
6. Access portainer at `<ip-address>:9000`
7. Monitor your runner :)

## Pinouts and Wiring

For the Above CI Test Bench to work you must connect the following pins:

### Raspberry Pi

| Pin Number (BOARD) | TMAG5273 Pin Name |
|--------------------|-------------------|
| 2   (SDA)          |      STEMA        |
| 3   (SCL)          |      STEMA        |
| 4                  | INT               |

### FT232H Breakout Board (Connected to Pi5 over USB)

| Pin Number (BOARD) | TMAG5273 Pin Name |
|--------------------|-------------------|
| STEMA              |  STEMA            |
| C0                 | INT               |

### ESP32C3 QTPY Board (Connected to Pi5 over USB)

| Pin Number (BOARD) | TMAG5273 Pin Name |
|--------------------|-------------------|
| STEMA              |  STEMA            |
| A0                 | INT               |

## Useful Links

- [GitHub Runner DockerHub](https://hub.docker.com/r/myoung34/github-runner)
- [GitHub Runner GitHub repo](https://github.com/myoung34/docker-github-actions-runner)
