package main

import (
	"database/sql"

	"golang.org/x/exp/slog"
)

type Upload struct {
	Id     *string
	User   *string
	Status *string
	Repo   *string
}

func ClaimUploads(db *sql.DB) ([]Upload, error) {
	rows, err := db.Query(`SELECT * FROM "Upload";`)
	defer rows.Close()
	if err != nil {
		return nil, err
	}

	uploads := make([]Upload, 0)

	var upload Upload
	for rows.Next() {
		rows.Scan(
			&upload.Id,
			&upload.User,
			&upload.Status,
			&upload.Repo,
		)
		uploads = append(uploads, upload)
		slog.Info("Claimed upload",
			"id", *upload.Id,
			"user", *upload.User,
			"repo", *upload.Repo,
		)
	}

	return uploads, nil
}
