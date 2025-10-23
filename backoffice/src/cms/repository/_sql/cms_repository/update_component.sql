update cms_component
set label=:label,
    raw_data=:raw_data
where id = :id;