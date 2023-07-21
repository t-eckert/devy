package process

import (
	"database/sql"
	"uploader/model"

	"golang.org/x/exp/slog"
)

func Claim(db *sql.DB) ([]model.Upload, error) {
	rows, err := db.Query(`SELECT * FROM "Upload";`)
	defer rows.Close()
	if err != nil {
		return nil, err
	}

	uploads := make([]model.Upload, 0)

	var upload model.Upload
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
