insert into cms_component (page_id, kind_uuid, position, raw_data, label)
VALUES (:page_id, :kind_uuid, (select (ifnull(max(position), -1)) + 1 as current_position
                               from cms_component
                               where page_id = :page_id
                               order by position desc
                               limit 1), :raw_data, :label)
returning id;