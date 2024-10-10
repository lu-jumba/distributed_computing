use aws_sdk_ec2::{Client, Error, Region};

pub async fn launch_ec2_instance() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let run_instance = client
        .run_instances()
        .image_id("ami-12345678")  // Replace with actual AMI
        .instance_type("t2.micro")
        .min_count(1)
        .max_count(1)
        .send()
        .await?;

    println!("Launched EC2 instance: {:?}", run_instance.instances.unwrap());

    Ok(())
}
