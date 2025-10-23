select id, kind_uuid, position, label
from cms_component
where page_id = :page_id
order by position;