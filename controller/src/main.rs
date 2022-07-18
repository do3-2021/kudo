
use kudo_controller_lib::internal_etcd;
use etcd_client::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut etcd = internal_etcd::interface::EtcdInterface::new().await;

    // PUT
    etcd.put("hello", "world").await?;
    let mut resp = etcd.get("hello").await?;
    println!("{:?}", resp);
    
    // PATCH
    etcd.patch("hello", "kudo").await?;
    resp = etcd.get("hello").await?;
    println!("{:?}", resp);
    
    // DELETE
    etcd.delete("hello").await?;
    // resp = etcd.get("hello").await?;
    println!("{:?}", resp);

    // GET ALL
    etcd.put("bar", "foo").await?;
    etcd.put("foo", "bar").await?;

    let values = etcd.get_all().await?;
    for value in values {
        println!("{}", value);
    }


    Ok(())
}