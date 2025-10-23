update cms_page
set title=:title,
    summary=:summary,
    updated=datetime('now'),
    status=:status
where id = :id;