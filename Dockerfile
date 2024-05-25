FROM alpine:latest AS runtime

# add user
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    mtc-user

RUN mkdir -p ./mtc-cms \
  && chown -R mtc-user ./mtc-cms

USER mtc-user

# copy app with assets to container volume
COPY ./target/x86_64-unknown-linux-musl/release/mtc-api ./mtc-cms/
COPY ./public ./mtc-cms/public

# run app
WORKDIR /mtc-cms
EXPOSE 80/tcp
EXPOSE 443/tcp
CMD ["/mtc-cms/mtc-api"]
