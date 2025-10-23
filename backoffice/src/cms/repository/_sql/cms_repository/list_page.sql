select cp.id,
       bu.username as author,
       cp.user_id,
       cp.title,
       cp.summary,
       cp.added,
       cp.updated,
       cp.status
from cms_page as cp
         inner join backoffice_users bu on bu.id = cp.user_id;