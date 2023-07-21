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
	loadDotenv()
	generateName()
	setupLogger()

	slog.Info("Starting worker")

	db, err := connectDB()
	if err != nil {
		slog.Error("Unable to connect to database", "error", err)
		os.Exit(1)
	}

	uploads, err := process.Claim(db)
	if err != nil {
		slog.Error("Unable to claim uploads", "error", err)
		os.Exit(1)
	}

	for _, upload := range uploads {
		err := process.Clone(upload)
		if err != nil {
			slog.Error("Unable to process upload", "error", err)
		}
	}

}

func loadDotenv() {
	if _, err := os.Stat(".env"); os.IsNotExist(err) {
		return
	}

	if err := godotenv.Load(".env"); err != nil {
		log.Fatalf("Unable to load .env environment variables: %v", err)
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
