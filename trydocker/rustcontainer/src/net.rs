use rtnetlink::new_connection;
use tokio;
pub async fn netrun(){   
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);
    handle
        .link()
        .add()
        .veth("veth-rs-1".into(), "veth-rs-2".into())
        .execute()
        .await
        .map_err(|e| format!("{}", e));
    //let mut request = handle.link().add().veth("host".to_string(),"guest".to_string());
    
}

pub fn netchild(){

}