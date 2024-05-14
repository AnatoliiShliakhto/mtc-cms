FROM alpine AS runtime
MAINTAINER Anatolii Shliakhto <a.shlyakhto@gmail.com>

RUN addgroup -S coolguys && adduser -S seiko -G seiko
COPY /target/x86_64-unknown-linux-gnu/release/mtc-api /usr/local/bin/
USER seiko

EXPOSE  8080

CMD ["/usr/local/bin/mtc-api"]
