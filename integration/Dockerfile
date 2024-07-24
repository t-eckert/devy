FROM python:3.9.18-slim-bullseye

WORKDIR /integration
COPY ./integration /integration

RUN pip install -r requirements.txt --no-cache-dir

CMD ["pytest", "-v"]
