FROM node:22-bookworm-slim AS build
RUN useradd --create-home pulse
USER pulse
WORKDIR /home/pulse
ARG TARGETPLATFORM
COPY --chown=pulse:pulse ./build/ ./build/
COPY --chown=pulse:pulse build.js build.js
RUN node build.js $TARGETPLATFORM

FROM debian:bookworm-slim AS release
RUN useradd --create-home pulse
USER pulse
WORKDIR /home/pulse
COPY --from=build /home/pulse/uptime-pulse-push ./uptime-pulse-push

ENTRYPOINT ["/home/pulse/uptime-pulse-push"]

