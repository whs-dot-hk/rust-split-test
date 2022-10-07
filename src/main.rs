fn main() {
    let test: Vec<&str> = "arn:aws:imagebuilder:us-east-1:102933037533:image/whslabs-cardano-node/1.0.0/2".split("/").collect();
    println!("{:?}", test);
    println!("https://us-east-1.console.aws.amazon.com/cloudwatch/home?region=us-east-1#logsV2:log-groups/log-group/$252Faws$252Fimagebuilder$252F{}/log-events/{}$252F{}", test[1], test[2], test[3]);
}
