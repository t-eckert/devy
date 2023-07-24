package main

import (
	"database/sql"
	"fmt"
	"log"
	"os"
	"uploader/process"

	"github.com/joho/godotenv"
	_ "github.com/lib/pq"
	"golang.org/x/exp/slog"
)

var name string

func main() {
	if _, err := os.Stat(".env"); err == nil {
		if err := godotenv.Load(".env"); err != nil {
			log.Fatalf("Unable to load .env environment variables: %v", err)
		}
	}

	generateName()
	setupLogger()

	slog.Info("Starting worker")

	db, err := connectDB()
	if err != nil {
		slog.Error("Unable to connect to database.", "error", err)
		os.Exit(1)
	} else {
		slog.Info("Connected to database.")
	}

	if err := process.Process(db); err != nil {
		slog.Error("Unable to process uploads.", "error", err)
		os.Exit(1)
	}
}

func setupLogger() {
	logger := slog.New(slog.NewTextHandler(os.Stdout, nil))
	slog.SetDefault(logger.With("name", name))
}

func connectDB() (*sql.DB, error) {
	conn := os.Getenv("DATABASE_URL")
	if conn == "" {
		return nil, fmt.Errorf("Missing environment variable DATABASE_URL.")
	}

	return sql.Open("postgres", conn)
}
