/*
 * Created January 22, 2022
 *
 * Copyright (c) 2023 - JaneTech Inc.
 * MIT License
 */
 
#[cfg(test)]
mod test_resource {

    use rust_docker_serverless::models::resource::{Resource};
    use chrono::{Utc};

    fn mock_resource() -> Resource {
        Resource{id:String::from("Test1"), created_at:Some(Utc::now())}
    }

    #[test]
    fn test_resource_create() {
        let resource = mock_resource();
        let created = Resource::create(&resource);

        assert_eq!(resource, created);
    }

    #[test]
    fn test_resource_get() {
        let resource = mock_resource();
        Resource::create(&resource);
        
        assert_eq!(resource, Resource::get(&String::from("Test1")));
    }
}