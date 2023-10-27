FROM python:latest

WORKDIR /app
COPY . /app

RUN pip install -r requirements.txt

CMD ["sh", "-c", "sleep 10 && pytest"]
