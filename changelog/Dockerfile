FROM python:3.9.18-slim-bullseye

WORKDIR /app
COPY ./changelog /app

EXPOSE 5000
CMD ["python", "-m", "http.server", "5000"]
