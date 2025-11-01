update cms_component
set position=:position
where id = :id
  and page_id = :page_id;