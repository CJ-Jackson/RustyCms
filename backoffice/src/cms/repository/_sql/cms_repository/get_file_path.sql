select file_path
from cms_file_attachment
where id = :id
  and component_id = :component_id;