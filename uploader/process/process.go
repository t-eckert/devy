package process

import (
	"database/sql"

	"golang.org/x/exp/slog"
)

func Process(db *sql.DB) error {
	// Claim uploads from the database.
	uploads, err := Claim(db)
	if err != nil {
		return err
	} else {
		slog.Info("Claimed uploads.", "count", len(uploads))
	}

	for _, upload := range uploads {
		repo, err := Clone(upload)
		if err != nil {
			slog.Error("Unable to clone upload", "error", err)
		}

		err = Diff(repo)
		if err != nil {
			slog.Error("Unable to diff upload", "error", err)
		}

	}

	return nil
}
