
use serde_derive::Serialize;
use crate::state::AddLink;

#[derive(Serialize)]
pub struct Links{
    links:Vec<Link>
}
impl Links {


    pub fn new() -> Links{
        Links{
            links:vec![],
        }
    }

    pub fn links(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn add_link(&mut self,link:AddLink) {
        
        self.links.push(Link { title:link.title,url:link.url,id:self.links.len() as u64 + 1 })
    }
}
#[derive(Serialize)]
pub struct Link{
    title:String,
    url:String,
    id:u64
}
impl Link{
    
    pub fn rm_link(id:u64,links_obj:&mut Links) -> Result<usize, ()> {
        let mut index:usize = 0;
        for ele in &links_obj.links {
            if (*ele).id == id {
                break;
            }
            index = index + 1;
        }
        (*links_obj).links.splice(index..index+1, vec![]);
        index = 0;
        let mut new_links = vec![];
        for ele in &links_obj.links {
            new_links.push(Link{
                url:ele.url.clone(),
                title:ele.title.clone(),
                id:index as u64                
            }) 
        }
        links_obj.links = new_links;
        Ok(id as usize)
    }
}
pub type LinkId = u64;


