select id, file_name, file_path, file_type, uploaded
from cms_file_attachment
where component_id = :component_id;