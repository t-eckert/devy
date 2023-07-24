FROM golang:1.20-alpine

RUN mkdir /app
COPY . /app
WORKDIR /app

ENV DATABASE_URL=

RUN go build -o uploader .

CMD ["/app/uploader"]
