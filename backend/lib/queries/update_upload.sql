update "upload"
set
    previous_upload_id = $2,
    status = $3,
    repo = $4,
    sha = $5,
    logs = $6,
    diff = $7,
    changeset = $8,
    updated_at = NOW()
where id=$1
returning id;
