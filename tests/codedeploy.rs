#![cfg(feature = "codedeploy")]

extern crate rusoto;

use rusoto::codedeploy::{CodeDeployClient, ListApplicationsInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_applications() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CodeDeployClient::new(credentials, Region::UsEast1);

    let request = ListApplicationsInput::default();

    match client.list_applications(&request) {
    	Ok(response) => {
    		println!("{:#?}", response); 
    		assert!(true)
    	},
    	Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}