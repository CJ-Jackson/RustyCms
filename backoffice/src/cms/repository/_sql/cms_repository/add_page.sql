insert into cms_page (user_id, title, summary, added, status)
values (:user_id, :title, '', datetime('now'), :status)
returning id;
