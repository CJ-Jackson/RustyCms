select cp.user_id
from cms_component as cm
         inner join main.cms_page cp on cp.id = cm.page_id
where cm.id = :id